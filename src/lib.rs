#![ cfg_attr( nightly, feature( external_doc, doc_cfg    ) ) ]
#![ cfg_attr( nightly, doc    ( include = "../README.md" ) ) ]
#![ doc = "" ] // empty doc line to handle missing doc warning when the feature is missing.

#![ doc    ( html_root_url = "https://docs.rs/pharos" ) ]
#![ deny   ( missing_docs                             ) ]
#![ forbid ( unsafe_code                              ) ]
#![ allow  ( clippy::suspicious_else_formatting       ) ]

#![ warn
(
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unused_extern_crates          ,
	unused_qualifications         ,
	single_use_lifetimes          ,
	unreachable_pub               ,
	variant_size_differences      ,
)]


mod error      ;
mod events     ;
mod observable ;
mod pharos     ;
mod filter     ;



pub use
{
	self::pharos :: { Pharos                             } ,
	filter       :: { Filter                             } ,
	observable   :: { Observable, ObserveConfig, Channel } ,
	events       :: { Events                             } ,
	error        :: { Error, ErrorKind                   } ,
};


mod import
{
	pub(crate) use
	{
		std            :: { fmt, error::Error as ErrorTrait, ops::Deref, any::type_name } ,
		std            :: { task::{ Poll, Context }, pin::Pin                           } ,
		futures        :: { Stream, Sink, ready                                         } ,

		futures::channel::mpsc::
		{
			self                                      ,
			Sender            as FutSender            ,
			Receiver          as FutReceiver          ,
			UnboundedSender   as FutUnboundedSender   ,
			UnboundedReceiver as FutUnboundedReceiver ,
			SendError         as FutSendError         ,
		},
	};

	#[ cfg( test ) ]
	//
	pub(crate) use
	{
		assert_matches :: { assert_matches                               } ,
		futures        :: { future::poll_fn, executor::block_on, SinkExt } ,
	};
}
