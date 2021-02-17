// Tested:
//
// ✔ Use a SharedPharos confronted by backpressure and using the lock.
// - A more involved test. Probably keep a global datastructure which records the order of operations
//   and then assert everything happens in the expected order. Eg. verifies that the back pressure is
//   actually doing something.
//
//
mod common;

use common::{ *, import::* };



#[ async_std::test ]
//
async fn basic()
{
	let mut shared = Shared::new();
	let mut events = shared.observe( Channel::Bounded( 1 ).into() ).await.expect( "observe" );

	shared.start();
	drop( shared );

	assert!(matches!( events.next().await.unwrap(), SharedEvent{..} ));
	assert!(matches!( events.next().await.unwrap(), SharedEvent{..} ));
	assert!(matches!( events.next().await.unwrap(), SharedEvent{..} ));
	assert!(matches!( events.next().await.unwrap(), SharedEvent{..} ));

	assert_eq!( None, events.next().await );
}
