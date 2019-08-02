#![ feature( async_await )]

// Tested:
//
// - ✔ basic functionality
// - ✔ test closing senders/receivers?
// - ✔ multiple observers + names coming back correctly
// - ✔ multiple observers + one drops, others continue to see messages
// - ✔ same names
// - ✔ send events of 2 types from one object + something other than an enum without data
// - ✔ accross threads
// - ✔ test big number of events


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

		let mut events: UnboundedReceiver<IsisEvent> = isis.observe_unbounded();

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

		let mut events: UnboundedReceiver<IsisEvent> = isis.observe_unbounded();

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

		let mut egypt_evts: UnboundedReceiver<IsisEvent> = isis.observe_unbounded();
		let mut shine_evts: UnboundedReceiver<IsisEvent> = isis.observe_unbounded();

		isis.sail().await;

		let shine_evt = shine_evts.next().await.unwrap();
		let egypt_evt = egypt_evts.next().await.unwrap();

		assert_eq!( IsisEvent::Sail, shine_evt );
		assert_eq!( IsisEvent::Sail , egypt_evt );

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

		let mut egypt_evts: UnboundedReceiver<IsisEvent> = isis.observe_unbounded();
		let mut shine_evts: UnboundedReceiver<IsisEvent> = isis.observe_unbounded();

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

		let mut egypt_evts: UnboundedReceiver<IsisEvent> = isis.observe_unbounded();
		let mut shine_evts: UnboundedReceiver<IsisEvent> = isis.observe_unbounded();

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

		let mut egypt_evts: UnboundedReceiver<IsisEvent> = isis.observe_unbounded();
		let mut shine_evts: UnboundedReceiver<NutEvent > = isis.observe_unbounded();

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

		let mut egypt_evts: UnboundedReceiver<IsisEvent> = isis.observe_unbounded();
		let mut shine_evts: UnboundedReceiver<NutEvent > = isis.observe_unbounded();


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




// Verify that the channel is really unbounded
//
#[ test ]
//
fn alot_of_events()
{
	run( async move
	{
		let mut w = Godess::new();

		let mut events: UnboundedReceiver<IsisEvent> = w.observe_unbounded();

		let amount = 1000;

		for _ in 0..amount
		{
			w.sail().await;
		}

		for _ in 0..amount
		{
			let evt = events.next().await.unwrap();

			assert_eq!( IsisEvent::Sail, evt );
		}
	});
}
