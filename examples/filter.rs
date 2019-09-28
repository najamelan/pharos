#![ allow( unused_variables, dead_code ) ]

use pharos::*;

#[ derive( Clone, Debug, PartialEq, Copy ) ]
//
enum NetworkEvent
{
	 Open    ,
	 Error   ,
	 Closing ,
	 Closed  ,
}

struct Connection { pharos: Pharos<NetworkEvent> }

impl Observable<NetworkEvent> for Connection
{
	type Error = pharos::Error;

	fn observe( &mut self, options: ObserveConfig<NetworkEvent>) -> Result< Events<NetworkEvent>, Self::Error >
	{
		 self.pharos.observe( options )
	}
}

fn main()
{
	let mut conn = Connection{ pharos: Pharos::default() };

	// We will only get close events.
	//
	let filter = Filter::Pointer( |e| e == &NetworkEvent::Closed );

	// By creating the config object through into, other options will be defaults, notably here
	// this will use unbounded channels.
	//
	let observer = conn.observe( filter.into() ).expect( "observe" );

	// Combine both options.
	//
	let filter = Filter::Pointer( |e| e != &NetworkEvent::Closed );
	let opts   = ObserveConfig::from( filter ).channel( Channel::Bounded(5) );

	// Get everything but close events over a bounded channel with queue size 5.
	//
	let bounded_observer = conn.observe( opts );
}
