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
			stream        :: StreamExt, SinkExt,
		},
	};
}


use import::*;


pub struct Goddess
{
	isis: Pharos<IsisEvent>,
	nut : Pharos<NutEvent >,
}


impl Goddess
{
	pub fn new() -> Self
	{
		Self
		{
			isis: Pharos::default(),
			nut : Pharos::default(),
		}
	}

	pub async fn sail( &mut self )
	{
		self.isis.send( IsisEvent::Sail ).await.expect( "send event" );
	}

	pub async fn dock( &mut self )
	{
		self.isis.send( IsisEvent::Dock ).await.expect( "send event" );
	}

	pub async fn shine( &mut self )
	{
		let evt = NutEvent { time: "midnight".into() };

		self.nut.send( evt ).await.expect( "send event" );
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


impl Observable<IsisEvent> for Goddess
{
	type Error = pharos::Error;

	fn observe( &mut self, options: ObserveConfig<IsisEvent> ) -> Result< Events<IsisEvent>, Self::Error >
	{
		self.isis.observe( options )
	}
}


impl Observable<NutEvent> for Goddess
{
	type Error = pharos::Error;

	fn observe( &mut self, options: ObserveConfig<NutEvent> ) -> Result< Events<NutEvent>, Self::Error >
	{
		self.nut.observe( options )
	}
}

