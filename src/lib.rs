//! Simple implementation of an observable pattern. The idea is that clients can call `observe` on
//! your object to subscribe, which will return a futures 0.3 stream of Events. You call notify
//! everytime you want to notify observers.
//!

#![ feature( await_macro, async_await ) ]

#![ cfg_attr( feature = "blanket", feature( specialization ) ) ]


mod named     ;
mod observable;

#[ cfg( feature = "blanket" ) ] mod observable_impl;


pub use
{
	named      :: Named      ,
	observable :: Observable ,
};


use std::{ pin::Pin, future::Future };

pub type ReturnNoSend<'a, R> = Pin<Box< dyn Future<Output = R> + 'a >> ;



mod import
{
	pub use
	{
		std     :: { sync::Arc                 } ,
		futures :: { prelude::{ Stream, Sink } } ,
		failure :: { Error                     } ,
	};


	#[ cfg( feature = "blanket" ) ]
	//
	pub use
	{
		hashbrown :: { HashMap                                               } ,
		futures   :: { SinkExt, FutureExt, channel::mpsc::{ self, UnboundedSender, UnboundedReceiver } } ,
	};
}
