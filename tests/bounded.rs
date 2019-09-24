// Tested:
//
// - ✔ basic functionality
// - ✔ test closing senders/receivers?
// - ✔ send events of 2 types from one object + something other than an enum without data
// - ✔ accross threads
// - TODO: test that sender task blocks when observer falls behind (see comments below)
// - ✔ Basic filter usage, only one event type should be returned.
// - ✔ A filter that always returns true should get all events.
// - ✔ A filter that always returns false should not get any events.
//
mod common;

use common::{ *, import::* };



#[ test ]
//
fn basic()
{
	block_on( async move
	{
		let mut isis   = Goddess::new();
		let mut events = isis.observe( Channel::Bounded( 5 ).into() ).expect( "observe" );

		isis.sail().await;
		isis.sail().await;
		drop( isis );

		assert_eq!( IsisEvent::Sail, events.next().await.unwrap() );
		assert_eq!( IsisEvent::Sail, events.next().await.unwrap() );
		assert_eq!( None           , events.next().await          );
	});
}



// Verify that if we close the receiver, the next notify will never arrive on the stream.
//
#[ test ]
//
fn close_receiver()
{
	block_on( async move
	{
		let mut isis   = Goddess::new();
		let mut events = isis.observe( Channel::Bounded( 5 ).into() ).expect( "observe" );

		isis.sail().await;
		events.close();
		isis.sail().await;

		assert_eq!( IsisEvent::Sail, events.next().await.unwrap() );
		assert_eq!( None           , events.next().await          );
	});
}


// Verify that if one receiver gets dropped, others can continue receiving
//
#[ test ]
//
fn one_receiver_drops()
{
	block_on( async move
	{
		let mut isis       = Goddess::new();
		let mut egypt_evts = isis.observe( Channel::Bounded( 1 ).into() ).expect( "observe" );
		let mut shine_evts = isis.observe( Channel::Bounded( 2 ).into() ).expect( "observe" );

		isis.sail().await;

		assert_eq!( IsisEvent::Sail, shine_evts.next().await.unwrap() );
		assert_eq!( IsisEvent::Sail, egypt_evts.next().await.unwrap() );

		drop( egypt_evts );

		isis.sail().await;
		isis.sail().await;

		assert_eq!( IsisEvent::Sail, shine_evts.next().await.unwrap() );
		assert_eq!( IsisEvent::Sail, shine_evts.next().await.unwrap() );
	});
}



// Send different types of events from same observable, and send a struct with data rather than just an enum
//
#[ test ]
//
fn types()
{
	block_on( async move
	{
		let mut isis = Goddess::new();

		// Note that because of the asserts below type inference works here and we don't have to
		// put type annotation, but I do find it quite obscure and better to be explicit.
		//
		let mut shine_evts: Events<NutEvent>  = isis.observe( Channel::Bounded( 5 ).into() ).expect( "observe" );
		let mut egypt_evts: Events<IsisEvent> = isis.observe( Channel::Bounded( 5 ).into() ).expect( "observe" );

		isis.shine().await;
		isis.sail ().await;

		let shine_evt = shine_evts.next().await.unwrap();
		let egypt_evt = egypt_evts.next().await.unwrap();

		assert_eq!( NutEvent{ time: "midnight".into() }, shine_evt );
		assert_eq!( IsisEvent::Sail                    , egypt_evt );
	});
}



// Send accross threads
//
#[ test ]
//
fn threads()
{
	block_on( async move
	{
		let mut isis       = Goddess::new();
		let mut egypt_evts = isis.observe( Channel::Bounded( 5 ).into() ).expect( "observe" );
		let mut shine_evts = isis.observe( Channel::Bounded( 5 ).into() ).expect( "observe" );


		thread::spawn( move ||
		{
			block_on( async move
			{
				isis.sail ().await;
				isis.shine().await;
			});
		});

		let shine_evt = shine_evts.next().await.unwrap();
		let egypt_evt = egypt_evts.next().await.unwrap();

		assert_eq!( NutEvent{ time: "midnight".into() }, shine_evt );
		assert_eq!( IsisEvent::Sail                    , egypt_evt );
	});
}



// TODO: this test doesn't work right now, because a spawned task swallows a panic...
//
// Other than that the bounded channels are proven to be really bounded.
//
// The reasoning is as follows. The channel if we pass queue_size 0, it actually has size 1 with
// 1 sender. So we fill that with an event, pull it out, but before the sender task can be woken up,
// we drop it. So now the sender task should still be blocked on the second send when it gets dropped.
// Thus, unreachable should never be called and the stream should return None. You can manually verify
// that this works by using `cargo test -- --nocapture` and verifying that the panic appears on screen
// when the second send is commented out, but the test will still be reported as a passed test.
//
/*#[ test ]
//
fn block_sender()
{
	let mut pool  = LocalPool::new();
	let mut exec  = pool.spawner();

	let mut isis = Goddess::new();
	let mut events: Receiver<IsisEvent> = isis.observe( 0 );

	let sender = async move
	{
		isis.sail().await;
		// isis.sail().await;

		unreachable!();
	};

	let (remote, handle) = sender.remote_handle();


	let receiver = async move
	{
		assert_eq!( IsisEvent::Sail, events.next().await.unwrap() );
		// drop(handle);
		assert_eq!( None , events.next().await );
	};

	exec.spawn_local( receiver ).expect( "Spawn receiver" );
	exec.spawn_local( remote ).unwrap();

	pool.block_on();
}
*/


// Basic filter usage, only Dock should be returned.
//
#[ test ]
//
fn filter()
{
	block_on( async move
	{
		let mut isis = Goddess::new();

		let filter = |evt: &IsisEvent|
		{
			match evt
			{
				IsisEvent::Sail => false,
				IsisEvent::Dock => true ,
			}
		};


		let opts = ObserveConfig::from( Channel::Bounded( 5 ) ).filter( filter );

		let mut events = isis.observe( opts ).expect( "observe" );

		isis.sail().await;
		isis.sail().await;
		isis.dock().await;
		isis.dock().await;
		isis.sail().await;

		drop( isis );

		assert_eq!( IsisEvent::Dock, events.next().await.unwrap() );
		assert_eq!( IsisEvent::Dock, events.next().await.unwrap() );
		assert_eq!( None           , events.next().await          );
	});
}



// A filter that always returns true should get all events.
//
#[ test ]
//
fn filter_true()
{
	block_on( async move
	{
		let mut isis = Goddess::new();

		let filter = |_: &IsisEvent| true;

		let opts = ObserveConfig::from( Channel::Bounded( 5 ) ).filter( filter );


		let mut events = isis.observe( opts ).expect( "observe" );

		isis.sail().await;
		isis.sail().await;
		isis.dock().await;
		isis.dock().await;
		isis.sail().await;

		drop( isis );

		assert_eq!( IsisEvent::Sail, events.next().await.unwrap() );
		assert_eq!( IsisEvent::Sail, events.next().await.unwrap() );
		assert_eq!( IsisEvent::Dock, events.next().await.unwrap() );
		assert_eq!( IsisEvent::Dock, events.next().await.unwrap() );
		assert_eq!( IsisEvent::Sail, events.next().await.unwrap() );
		assert_eq!( None           , events.next().await          );
	});
}



// A filter that always returns false should not get any events.
//
#[ test ]
//
fn filter_false()
{
	block_on( async move
	{
		let mut isis = Goddess::new();

		let filter = |_: &IsisEvent| false;

		let opts = ObserveConfig::from( Channel::Bounded( 5 ) ).filter( filter );


		let mut events = isis.observe( opts ).expect( "observe" );

		isis.sail().await;
		isis.sail().await;
		isis.dock().await;
		isis.dock().await;
		isis.sail().await;

		drop( isis );

		assert_eq!( None, events.next().await );
	});
}



// Make sure we can move something into the closure, only Dock should be returned.
//
#[ test ]
//
fn filter_move()
{
	block_on( async move
	{
		let mut isis   = Goddess::new();
		let v: Vec<u8> = Vec::new();

		let filter = move |evt: &IsisEvent|
		{
			match evt
			{
				IsisEvent::Sail if v.is_empty() => false,
				IsisEvent::Dock if v.is_empty() => true ,
				_                               => false,
			}
		};

		let opts = ObserveConfig::from( Channel::Bounded( 5 ) ).filter_boxed( filter );


		let mut events = isis.observe( opts ).expect( "observe" );

		isis.sail().await;
		isis.sail().await;
		isis.dock().await;
		isis.dock().await;
		isis.sail().await;

		drop( isis );

		assert_eq!( IsisEvent::Dock, events.next().await.unwrap() );
		assert_eq!( IsisEvent::Dock, events.next().await.unwrap() );
		assert_eq!( None           , events.next().await          );
	});
}


