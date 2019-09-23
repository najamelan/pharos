use crate :: { import::*, Filter, ObserveConfig, observable::Channel, Error };

/// A stream of events. This is returned from [Observable::observe](crate::Observable::observe).
///
/// For pharos 0.3.0 on x64 linux: `std::mem::size_of::<Events<_>>() == 16`
//
#[ derive( Debug ) ]
//
pub struct Events<Event> where Event: Clone + 'static + Send
{
	rx: Receiver<Event>,
}


impl<Event> Events<Event> where Event: Clone + 'static + Send
{
	pub(crate) fn new( config: ObserveConfig<Event> ) -> (Self, Sender<Event>)
	{
		let (tx, rx) = match config.channel
		{
			Channel::Bounded( queue_size ) =>
			{
				let (tx, rx) = mpsc::channel( queue_size );

				( Sender::Bounded{ tx, filter: config.filter }, Receiver::Bounded{ rx } )
			}

			Channel::Unbounded =>
			{
				let (tx, rx) = mpsc::unbounded();

				( Sender::Unbounded{ tx, filter: config.filter }, Receiver::Unbounded{ rx } )
			}

			_ => unreachable!(),
		};


		( Self{ rx }, tx )
	}


	/// Close the channel. This way the sender will stop sending new events, and you can still
	/// continue to read any events that are still pending in the channel. This avoids data loss
	/// compared to just dropping this object.
	//
	pub fn close( &mut self )
	{
		self.rx.close();
	}
}




impl<Event> Stream for Events<Event> where Event: Clone + 'static + Send
{
	type Item = Event;

	fn poll_next( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll< Option<Self::Item> >
	{
		Pin::new( &mut self.rx ).poll_next( cx )
	}
}



/// The sender of the channel.
/// For pharos 0.3.0 on x64 linux: `std::mem::size_of::<Sender<_>>() == 56`
//
#[ pin_project ]
//
pub(crate) enum Sender<Event> where Event: Clone + 'static + Send
{
	Bounded  { #[pin] tx: FutSender<Event>         , filter: Option<Filter<Event>> } ,
	Unbounded{ #[pin] tx: FutUnboundedSender<Event>, filter: Option<Filter<Event>> } ,
}




impl<Event> Sender<Event>  where Event: Clone + 'static + Send
{
	// Verify whether this observer is still around
	//
	pub(crate) fn is_closed( &self ) -> bool
	{
		match self
		{
			Sender::Bounded  { tx, .. } => tx.is_closed(),
			Sender::Unbounded{ tx, .. } => tx.is_closed(),
		}
	}


	// Notify the observer and return a bool indicating whether this observer is still
	// operational. If an error happens on a channel it usually means that the channel
	// is closed, in which case we should drop this sender.
	//
	pub(crate) async fn notify( &mut self, evt: &Event ) -> bool
	{
		if self.is_closed() { return false }

		match self
		{
			Sender::Bounded  { tx, filter } => Self::notifier( tx, filter, evt ).await,
			Sender::Unbounded{ tx, filter } => Self::notifier( tx, filter, evt ).await,
		}
	}


	async fn notifier
	(
		mut tx: impl Sink<Event> + Unpin   ,
		filter: &mut Option<Filter<Event>> ,
		evt   : &Event                     ,
	)

		-> bool

	{
		let interested = match filter
		{
			Some(f) => f.call(evt),
			None    => true       ,
		};


		#[ allow( clippy::match_bool ) ]
		//
		match interested
		{
			true  => tx.send( evt.clone() ).await.is_ok(),

			// since we don't try to send, we know nothing about whether they are still
			// observing, so assume they do.
			//
			false => true,
		}
	}
}



/// The receiver of the channel.
//
#[ pin_project ]
//
enum Receiver<Event> where Event: Clone + 'static + Send
{
	Bounded  { #[pin] rx: FutReceiver<Event>          } ,
	Unbounded{ #[pin] rx: FutUnboundedReceiver<Event> } ,
}


impl<Event> Receiver<Event> where Event: Clone + 'static + Send
{
	fn close( &mut self )
	{
		match self
		{
			Receiver::Bounded  { rx } => rx.close(),
			Receiver::Unbounded{ rx } => rx.close(),
		};
	}
}



impl<Event> fmt::Debug for Receiver<Event>  where Event: 'static + Clone + Send
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		match self
		{
			Self::Bounded  {..} => write!( f, "pharos::events::Receiver::<{}>::Bounded(_)"  , type_name::<Event>() ),
			Self::Unbounded{..} => write!( f, "pharos::events::Receiver::<{}>::Unbounded(_)", type_name::<Event>() ),
		}
	}
}




impl<Event> Stream for Receiver<Event> where Event: Clone + 'static + Send
{
	type Item = Event;

	#[ project ]
	//
	fn poll_next( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll< Option<Self::Item> >
	{
		#[ project ]
		//
		match self.project()
		{
			Receiver::Bounded  { rx } => rx.poll_next( cx ),
			Receiver::Unbounded{ rx } => rx.poll_next( cx ),
		}
	}
}



impl<Event> Sink<Event> for Sender<Event> where Event: Clone + 'static + Send
{
	type Error = Error;

	#[ project ]
	//
	fn poll_ready( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		#[ project ]
		//
		match self.project()
		{
			Sender::Bounded  { tx, .. } => tx.poll_ready( cx ).map_err( Into::into ),
			Sender::Unbounded{ tx, .. } => tx.poll_ready( cx ).map_err( Into::into ),
		}
	}

	#[ project ]
	//
	fn start_send( self: Pin<&mut Self>, item: Event ) -> Result<(), Self::Error>
	{
		#[ project ]
		//
		match self.project()
		{
			Sender::Bounded  { tx, .. } => tx.start_send( item ).map_err( Into::into ),
			Sender::Unbounded{ tx, .. } => tx.start_send( item ).map_err( Into::into ),
		}
	}

	/// This will do a send under the hood, so the same errors as from start_send can occur here.
	//
	#[ project ]
	//
	fn poll_flush( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		#[ project ]
		//
		match self.project()
		{
			Sender::Bounded  { tx, .. } => tx.poll_flush( cx ).map_err( Into::into ),
			Sender::Unbounded{ tx, .. } => tx.poll_flush( cx ).map_err( Into::into ),
		}
	}

	#[ project ]
	//
	fn poll_close( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		#[ project ]
		//
		match self.project()
		{
			Sender::Bounded  { tx, .. } => tx.poll_close( cx ).map_err( Into::into ),
			Sender::Unbounded{ tx, .. } => tx.poll_close( cx ).map_err( Into::into ),
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
	fn debug()
	{
		let e = Events::<bool>::new( ObserveConfig::default() );

		assert_eq!( "Events { rx: pharos::events::Receiver::<bool>::Unbounded(_) }", &format!( "{:?}", e.0 ) );
	}
}
