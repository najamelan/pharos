use crate :: { Filter, Events };

/// Indicate that a type is observable. You can call [`observe`](Observable::observe) to get a
/// stream of events.
//
pub trait Observable<Event>

	where Event: Clone + 'static + Send ,
{
	/// Add an observer to the observable. Options can be in order to choose channel type and
	/// to filter events with a predicate.
	//
	fn observe( &mut self, options: ObserveConfig<Event> ) -> Events<Event>;
}



/// Choose the type of channel that will be used for your event stream.
//
#[ derive( Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord )]
//
pub enum Channel
{
	/// A channel with a limited buffer (the usize parameter). Creates back pressure when the buffer is full.
	/// This means that producer tasks may block if consumers can't process fast enough.
	//
	Bounded(usize),

	/// A channel with unbounded capacity. Note that this may lead to unbouded memory consumption if producers
	/// outpace consumers.
	//
	Unbounded,

	/// This enum might grow in the future, thanks to this that won't be a breaking change.
	//
	__NonExhaustive__
}


impl Default for Channel
{
	fn default() -> Self
	{
		Channel::Unbounded
	}
}



/// Configuration for your event stream, passed to [Observable::observe] when subscribing.
/// This let's you choose the type of channel (currently Bounded or Unbounded) and let's
/// you set a filter to ignore certain events (see: [Filter]).
//
#[ derive( Debug ) ]
//
pub struct ObserveConfig<Event> where Event: Clone + 'static + Send
{
	pub(crate) channel: Channel,
	pub(crate) filter : Option<Filter<Event>>,
}



/// Create a default configuration:
/// - no filter
/// - an unbounded channel
//
impl<Event> Default for ObserveConfig<Event> where Event: Clone + 'static + Send
{
	fn default() -> Self
	{
		Self
		{
			channel: Channel::default(),
			filter : None              ,
		}
	}
}



impl<Event> ObserveConfig<Event> where Event: Clone + 'static + Send
{
	/// Choose which channel implementation to use for your event stream.
	//
	pub fn channel( mut self, channel: Channel ) -> Self
	{
		self.channel = channel;
		self
	}


	/// Filter your event stream with a predicate that is a fn pointer.
	//
	pub fn filter( mut self, filter: fn(&Event) -> bool ) -> Self
	{
		debug_assert!( self.filter.is_none(), "You can only set one filter on ObserveConfig" );

		self.filter = Some( Filter::Pointer(filter) );
		self
	}


	/// Filter your event stream with a predicate that is a closure that captures environment.
	/// It is preferred to use [filter](ObserveConfig::filter) if you can as this will box the closure.
	//
	pub fn filter_boxed( mut self, filter: impl FnMut(&Event) -> bool + Send + 'static ) -> Self
	{
		debug_assert!( self.filter.is_none(), "You can only set one filter on ObserveConfig" );

		self.filter = Some( Filter::from_closure(filter) );
		self
	}
}


/// Create a ObserveConfig from a [Channel], getting default values for other options.
//
impl<Event> From<Channel> for ObserveConfig<Event> where Event: Clone + 'static + Send
{
	fn from( channel: Channel ) -> Self
	{
		Self::default().channel( channel )
	}
}


/// Create a ObserveConfig from a [Filter], getting default values for other options.
//
impl<Event> From<Filter<Event>> for ObserveConfig<Event> where Event: Clone + 'static + Send
{
	fn from( filter: Filter<Event> ) -> Self
	{
		let mut s = Self::default();
		s.filter = Some( filter );
		s
	}
}
