use crate :: { import::*, Observable, Events, ObserveConfig, events::Sender, Error, ErrorKind };


/// The Pharos lighthouse. When you implement Observable on your type, you can forward
/// the [`observe`](Observable::observe) method to Pharos and call notify on it.
///
/// You can of course create several `Pharos` (I know, historical sacrilege) for (different) types
/// of events.
///
/// Please see the docs for [Observable] for an example. Others can be found in the README and
/// the [examples](https://github.com/najamelan/pharos/tree/master/examples) directory of the repository.
///
/// ## Implementation.
///
/// Currently just holds a `Vec<Option<Sender>>`. It will stop notifying observers if the channel has
/// returned an error, which means it is closed or disconnected. However, we currently don't
/// compact the vector. Slots are reused for new observers, but the vector never shrinks.
///
/// In observe, we do loop the vector to find a free spot to re-use before pushing.
///
/// **Note**: we only detect that observers can be removed when [futures::SinkExt::send] or [Pharos::num_observers]
/// is being called. Otherwise, we won't find out about disconnected observers and the vector of observers
/// will not mark deleted observers and thus their slots can not be reused.
///
/// The [Sink] impl is not very optimized for the moment. It just loops over all observers in each poll method
/// so it will call `poll_ready` and `poll_flush` again for observers that already returned `Poll::Ready(Ok(()))`.
///
/// TODO: I will do some benchmarking and see if this can be improved, eg. by keeping a state which tracks which
/// observers we still have to poll.
//
pub struct Pharos<Event>  where Event: 'static + Clone + Send
{
	// Observers never get moved. Their index stays stable, so that when we free a slot,
	// we can store that in `free_slots`.
	//
	observers : Vec<Option< Sender<Event> >>,
	free_slots: Vec<usize>                  ,
	state     : State                       ,
}


#[ derive( Clone, Debug, PartialEq ) ]
//
enum State
{
	Ready,
	Closed,
}



impl<Event> fmt::Debug for Pharos<Event>  where Event: 'static + Clone + Send
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		write!( f, "pharos::Pharos<{}>", type_name::<Event>() )
	}
}



impl<Event> Pharos<Event>  where Event: 'static + Clone + Send
{
	/// Create a new Pharos. May it's light guide you to safe harbor.
	///
	/// You can set the initial capacity of the vector of senders, if you know you will a lot of observers
	/// it will save allocations by setting this to a higher number.
	///
	/// For pharos 0.4.0 on x64 Linux: `std::mem::size_of::<Option<Sender<_>>>() == 56 bytes`.
	//
	pub fn new( capacity: usize ) -> Self
	{
		Self
		{
			observers : Vec::with_capacity( capacity ),
			free_slots: Vec::with_capacity( capacity ),
			state     : State::Ready                  ,
		}
	}


	/// Returns the size of the vector used to store the observers. Useful for debugging and testing if it
	/// seems to get to big.
	//
	pub fn storage_len( &self ) -> usize
	{
		self.observers.len()
	}


	/// Returns the number of actual observers that are still listening (have not closed or dropped the Events).
	/// This will loop and it will verify for each if they are closed, clearing them from the internal storage
	/// if they are closed. This is similar to what notify does, but without sending an event.
	//
	pub fn num_observers( &mut self ) -> usize
	{
		let mut count = 0;


		for (i, opt) in self.observers.iter_mut().enumerate()
		{
			if let Some(observer) = opt
			{
				if !observer.is_closed()
				{
					count += 1;
				}

				else
				{
					self.free_slots.push( i );
					*opt = None
				}
			}
		}

		count
	}
}



/// Creates a new pharos, using 10 as the initial capacity of the vector used to store
/// observers. If this number does really not fit your use case, call [Pharos::new].
//
impl<Event> Default for Pharos<Event>  where Event: 'static + Clone + Send
{
	fn default() -> Self
	{
		Self::new( 10 )
	}
}


impl<Event> Observable<Event> for Pharos<Event>  where Event: 'static + Clone + Send
{
	type Error = Error;

	/// Will try to re-use slots in the vector from disconnected observers.
	//
	fn observe( &mut self, options: ObserveConfig<Event> ) -> Result< Events<Event>, Self::Error >
	{
		if self.state == State::Closed
		{
			return Err( ErrorKind::Closed.into() );
		}


		let (events, sender) = Events::new( options );


		// Try to reuse a free slot
		//
		if let Some( i ) = self.free_slots.pop()
		{
			self.observers[i] = Some( sender );
		}

		else
		{
			self.observers.push( Some( sender ) );
		}

		Ok( events )
	}
}



impl<Event> Sink<Event> for Pharos<Event> where Event: Clone + 'static + Send
{
	type Error = Error;


	fn poll_ready( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{

		if self.state == State::Closed
		{
			return Err( ErrorKind::Closed.into() ).into();
		}


		for obs in self.get_mut().observers.iter_mut()
		{
			if let Some( ref mut o ) = obs
			{
				let res = ready!( Pin::new( o ).poll_ready( cx ) );

				if res.is_err()
				{
					*obs = None;
				}
			}
		}

		Ok(()).into()
	}


	fn start_send( self: Pin<&mut Self>, evt: Event ) -> Result<(), Self::Error>
	{

		if self.state == State::Closed
		{
			return Err( ErrorKind::Closed.into() );
		}


		let this = self.get_mut();

		for (i, opt) in this.observers.iter_mut().enumerate()
		{
			// if this spot in the vector has a sender
			//
			if let Some( obs ) = opt
			{
				// if it's closed, let's remove it.
				//
				if obs.is_closed()
				{
					this.free_slots.push( i );

					*opt = None;
				}

				// else if it is interested in this event
				//
				else if obs.filter( &evt )
				{
					// if sending fails, remove it
					//
					if Pin::new( obs ).start_send( evt.clone() ).is_err()
					{
						this.free_slots.push( i );

						*opt = None;
					}
				}
			}
		}

		Ok(()).into()
	}



	fn poll_flush( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{

		if self.state == State::Closed
		{
			return Err( ErrorKind::Closed.into() ).into();
		}


		let mut pending = false;
		let     this    = self.get_mut();

		for (i, opt) in this.observers.iter_mut().enumerate()
		{
			if let Some( ref mut obs ) = opt
			{
				match Pin::new( obs ).poll_flush( cx )
				{
					Poll::Pending       => pending = true ,
					Poll::Ready(Ok())   => continue       ,

					Poll::Ready(Err(_)) =>
					{
						this.free_slots.push( i );

						*opt = None;
					}
				}
			}
		}


		Ok(()).into()
	}



	/// Will close and drop all observers. The pharos object will remain operational however.
	/// The main annoyance would be that we'd have to make
	//
	fn poll_close( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		if self.state == State::Closed
		{
			return Ok(()).into();
		}

		else
		{
			self.state = State::Closed;
		}


		let this = self.get_mut();

		for (i, opt) in this.observers.iter_mut().enumerate()
		{
			if let Some( ref mut obs ) = opt
			{
				let res = ready!( Pin::new( obs ).poll_close( cx ) );

				if res.is_err()
				{
					this.free_slots.push( i );

					*opt = None;
				}
			}
		}

		Ok(()).into()
	}
}





#[ cfg( test ) ]
//
mod tests
{
	use super::*;
	use futures::SinkExt;

	#[test]
	//
	fn debug()
	{
		let lighthouse = Pharos::<bool>::default();

		assert_eq!( "pharos::Pharos<bool>", &format!( "{:?}", lighthouse ) );
	}

	// #[test]
	// //
	// fn size_of_sender()
	// {
	// 	dbg!( std::mem::size_of::<Option<Sender<bool>>>() );
	// 	dbg!( std::mem::size_of::<Events<bool>>() );
	// }


	// verify storage_len and num_observers
	//
	#[test]
	//
	fn new()
	{
		let ph = Pharos::<bool>::new( 5 );

		assert_eq!( ph.observers.capacity(), 5 );
	}


	// verify storage_len and num_observers
	//
	#[test]
	//
	fn storage_len()
	{
		let mut ph = Pharos::<bool>::default();

			assert_eq!( ph.storage_len   (), 0 );
			assert_eq!( ph.num_observers (), 0 );
			assert_eq!( ph.free_slots.len(), 0 );

		let mut a = ph.observe( ObserveConfig::default() ).expect( "observe" );

			assert_eq!( ph.storage_len   (), 1 );
			assert_eq!( ph.num_observers (), 1 );
			assert_eq!( ph.free_slots.len(), 0 );

		let b = ph.observe( ObserveConfig::default() ).expect( "observe" );

			assert_eq!( ph.storage_len   (), 2 );
			assert_eq!( ph.num_observers (), 2 );
			assert_eq!( ph.free_slots.len(), 0 );

		a.close();

			assert_eq!( ph.storage_len  () , 2    );
			assert_eq!( ph.num_observers() , 1    );
			assert_eq!( &ph.free_slots     , &[0] );

		drop( b );

			assert_eq!( ph.storage_len  (), 2       );
			assert_eq!( ph.num_observers(), 0       );
			assert_eq!( &ph.free_slots    , &[0, 1] );
	}


	// Make sure we are reusing slots
	//
	#[test]
	//
	fn reuse()
	{
		let mut ph = Pharos::<bool>::default();
		let _a = ph.observe( ObserveConfig::default() );
		let  b = ph.observe( ObserveConfig::default() );
		let _c = ph.observe( ObserveConfig::default() );

			assert_eq!( ph.storage_len  (), 3 );
			assert_eq!( ph.num_observers(), 3 );

		drop( b );

			// It's important we call num_observers here, to clear the dropped one
			//
			assert_eq!( ph.storage_len  (), 3 );
			assert_eq!( ph.num_observers(), 2 );

			assert!( ph.observers[1].is_none() );
			assert_eq!( &ph.free_slots, &[1] );


		let _d = ph.observe( ObserveConfig::default() );

			assert_eq!( ph.storage_len   (), 3 );
			assert_eq!( ph.num_observers (), 3 );
			assert_eq!( ph.free_slots.len(), 0 );

		let _e = ph.observe( ObserveConfig::default() );

			// Now we should have pushed again
			//
			assert_eq!( ph.storage_len   (), 4 );
			assert_eq!( ph.num_observers (), 4);
			assert_eq!( ph.free_slots.len(), 0 );
	}


	// verify we can no longer observer after calling close
	//
	#[test]
	//
	fn observe_after_close()
	{
		let mut ph = Pharos::<bool>::default();

		futures::executor::block_on( ph.close() ).expect( "close" );

		assert!( ph.observe( ObserveConfig::default() ).is_err() );
	}
}
