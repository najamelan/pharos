#![ allow( dead_code ) ]

pub mod import
{
	#[ allow( unused_imports )]
	//
	pub(crate) use
	{
		pharos :: { *                 } ,
		std    :: { sync::Arc, thread } ,

		futures ::
		{
			channel::mpsc :: Receiver          ,
			channel::mpsc :: UnboundedReceiver ,
			executor      :: block_on          ,
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
		self.isis.notify( &IsisEvent::Sail ).await;
	}

	pub async fn dock( &mut self )
	{
		self.isis.notify( &IsisEvent::Dock ).await;
	}

	pub async fn shine( &mut self )
	{
		let evt = NutEvent { time: "midnight".into() };

		self.nut.notify( &evt ).await;
	}
}



#[ derive( Clone, Debug, PartialEq, Copy ) ]
//
pub enum IsisEvent
{
	Sail,
	Dock,
}



#[ derive( Clone, Debug, PartialEq ) ]
//
pub struct NutEvent
{
	pub time: Arc<str>
}


impl Observable<IsisEvent> for Godess
{
	fn observe( &mut self, options: ObserveConfig<IsisEvent> ) -> Events<IsisEvent>
	{
		self.isis.observe( options )
	}
}


impl Observable<NutEvent> for Godess
{
	fn observe( &mut self, options: ObserveConfig<NutEvent> ) -> Events<NutEvent>
	{
		self.nut.observe( options )
	}
}

