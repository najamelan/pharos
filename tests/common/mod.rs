#![ allow( dead_code ) ]

pub mod import
{
	#[ allow( unused_imports )]
	//
	pub(crate) use
	{
		pharos          :: { *                                                    } ,
		std             :: { sync::Arc, thread, task::{ Context, Poll }, pin::Pin } ,
		async_executors :: AsyncStd ,

		futures ::
		{
			channel::mpsc :: Receiver                   ,
			channel::mpsc :: UnboundedReceiver          ,
			executor      :: block_on                   ,
			task          :: { Spawn, SpawnExt }        ,
			stream        :: Stream, StreamExt, SinkExt ,
			sink          :: Sink                       ,
			future        :: poll_fn                    ,
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
	type Error = PharErr;

	fn observe( &mut self, options: ObserveConfig<IsisEvent> ) -> Observe< '_, IsisEvent, Self::Error >
	{
		self.isis.observe( options )
	}
}


impl Observable<NutEvent> for Goddess
{
	type Error = PharErr;

	fn observe( &mut self, options: ObserveConfig<NutEvent> ) -> Observe< '_, NutEvent, Self::Error >
	{
		self.nut.observe( options )
	}
}








pub struct Shared
{
	ph : SharedPharos<SharedEvent>,
}


impl Shared
{
	pub fn new() -> Self
	{
		Self
		{
			ph: SharedPharos::default(),
		}
	}


	pub fn start( &mut self, exec: impl Spawn )
	{
		for _ in 0..4
		{
			let ph = self.ph.clone();

			exec.spawn( async move
			{
				ph.notify( SharedEvent{ time: "test".into() } ).await.expect( "notify" );

			}).expect( "spawn" );
		}
	}
}



#[ derive( Clone, Debug, PartialEq ) ]
//
pub struct SharedEvent
{
	pub time: Arc<str>
}


impl Observable<SharedEvent> for Shared
{
	type Error = PharErr;

	fn observe( &mut self, options: ObserveConfig<SharedEvent> ) -> Observe< '_, SharedEvent, Self::Error >
	{
		self.ph.observe( options )
	}
}
