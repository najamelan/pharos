use crate :: { import::* };

/// Predicate for filtering events. This is an enum because closures that capture variables from
/// their environment need to be boxed. More often than not, an event will be a simple enum and
/// the predicate will just match on the variant, so it would be wasteful to impose boxing in those
/// cases, hence there is a function pointer variant which does not require boxing. This should
/// be preferred where possible.
//
pub enum Filter<Event>

	where Event: Clone + 'static + Send ,

{
	/// A function pointer to use to filter events.
	//
	Pointer( fn(&Event) -> bool ),

	/// A boxed closure to use to filter events.
	//
	Closure( Box<dyn FnMut(&Event) -> bool + Send> ),
}


impl<Event> Filter<Event>  where Event: Clone + 'static + Send
{
	/// Construct a filter from a closure that captures something from it's environment. This will
	/// be boxed and stored under the [Filter::Closure] variant. To avoid boxing, do not capture
	/// any variables from the environment and use [Filter::from_pointer].
	//
	pub fn from_closure<F>( predicate: F ) -> Self where  F: FnMut(&Event) -> bool + Send + 'static
	{
		Self::Closure( Box::new( predicate ) )
	}


	/// Construct a filter from a function pointer to a predicate.
	//
	pub fn from_pointer( predicate: fn(&Event) -> bool ) -> Self
	{
		Self::Pointer( predicate )
	}


	/// Invoke the predicate
	//
	pub(crate) fn call( &mut self, evt: &Event ) -> bool
	{
		match self
		{
			Self::Pointer(f) => f(evt),
			Self::Closure(f) => f(evt),
		}
	}
}


impl<Event> fmt::Debug for Filter<Event>  where Event: Clone + 'static + Send
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		match self
		{
			Self::Pointer(_) => write!( f, "pharos::Filter<{}>::Pointer(_)", type_name::<Event>() ),
			Self::Closure(_) => write!( f, "pharos::Filter<{}>::Closure(_)", type_name::<Event>() ),
		}

	}
}



#[ cfg( test ) ]
//
mod tests
{
	use super::*;

	#[test]
	//
	fn from_pointer()
	{
		let f = Filter::from_pointer( |b| *b );

		assert_matches!( f, Filter::Pointer(_) );
	}

	#[test]
	//
	fn from_closure()
	{
		let f = Filter::from_closure( |b| *b );

		assert_matches!( f, Filter::Closure(_) );
	}

	#[test]
	//
	fn debug()
	{
		let f = Filter::from_pointer( |b| *b );
		let g = Filter::from_closure( |b| *b );

		assert_eq!( "pharos::Filter<bool>::Pointer(_)", &format!( "{:?}", f ) );
		assert_eq!( "pharos::Filter<bool>::Closure(_)", &format!( "{:?}", g ) );
	}
}
