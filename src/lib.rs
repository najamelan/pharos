// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![ cfg_attr( feature = "external_doc", feature(external_doc)         ) ]
#![ cfg_attr( feature = "external_doc", doc(include = "../README.md") ) ]
//!


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


mod observable;
mod pharos    ;



pub use
{
	self::pharos :: { Pharos                          } ,
	observable   :: { Observable, UnboundedObservable } ,
};


/// The type of predicates used to filter events.
//
pub type Predicate<Event> = Box<dyn Fn(&Event) -> bool + Send >;


mod import
{
	pub(crate) use
	{
		std:: { fmt },

		futures ::
		{
			join,

			future::{ join_all }, Sink, SinkExt,

			channel::mpsc::
			{
				self         ,
				Sender         , Receiver          ,
				UnboundedSender, UnboundedReceiver ,
			} ,
		},
	};
}
