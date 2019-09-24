// Tested:
//
// - ✔ basic functionality
// - ✔ test closing senders/receivers?
// - ✔ multiple observers + one drops, others continue to see messages
// - ✔ send events of 2 types from one object + something other than an enum without data
// - ✔ accross threads
// - ✔ test big number of events
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
		let mut isis = Goddess::new();

		let mut events = isis.observe( ObserveConfig::default() ).expect( "observe" );

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
		let mut isis = Goddess::new();

		let mut events = isis.observe( ObserveConfig::default() ).expect( "observe" );

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
		let mut isis = Goddess::new();

		let mut egypt_evts = isis.observe( ObserveConfig::default() ).expect( "observe" );
		let mut shine_evts = isis.observe( ObserveConfig::default() ).expect( "observe" );

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
		let mut isis = Goddess::new();

		let mut egypt_evts: Events<IsisEvent> = isis.observe( ObserveConfig::default() ).expect( "observe" );
		let mut shine_evts: Events<NutEvent > = isis.observe( ObserveConfig::default() ).expect( "observe" );

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
		let mut isis = Goddess::new();

		let mut egypt_evts = isis.observe( ObserveConfig::default() ).expect( "observe" );
		let mut shine_evts = isis.observe( ObserveConfig::default() ).expect( "observe" );


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
		let mut w = Goddess::new();

		let mut events = w.observe( ObserveConfig::default() ).expect( "observe" );

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

		let mut events = isis.observe( ObserveConfig::default().filter( filter ) ).expect( "observe" );

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

		let mut events = isis.observe( ObserveConfig::default().filter( filter ) ).expect( "observe" );

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

		let mut events = isis.observe( ObserveConfig::default().filter( filter ) ).expect( "observe" );

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
		let mut isis = Goddess::new();
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

		let mut events = isis.observe( ObserveConfig::default().filter_boxed( filter ) ).expect( "observe" );

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

