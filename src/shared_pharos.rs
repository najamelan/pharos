use crate::{ import::*, Pharos, PharErr, Observable, Observe, ObserveConfig, Events };


/// A handy wrapper that uses a futures aware mutex to allow using Pharos from a shared
/// reference.
//
#[ derive( Debug ) ]
//
pub struct SharedPharos<Event> where Event: 'static + Clone + Send
{
	pharos: Mutex< Pharos<Event> >,
}


impl<Event> SharedPharos<Event> where Event: 'static + Clone + Send
{
	/// Create a SharedPharos object.
	//
	pub fn new( pharos: Pharos<Event> ) -> Self
	{
		Self{ pharos: Mutex::new( pharos ) }
	}


	/// Notify observers.
	//
	pub async fn notify( &self, evt: Event ) -> Result<(), PharErr>
	{
		let mut ph = self.pharos.lock().await;

		ph.send( evt ).await
	}


	/// Start Observing this Pharos object.
	//
	pub async fn observe( &self, options: ObserveConfig<Event> ) -> Result<Events<Event>, <Self as Observable<Event>>::Error >
	{
		let mut ph = self.pharos.lock().await;

		ph.observe( options ).await
	}
}


impl<Event> Observable<Event> for SharedPharos<Event>

	where Event: 'static + Clone + Send
{
	type Error = PharErr;

	fn observe( &mut self, options: ObserveConfig<Event> ) -> Observe< '_, Event, Self::Error >
	{
		SharedPharos::observe( self, options ).boxed()
	}
}
