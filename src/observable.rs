use crate :: { import::* };

/// Indicate that a type is observable. You can call [observe] to get a
/// stream of events.
//
pub trait Observable<Event>

	where Event: Clone + 'static + Send ,
{
	fn observe( &mut self, name: Arc<str>, queue_size: usize ) -> Receiver<(Arc<str>, Event)>;
}


/// Indicate that a type is observable through an unbounded stream. You can call [observe_unbounded]
/// to get a stream of events.
//
pub trait UnboundedObservable<Event>

	where Event: Clone + 'static + Send ,
{
	fn observe_unbounded( &mut self, name: Arc<str> ) -> UnboundedReceiver<(Arc<str>, Event)>;
}
