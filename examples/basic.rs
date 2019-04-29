#![ feature( async_await, await_macro )]

use
{
	pharos :: { *          } ,
	std    :: { sync::Arc  } ,

	futures ::
	{
		channel::mpsc :: Receiver      ,
		executor      :: LocalPool     ,
		task          :: LocalSpawnExt ,
		stream        :: StreamExt     ,
	},
};


struct Isis { pharos: Pharos<IsisEvent> }


impl Isis
{
	fn new() -> Self
	{
		Self { pharos: Pharos::new() }
	}

	pub async fn say_hello( &mut self )
	{
		await!( self.pharos.notify( IsisEvent::Hello ) );
	}
}



#[ derive( Clone, Debug ) ]
//
enum IsisEvent
{
	Hello
}


impl Observable<IsisEvent> for Isis
{
	fn observe( &mut self, name: Arc<str>, queue_size: usize ) -> Receiver<(Arc<str>, IsisEvent)>
	{
		self.pharos.observe( name, queue_size )
	}
}


fn main()
{
	let mut pool  = LocalPool::new();
	let mut exec  = pool.spawner();

	let program = async move
	{
		let mut w = Isis::new();

		let mut events = w.observe( "w".into(), 3 );

		await!( w.say_hello() );

		dbg!( await!( events.next() ) );
	};

	exec.spawn_local( program ).expect( "Spawn program" );

	pool.run();
}
