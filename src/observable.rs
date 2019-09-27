use crate :: { import::*, Filter, Events };

/// Indicate that a type is observable. You can call [`observe`](Observable::observe) to get a
/// stream of events.
///
/// Generally used with a [Pharos](crate::Pharos) object which manages the observers for you.
///
/// ```
/// use pharos::*;
/// use futures::stream::StreamExt;
///
/// // The event we want to broadcast
/// //
/// #[ derive( Debug, Clone ) ]
/// //
/// enum Steps
/// {
///   Step1     ,
///   Step2     ,
///   Done      ,
///
///   // Data is possible, but it has to be clone and will be cloned for each observer
///   // except observers that filter this event out.
///   //
///   Error(u8) ,
/// }
///
///
/// impl Steps
/// {
///    // We can use this as a predicate to filter events.
///    //
///    fn is_err( &self ) -> bool
///    {
///       match self
///       {
///          Self::Error(_) => true  ,
///          _              => false ,
///       }
///    }
/// }
///
///
/// // The object we want to be observable.
/// //
/// struct Foo { pharos: Pharos<Steps> };
///
/// impl Observable<Steps> for Foo
/// {
///    type Error = pharos::Error;
///
///    // Pharos implements observable, so we just forward the call.
///    //
///    fn observe( &mut self, options: ObserveConfig<Steps> ) -> Result< Events<Steps>, Self::Error >
///    {
///       self.pharos.observe( options )
///    }
/// }
///
///
/// // use in async context
/// //
/// async fn task()
/// {
///    let mut foo    = Foo { pharos: Pharos::default() };
///    let mut errors = foo.observe( Filter::Pointer( Steps::is_err ).into() ).expect( "observe" );
///
///    // will only be notified on errors thanks to the filter.
///    //
///    let next_error = errors.next().await;
/// }
/// ```
//
pub trait Observable<Event>

   where Event: Clone + 'static + Send ,
{
   /// The error type that is returned if observing is not possible. [Pharos](crate::Pharos) implements Sink
   /// which has a close method, so observing will no longer be possible after close is called.
   ///
   /// Other than that, you might want to have moments in your objects lifetime when you don't want to take
   /// any more observers. Returning a result from [observe](Observable::observe) enables that.
   ///
   /// You can of course map the error of pharos to your own error type.
   //
   type Error: ErrorTrait;

   /// Add an observer to the observable. Options can be in order to choose channel type and
   /// to filter events with a predicate.
   //
   fn observe( &mut self, options: ObserveConfig<Event> ) -> Result<Events<Event>, Self::Error>;
}



/// Choose the type of channel that will be used for your event stream. Used in [ObserveConfig].
//
#[ derive( Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord )]
//
pub enum Channel
{

   /// A channel with a limited message queue (the usize parameter). Creates back pressure when the buffer is full.
   /// This means that producer tasks may block if consumers can't process fast enough.
   ///
   /// ## Implementation
   ///
   /// Some background on the bounded channels from the futures library (that we use as a backend):
   /// - the `queue_size` is buffer + num senders (pharos is the only sender in our case). That means
   /// that you can set a buffer size of 0 and still send a message in, which is somewhat counter
   /// intuitive. It would make sense to do -1 on the user supplied queue_size to compensate, but:
   /// - `poll_flush` just calls poll_ready. That means that flush will return Pending if the buffer is full,
   /// even though in principle all messages are ready for the reader to read, so flush should be a
   /// noop for a channel. It also kind of kills an exact buffer size, because it will make `SinkExt::send`
   /// block when you send the last message that fits in the buffer. :-(
   ///
   /// Conclusion, we don't do the minus one. SinkExt::send works as expected, we don't need to validate
   /// against users sending in 0 queue_size, because it's now a valid input, since we take a usize, you
   /// can't send in negative numbers.
   ///
   //
   Bounded(usize),

   /// A channel with unbounded capacity. Note that this may lead to unbounded memory consumption if producers
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
/// This let's you choose the type of [channel](Channel) and let's
/// you set a [filter](Filter) to ignore certain events.
///
/// ```
/// use pharos::*;
///
/// // We choose event type usize for simplicity. You choose whatever type you want here,
/// // see the bounds on the Event type parameter throughout this library.
/// //
/// let mut pharos = Pharos::<usize>::default();
///
/// // Use defaults, unbounded channel and no filter.
/// //
/// pharos.observe( ObserveConfig::default() );
///
/// // Use bounded channel and defaults for other options.
/// //
/// pharos.observe( Channel::Bounded(5).into() );
///
/// // Use a filter and defaults for other options.
/// // Will only receive events if they are bigger than three.
/// //
/// pharos.observe( Filter::Pointer( |evt| *evt > 3 ).into() );
///
/// // Set both channel and filter. Note you can only set one filter per observable.
/// //
/// let opts = ObserveConfig::default()
///
///    .channel( Channel::Bounded( 5 ) )
///    .filter ( |evt| *evt > 3        )
/// ;
///
/// pharos.observe( opts );
/// ```
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
   /// You can only set one filter per observable.
   //
   pub fn filter( mut self, filter: fn(&Event) -> bool ) -> Self
   {
      debug_assert!( self.filter.is_none(), "You can only set one filter on ObserveConfig" );

      self.filter = Some( Filter::Pointer(filter) );
      self
   }


   /// Filter your event stream with a predicate that is a closure that captures environment.
   /// It is preferred to use [filter](ObserveConfig::filter) if you can as this will box the closure.
   /// You can only set one filter per observable.
   //
   pub fn filter_boxed( mut self, filter: impl FnMut(&Event) -> bool + Send + 'static ) -> Self
   {
      debug_assert!( self.filter.is_none(), "You can only set one filter on ObserveConfig" );

      self.filter = Some( Filter::Closure( Box::new(filter) ) );
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
