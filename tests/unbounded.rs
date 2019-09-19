// Tested:
//
// - ✔ basic functionality
// - ✔ test closing senders/receivers?
// - ✔ multiple observers + one drops, others continue to see messages
// - ✔ send events of 2 types from one object + something other than an enum without data
// - ✔ accross threads
// - ✔ test big number of events


mod common;

use common::{ *, import::* };


#[ test ]
//
fn basic()
{
	block_on( async move
	{
		let mut isis = Godess::new();

		let mut events = isis.observe_unbounded();

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
		let mut isis = Godess::new();

		let mut events = isis.observe_unbounded();

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
		let mut isis = Godess::new();

		let mut egypt_evts = isis.observe_unbounded();
		let mut shine_evts = isis.observe_unbounded();

		isis.sail().await;

		let shine_evt = shine_evts.next().await.unwrap();
		let egypt_evt = egypt_evts.next().await.unwrap();

		assert_eq!( IsisEvent::Sail, shine_evt );
		assert_eq!( IsisEvent::Sail, egypt_evt );

		drop( egypt_evts );

		isis.sail().await;
		isis.sail().await;

		assert_eq!( IsisEvent::Sail, shine_evts.next().await.unwrap() );
		assert_eq!( IsisEvent::Sail, shine_evts.next().await.unwrap() );
	});
}


// Send different types of objects, and send a struct with data rather than just an enum
//
#[ test ]
//
fn types()
{
	block_on( async move
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
	block_on( async move
	{
		let mut isis = Godess::new();

		let mut egypt_evts = isis.observe_unbounded();
		let mut shine_evts = isis.observe_unbounded();


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




// Verify that the channel is really unbounded
//
#[ test ]
//
fn alot_of_events()
{
	block_on( async move
	{
		let mut w = Godess::new();

		let mut events = w.observe_unbounded();

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
