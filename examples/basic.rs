use
{
	pharos :: { * } ,

	futures ::
	{
		channel::mpsc :: Receiver      ,
		executor      :: LocalPool     ,
		task          :: LocalSpawnExt ,
		stream        :: StreamExt     ,
	},
};


// here we put a pharos object on our struct
//
struct Godess { pharos: Pharos<GodessEvent> }


impl Godess
{
	fn new() -> Self
	{
		Self { pharos: Pharos::new() }
	}

	// Send Godess sailing so she can tweet about it!
	//
	pub async fn sail( &mut self )
	{
		self.pharos.notify( &GodessEvent::Sailing ).await;
	}
}



// Event types need to implement clone, but you can wrap them in Arc if not. Also they will be
// cloned, so if you will have several observers and big event data, putting them in an Arc is
// definitely best. It has no benefit to put a simple dataless enum in an Arc though.
//
#[ derive( Clone, Debug, PartialEq, Copy ) ]
//
enum GodessEvent
{
	Sailing
}


// This is the needed implementation of Observable. We might one day have a derive for this,
// but it's not so interesting, since you always have to point it to your pharos object,
// and when you want to be observable over several types of events, you might want to keep
// pharos in a hashmap over type_id, and a derive would quickly become a mess.
//
impl Observable<GodessEvent> for Godess
{
	fn observe( &mut self, queue_size: usize, predicate: Option<Predicate<GodessEvent>> ) -> Receiver<GodessEvent>
	{
		self.pharos.observe( queue_size, predicate )
	}
}


fn main()
{
	let mut pool  = LocalPool::new();
	let mut exec  = pool.spawner();

	let program = async move
	{
		let mut isis = Godess::new();

		// subscribe
		//
		let mut events = isis.observe( 3, None );

		// trigger an event
		//
		isis.sail().await;

		// read from stream
		//
		let from_stream = events.next().await.unwrap();

		dbg!( from_stream );
		assert_eq!( GodessEvent::Sailing, from_stream );
	};

	exec.spawn_local( program ).expect( "Spawn program" );

	pool.run();
}
