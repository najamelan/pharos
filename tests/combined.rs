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
		let mut isis = Godess::new();

		let mut events:          Receiver<IsisEvent> = isis.observe( 5, None )       ;
		let mut ubevts: UnboundedReceiver<IsisEvent> = isis.observe_unbounded( None) ;

		let mut nuevts:          Receiver<NutEvent>  = isis.observe( 5, None )       ;
		let mut ubnuts: UnboundedReceiver<NutEvent>  = isis.observe_unbounded( None) ;

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
