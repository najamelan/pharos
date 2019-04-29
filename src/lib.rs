//! Simple implementation of an observable pattern. The idea is that clients can call `observe` on
//! your object to subscribe, which will return a futures 0.3 stream of Events. You call notify
//! everytime you want to notify observers.
//!

#![ feature( async_await, await_macro ) ]


mod observable;
mod pharos    ;



pub use
{
	pharos     :: { Pharos                          } ,
	observable :: { Observable, UnboundedObservable } ,
};



mod import
{
	pub use
	{
		std :: { sync::Arc } ,

		futures ::
		{
			future::{ self, join_all } ,
			Stream   , Sink            ,
			FutureExt, SinkExt         ,

			channel::mpsc::
			{
				self           , SendError         ,
				Sender         , Receiver          ,
				UnboundedSender, UnboundedReceiver ,
			} ,
		},
	};
}
