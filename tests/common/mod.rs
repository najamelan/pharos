pub mod import
{
	pub use
	{
		pharos :: { *                                 } ,
		std    :: { sync::Arc, future::Future, thread } ,

		futures ::
		{
			future        :: FutureExt         ,
			channel::mpsc :: Receiver          ,
			channel::mpsc :: UnboundedReceiver ,
			executor      :: LocalPool         ,
			task          :: LocalSpawnExt     ,
			stream        :: StreamExt         ,
		},
	};
}


use import::*;


pub struct Godess
{
	isis: Pharos<IsisEvent>,
	nut : Pharos<NutEvent >,
}


impl Godess
{
	pub fn new() -> Self
	{
		Self
		{
			isis: Pharos::new(),
			nut : Pharos::new(),
		}
	}

	pub async fn sail( &mut self )
	{
		await!( self.isis.notify( &IsisEvent::Sail ) );
	}

	pub async fn shine( &mut self )
	{
		let evt = NutEvent { time: "midnight".into() };

		await!( self.nut.notify( &evt ) );
	}
}



#[ derive( Clone, Debug, PartialEq, Copy ) ]
//
pub enum IsisEvent
{
	Sail
}



#[ derive( Clone, Debug, PartialEq ) ]
//
pub struct NutEvent
{
	pub time: Arc<str>
}


impl Observable<IsisEvent> for Godess
{
	fn observe( &mut self, queue_size: usize ) -> Receiver<IsisEvent>
	{
		self.isis.observe( queue_size )
	}
}


impl Observable<NutEvent> for Godess
{
	fn observe( &mut self, queue_size: usize ) -> Receiver<NutEvent>
	{
		self.nut.observe( queue_size )
	}
}


impl UnboundedObservable<IsisEvent> for Godess
{
	fn observe_unbounded( &mut self ) -> UnboundedReceiver<IsisEvent>
	{
		self.isis.observe_unbounded()
	}
}


impl UnboundedObservable<NutEvent> for Godess
{
	fn observe_unbounded( &mut self ) -> UnboundedReceiver<NutEvent>
	{
		self.nut.observe_unbounded()
	}
}
