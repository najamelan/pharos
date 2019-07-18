#![ feature( async_await, await_macro )]

// Tested:
//
// - ✔ basic functionality
// - ✔ test closing senders/receivers?
// - ✔ multiple observers + names coming back correctly
// - ✔ same names
// - ✔ send events of 2 types from one object + something other than an enum without data
// - ✔ accross threads
// - TODO: test that sender task blocks when observer falls behind (see comments below)


mod common;

use common::{ *, import::* };


fn run( task: impl Future<Output=()> + 'static )
{
	let mut pool  = LocalPool::new();
	let mut exec  = pool.spawner();

	exec.spawn_local( task ).expect( "Spawn task" );
	pool.run();
}


#[ test ]
//
fn basic()
{
	run( async move
	{
		let mut isis = Godess::new();

		let mut events: Receiver<IsisEvent> = isis.observe( 5 );

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
	run( async move
	{
		let mut isis = Godess::new();

		let mut events: Receiver<IsisEvent> = isis.observe( 5 );

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
	run( async move
	{
		let mut isis = Godess::new();

		let mut egypt_evts: Receiver<IsisEvent> = isis.observe( 1 );
		let mut shine_evts: Receiver<IsisEvent> = isis.observe( 2 );

		isis.sail().await;

		let shine_evt = shine_evts.next().await.unwrap();
		let egypt_evt = egypt_evts.next().await.unwrap();

		assert_eq!( IsisEvent::Sail, shine_evt );
		assert_eq!( IsisEvent::Sail, egypt_evt );

		drop( egypt_evts );

		isis.sail().await;
		isis.sail().await;

		let shine_evt = shine_evts.next().await.unwrap();
		assert_eq!( IsisEvent::Sail, shine_evt );

		let shine_evt = shine_evts.next().await.unwrap();
		assert_eq!( IsisEvent::Sail, shine_evt );
	});
}



// Have two receivers with different names on the same object and verify that the names are correct on reception.
//
#[ test ]
//
fn names()
{
	run( async move
	{
		let mut isis = Godess::new();

		let mut egypt_evts: Receiver<IsisEvent> = isis.observe( 5 );
		let mut shine_evts: Receiver<IsisEvent> = isis.observe( 5 );

		isis.sail().await;

		let shine_evt = shine_evts.next().await.unwrap();
		let egypt_evt = egypt_evts.next().await.unwrap();

		assert_eq!( IsisEvent::Sail, shine_evt );
		assert_eq!( IsisEvent::Sail, egypt_evt );
	});
}



// Verify that several observers can set the same name.
//
#[ test ]
//
fn same_names()
{
	run( async move
	{
		let mut isis = Godess::new();

		let mut egypt_evts: Receiver<IsisEvent> = isis.observe( 5 );
		let mut shine_evts: Receiver<IsisEvent> = isis.observe( 5 );

		isis.sail().await;

		let shine_evt = shine_evts.next().await.unwrap();
		let egypt_evt = egypt_evts.next().await.unwrap();

		assert_eq!( IsisEvent::Sail, shine_evt );
		assert_eq!( IsisEvent::Sail, egypt_evt );
	});
}



// Send different types of objects, and send a struct with data rather than just an enum
//
#[ test ]
//
fn types()
{
	run( async move
	{
		let mut isis = Godess::new();

		let mut egypt_evts: Receiver<IsisEvent> = isis.observe( 5 );
		let mut shine_evts: Receiver<NutEvent > = isis.observe( 5 );

		isis.sail ().await;
		isis.shine().await;

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
	run( async move
	{
		let mut isis = Godess::new();

		let mut egypt_evts: Receiver<IsisEvent> = isis.observe( 5 );
		let mut shine_evts: Receiver<NutEvent > = isis.observe( 5 );


		thread::spawn( move ||
		{
			run( async move
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

	let mut isis = Godess::new();
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

	pool.run();
}
*/
