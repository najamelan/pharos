use
{
	pharos  :: { *                  } ,
	futures :: { StreamExt, SinkExt } ,
};


// here we put a pharos object on our struct
//
struct Goddess { pharos: Pharos<GoddessEvent> }


impl Goddess
{
	fn new() -> Self
	{
		Self { pharos: Pharos::default() }
	}

	// Send Goddess sailing so she can tweet about it!
	//
	pub async fn sail( &mut self )
	{
		// It's infallible. Observers that error will be dropped, since the only kind of errors on
		// channels are when the channel is closed.
		//
		let _ = self.pharos.send( GoddessEvent::Sailing ).await;
	}
}


// Event types need to implement clone, but you can wrap them in Arc if not. Also they will be
// cloned, so if you will have several observers and big event data, putting them in an Arc is
// definitely best. It has no benefit to put a simple dataless enum in an Arc though.
//
#[ derive( Clone, Debug, PartialEq, Copy ) ]
//
enum GoddessEvent
{
	Sailing
}


// This is the needed implementation of Observable. We might one day have a derive for this,
// but it's not so interesting, since you always have to point it to your pharos object,
// and when you want to be observable over several types of events, you might want to keep
// pharos in a hashmap over type_id, and a derive would quickly become a mess.
//
impl Observable<GoddessEvent> for Goddess
{
	type Error = PharErr;

	fn observe( &mut self, options: ObserveConfig<GoddessEvent>) -> Observe<'_, GoddessEvent, Self::Error >
	{
		self.pharos.observe( options )
	}
}


#[async_std::main]
//
async fn main()
{
	let mut isis = Goddess::new();

	// subscribe, the observe method takes options to let you choose:
	// - channel type (bounded/unbounded)
	// - a predicate to filter events
	//
	let mut events = isis.observe( Channel::Bounded( 3 ).into() ).await.expect( "observe" );

	// trigger an event
	//
	isis.sail().await;

	// read from stream and let's put on the console what the event looks like.
	//
	let evt = dbg!( events.next().await.unwrap() );

	// After this reads on the event stream will return None.
	//
	drop( isis );

	assert_eq!( GoddessEvent::Sailing, evt );
	assert_eq!( None, events.next().await );
}
