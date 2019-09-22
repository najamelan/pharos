use crate :: { import::*, Observable, UnboundedObservable, Filter };


/// The Pharos lighthouse. When you implement Observable on your type, you can forward
/// the [`observe`](Pharos::observe) method to Pharos and call notify on it.
///
/// You can of course create several `Pharos` (I know, historical sacrilege) for (different) types
/// of events.
//
pub struct Pharos<Event: Clone + 'static + Send>
{
	observers: Vec<Option< (Sender         <Event>, Option<Filter<Event>>) >>,
	unbounded: Vec<Option< (UnboundedSender<Event>, Option<Filter<Event>>) >>,
}



// TODO: figure out what we really want here...
//
impl<Event: Clone + 'static + Send> fmt::Debug for Pharos<Event>
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		write!( f, "Pharos" )
	}
}



impl<Event: Clone + 'static + Send> Pharos<Event>
{
	/// Create a new Pharos. May it's light guide you to safe harbour.
	//
	pub fn new() -> Self
	{
		Self::default()
	}



	/// Notify all observers of Event evt.
	//
	pub async fn notify<'a>( &'a mut self, evt: &'a Event )
	{
		let unbound = Self::notify_inner( &mut self.unbounded, &evt );
		let bounded = Self::notify_inner( &mut self.observers, &evt );

		join!( unbound, bounded );
	}



	// Helper method to abstract out over bounded and unbounded observers.
	//
	async fn notify_inner<'a>
	(
		observers: &'a mut Vec< Option< (impl Sink<Event> + Unpin + Clone, Option<Filter<Event>>) > > ,
		evt: &'a Event
	)
	{
		// Try to send to all channels in parallel, so they can all start processing this event
		// even if one of them is blocked on a full queue.
		//
		// We can not have mutable access in parallel, so we destructure our vector. This probably
		// allocates a new vector every time. If you have a better idea, please open an issue!
		//
		// The output of the join is a vec of options with the disconnected observers removed.
		//
		let fut = join_all
		(
			( 0..observers.len() ).map( |i|
			{
				let evt = evt.clone();
				let opt = observers[i].take();

				async move
				{
					if let Some( (mut tx, filter_opt) ) = opt
					{
						// If we have a filter, run it, otherwise return true.
						// we return the filter, since we need to give it back at the end.
						//
						let (go, pre_opt2) = filter_opt.map_or( (true, None), |mut filter|
						{
							let filtered = match filter
							{
								Filter::Pointer(ref     p) => p( &evt ),
								Filter::Closure(ref mut p) => p( &evt ),
							};

							(filtered, Some(filter))
						});

						// We count on the send not being executed if go is false.
						// If an error is returned, it's disconnected, drop it.
						//
						if go && tx.send( evt ).await.is_err()
						{
							None
						}

						// Put it back after use
						//
						else { Some( (tx, pre_opt2) ) }
					}

					// It was already none
					//
					else { None }
				}
			})
		);


		// Put back the observers that we "borrowed"
		// TODO: compact the vector from time to time?
		//
		*observers = fut.await;
	}
}



impl<Event: Clone + 'static + Send> Default for Pharos<Event>
{
	fn default() -> Self
	{
		Self
		{
			observers: Vec::new(),
			unbounded: Vec::new(),
		}
	}
}


impl<Event: 'static + Clone + Send> Observable<Event> for Pharos<Event>
{
	/// Add an observer to the pharos. This will use a bounded channel of the size of `queue_size`.
	/// Note that the use of a bounded channel provides backpressure and can slow down the observed
	/// task.
	//
	fn observe( &mut self, queue_size: usize, predicate: Option< Filter<Event> > ) -> Receiver<Event>
	{
		let (tx, rx) = mpsc::channel( queue_size );

		self.observers.push( Some(( tx, predicate )) );

		rx
	}
}


impl<Event: 'static + Clone + Send> UnboundedObservable<Event> for Pharos<Event>
{
	/// Add an observer to the pharos. This will use an unbounded channel. Beware that if the observable
	/// outpaces the observer, this will lead to growing memory consumption over time.
	//
	fn observe_unbounded( &mut self, predicate: Option< Filter<Event> > ) -> UnboundedReceiver<Event>
	{
		let (tx, rx) = mpsc::unbounded();

		self.unbounded.push( Some(( tx, predicate )) );

		rx
	}
}
