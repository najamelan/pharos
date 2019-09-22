use crate::{ import::* };


/// The error type for errors happening in `pharos`.
///
/// Use [`err.kind()`] to know which kind of error happened.
//
#[ derive( Debug ) ]
//
pub struct Error
{
	pub(crate) inner: Option< Box<dyn ErrorTrait + Send> >,
	pub(crate) kind : ErrorKind,
}



/// The different kind of errors that can happen when you use the `pharos` API.
//
#[ derive( Debug ) ]
//
pub enum ErrorKind
{
	/// Failed to send on channel, normally means it's closed.
	//
	SendError,

	#[ doc( hidden ) ]
	//
	__NonExhaustive__
}



impl ErrorTrait for Error
{
	fn source( &self ) -> Option< &(dyn ErrorTrait + 'static) >
	{
		self.inner.as_ref().map( |e| -> &(dyn ErrorTrait + 'static) { e.deref() } )
	}
}




impl fmt::Display for ErrorKind
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		match self
		{
			Self::SendError   => fmt::Display::fmt( "Channel closed.", f ) ,

			_ => unreachable!(),
		}
	}
}


impl fmt::Display for Error
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		let inner = match self.source()
		{
			Some(e) => format!( " Caused by: {}", e ),
			None    => String::new()                 ,
		};

		write!( f, "pharos::Error: {}{}", self.kind, inner )
	}
}



impl Error
{
	/// Allows matching on the error kind
	//
	pub fn kind( &self ) -> &ErrorKind
	{
		&self.kind
	}
}

impl From<ErrorKind> for Error
{
	fn from( kind: ErrorKind ) -> Error
	{
		Error { inner: None, kind }
	}
}

impl From<FutSendError> for Error
{
	fn from( inner: FutSendError ) -> Error
	{
		Error { inner: Some( Box::new( inner ) ), kind: ErrorKind::SendError }
	}
}
