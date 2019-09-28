// Tested:
//
// - ✔ an object which has both bounded and unbounded observers.
//
mod common;

use common::{ *, import::* };


#[ test ]
//
fn both()
{
	block_on( async move
	{
		let mut isis = Goddess::new();

		let mut events: Events<IsisEvent> = isis.observe( Channel::Bounded( 5 ).into() ).expect( "observe" );
		let mut nuevts: Events<NutEvent>  = isis.observe( Channel::Bounded( 5 ).into() ).expect( "observe" );

		let mut ubevts: Events<IsisEvent> = isis.observe( ObserveConfig::default()     ).expect( "observe" );
		let mut ubnuts: Events<NutEvent>  = isis.observe( ObserveConfig::default()     ).expect( "observe" );

		isis.sail ().await;
		isis.shine().await;
		isis.sail ().await;
		isis.shine().await;

		drop( isis );

		assert_eq!( IsisEvent::Sail, events.next().await.unwrap() );
		assert_eq!( IsisEvent::Sail, events.next().await.unwrap() );
		assert_eq!( None           , events.next().await          );

		assert_eq!( IsisEvent::Sail, ubevts.next().await.unwrap() );
		assert_eq!( IsisEvent::Sail, ubevts.next().await.unwrap() );
		assert_eq!( None           , ubevts.next().await          );

		let nut_event = NutEvent { time: "midnight".into() };

		assert_eq!( nut_event, nuevts.next().await.unwrap() );
		assert_eq!( nut_event, nuevts.next().await.unwrap() );
		assert_eq!( None     , nuevts.next().await          );

		assert_eq!( nut_event, ubnuts.next().await.unwrap() );
		assert_eq!( nut_event, ubnuts.next().await.unwrap() );
		assert_eq!( None     , ubnuts.next().await          );
	});
}
