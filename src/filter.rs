use crate :: { import::* };

/// Filter events
//
pub enum Filter<Event>

	where Event: Clone + 'static + Send ,

{
	/// A function pointer to use to filter events
	//
	Pointer( fn(&Event) -> bool ),

	/// A boxed closure to use to filter events
	//
	Closure( Box<dyn FnMut(&Event) -> bool + Send> ),
}



impl<Event: Clone + 'static + Send> fmt::Debug for Filter<Event>
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		write!( f, "pharos::Filter" )
	}
}


impl<Event: Clone + 'static + Send> From<fn(&Event) -> bool> for Filter<Event>
{
	fn from( pointer: fn(&Event) -> bool ) -> Self
	{
		Filter::Pointer( pointer )
	}
}


impl<Event: Clone + 'static + Send> From<Box<dyn FnMut(&Event) -> bool + Send>> for Filter<Event>
{
	fn from( closure: Box<dyn FnMut(&Event) -> bool + Send> ) -> Self
	{
		Filter::Closure( closure )
	}
}
