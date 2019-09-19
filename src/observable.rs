use crate :: { import::*, Predicate };

/// Indicate that a type is observable. You can call [`observe`](Observable::observe) to get a
/// stream of events.
//
pub trait Observable<Event>

	where Event: Clone + 'static + Send ,
{
	/// Add an observer to the observable. This will use a bounded channel of the size of `queue_size`.
	/// Note that the use of a bounded channel provides backpressure and can slow down the observed
	/// task.
	///
	/// The predicate parameter allows filtering the events that should be send to this observer.
	/// It receives a reference to the event. If the predicate returns true, it will be sent.
	//
	fn observe( &mut self, queue_size: usize, predicate: Option< Predicate<Event> > ) -> Receiver<Event>;
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
	fn observe_unbounded( &mut self, predicate: Option<Predicate<Event>> ) -> UnboundedReceiver<Event>;
}
