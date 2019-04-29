use crate :: { import::* };

/// Indicate that a type is observable. You can call [`observe`](Observable::observe) to get a
/// stream of events.
//
pub trait Observable<Event>

	where Event: Clone + 'static + Send ,
{
	/// Add an observer to the observable. This will use a bounded channel of the size of `queue_size` + 1.
	/// Note that the use of a bounded channel provides backpressure and can slow down the observed
	/// task.
	//
	fn observe( &mut self, queue_size: usize ) -> Receiver<Event>;
}


/// Indicate that a type is observable through an unbounded stream. You can call [`observe_unbounded`](UnboundedObservable::observe_unbounded)
/// to get a stream of events.
//
pub trait UnboundedObservable<Event>

	where Event: Clone + 'static + Send ,
{
	/// Add an observer to the observable. This will use an unbounded channel. Beware that if the observable
	/// outpaces the observer, this will lead to growing memory consumption over time.
	//
	fn observe_unbounded( &mut self ) -> UnboundedReceiver<Event>;
}
