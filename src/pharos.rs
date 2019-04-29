use crate :: { import::* };


/// The Pharos lighthouse. When you implement Observable on your type, you can forward
/// the [observe] method to Pharos and call notify on it.
///
/// You can of course create several Pharos (I know, historical sacrilege) for (different) types
/// of events.
//
#[ derive( Clone, Debug ) ]
//
pub struct Pharos<Event: Clone + 'static + Send>
{
	observers: Vec< Option<( Arc<str>, Sender         <(Arc<str>, Event)> )> >,
	unbounded: Vec< Option<( Arc<str>, UnboundedSender<(Arc<str>, Event)> )> >,
}


impl<Event: Clone + 'static + Send> Pharos<Event>
{
	/// Create a new Pharos. May it's light guide you to safe harbour.
	//
	pub fn new() -> Self
	{
		Self
		{
			observers: Vec::new(),
			unbounded: Vec::new(),
		}
	}



	/// Add an observer to the pharos. This will use a bounded channel of the size of `queue_size` + 1.
	/// Note that the use of a bounded channel provides backpressure and can slow down the observed
	/// task.
	//
	pub fn observe( &mut self, name: Arc<str>, queue_size: usize ) -> Receiver<(Arc<str>, Event)>
	{
		let (tx, rx) = mpsc::channel( queue_size );

		self.observers.push( Some((name, tx)) );

		rx
	}



	/// Add an observer to the pharos. This will use an unbounded channel. Beware that if the observable
	/// outpaces the observer, this will lead to growing memory consumption over time.
	//
	pub fn observe_unbound( &mut self, name: Arc<str> ) -> UnboundedReceiver<(Arc<str>, Event)>
	{
		let (tx, rx) = mpsc::unbounded();

		self.unbounded.push( Some((name, tx)) );

		rx
	}



	/// Notify all observers of Event evt.
	//
	pub async fn notify( &mut self, evt: Event )
	{
		await!( Self::notify_inner( &mut self.observers, evt.clone() ) );
		await!( Self::notify_inner( &mut self.unbounded, evt         ) );
	}



	// Helper method to abstract out over bounded and unbounded observers.
	//
	async fn notify_inner
	(
		observers: &mut Vec< Option<( Arc<str>, impl Sink<(Arc<str>, Event), SinkError=SendError> + Unpin + Clone )> > ,
		evt: Event
	)
	{
		let imm = &*observers;

		// Try to send to all unbounded in parallel, so they can all start processing this event
		// even if one of them is blocked on a full queue
		//
		let fut = join_all
		(
			( 0..imm.len() ).map( |i|
			{
				let evt = evt.clone();

				async move
				{
					let result = if let Some(( name, tx )) = &imm[ i ]
					{
						let mut tx = tx.clone();

						await!( tx.send(( name.clone(), evt )) )
					}

					else { Ok(()) };

					result.is_err()
				}
			})
		);


		// if any receivers where dropped, set them to none so we don't try to send to them again.
		//
		for (i, err) in await!( fut ).iter().enumerate()
		{
			if *err { observers[ i ] = None; }
		}
	}
}
