// Tested:
//
// ✔ Use a SharedPharos confronted by backpressure and using the lock.
// - A more involved test. Probably keep a global datastructure which records the order of operations
//   and then assert everything happens in the expected order. Eg. verifies that the back pressure is
//   actually doing something.
//
//
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

mod common;
use common::{ *, import::* };


#[ wasm_bindgen_test ]
//
async fn basic_wasm()
{
	let mut shared = Shared::new();
	let mut events = shared.observe( Channel::Bounded( 1 ).into() ).await.expect( "observe" );

	shared.start( AsyncStd );
	drop( shared );

	assert!(matches!( events.next().await.unwrap(), SharedEvent{..} ));
	assert!(matches!( events.next().await.unwrap(), SharedEvent{..} ));
	assert!(matches!( events.next().await.unwrap(), SharedEvent{..} ));
	assert!(matches!( events.next().await.unwrap(), SharedEvent{..} ));

	assert_eq!( None, events.next().await );
}
