use crate :: { import::* };


/// The Pharos lighthouse. When you implement Observable on your type, you can forward
/// the [`observe`](Pharos::observe) method to Pharos and call notify on it.
///
/// You can of course create several `Pharos` (I know, historical sacrilege) for (different) types
/// of events.
///
// TODO: why do we require Send?
//
#[ derive( Clone, Debug ) ]
//
pub struct Pharos<Event: Clone + 'static + Send>
{
	observers: Vec<Option< Sender         <Event> >>,
	unbounded: Vec<Option< UnboundedSender<Event> >>,
}


impl<Event: Clone + 'static + Send> Pharos<Event>
{
	/// Create a new Pharos. May it's light guide you to safe harbour.
	//
	pub fn new() -> Self
	{
		Self::default()
	}



	/// Add an observer to the pharos. This will use a bounded channel of the size of `queue_size`.
	/// Note that the use of a bounded channel provides backpressure and can slow down the observed
	/// task.
	//
	pub fn observe( &mut self, queue_size: usize ) -> Receiver<Event>
	{
		let (tx, rx) = mpsc::channel( queue_size );

		self.observers.push( Some( tx ) );

		rx
	}



	/// Add an observer to the pharos. This will use an unbounded channel. Beware that if the observable
	/// outpaces the observer, this will lead to growing memory consumption over time.
	//
	pub fn observe_unbounded( &mut self ) -> UnboundedReceiver<Event>
	{
		let (tx, rx) = mpsc::unbounded();

		self.unbounded.push( Some( tx ) );

		rx
	}



	/// Notify all observers of Event evt.
	//
	pub async fn notify<'a>( &'a mut self, evt: &'a Event )
	{
		Self::notify_inner( &mut self.unbounded, &evt ).await;
		Self::notify_inner( &mut self.observers, &evt ).await;
	}



	// Helper method to abstract out over bounded and unbounded observers.
	//
	async fn notify_inner<'a>
	(
		observers: &'a mut Vec< Option<impl Sink<Event> + Unpin + Clone> > ,
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
					if let Some( mut tx ) = opt
					{
						// It's disconnected, drop it
						//
						if tx.send( evt ).await.is_err()
						{
							None
						}

						// Put it back after use
						//
						else { Some( tx ) }
					}

					// It was already none
					//
					else { None }
				}
			})
		);


		// Put back the observers that we "borrowed"
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
