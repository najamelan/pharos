use crate :: { import::*, Observable, Events, ObserveConfig, events::Sender };


/// The Pharos lighthouse. When you implement Observable on your type, you can forward
/// the [`observe`](Observable::observe) method to Pharos and call notify on it.
///
/// You can of course create several `Pharos` (I know, historical sacrilege) for (different) types
/// of events.
///
/// Please see the docs for [Observable] for an example. Others can be found in the README and
/// the [examples](https://github.com/najamelan/pharos/tree/master/examples) directory of the repository.
//
pub struct Pharos<Event>  where Event: 'static + Clone + Send
{
	observers: Vec<Option< Sender<Event> >>,
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
	/// Create a new Pharos. May it's light guide you to safe harbour.
	///
	/// You can set the initial capacity of the vector of senders, if you know you will a lot of observers
	/// it will save allocations by setting this to a higher number.
	///
	/// For pharos 0.3.0 on x64 linux: std::mem::size_of::<Option<Sender<_>>>() == 56 bytes.
	//
	pub fn new( capacity: usize ) -> Self
	{
		Self
		{
			observers: Vec::with_capacity( capacity ),
		}
	}



	/// Notify all observers of Event evt.
	//
	pub async fn notify( &mut self, evt: &Event )
	{
		// Try to send to all channels in parallel, so they can all start processing this event
		// even if one of them is blocked on a full queue.
		//
		// We can not have mutable access in parallel, so we take options out and put them back. This
		// allocates a new vector every time. If you have a better idea, please open an issue!
		//
		// The output of the join is a vec of options with the disconnected observers removed.
		//
		let fut = join_all
		(
			( 0..self.observers.len() ).map( |i|
			{
				let opt = self.observers[i].take();
				let evt = evt.clone();

				async move
				{
					let mut new = None;

					if let Some( mut s ) = opt
					{
						match s.notify( &evt ).await
						{
							true  => new = Some( s ),
							false => {}
						}
					}

					new
				}

			})
		);


		// Put back the observers that we "borrowed"
		// TODO: compact the vector from time to time?
		//
		self.observers = fut.await;
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

		for opt in self.observers.iter_mut()
		{
			if let Some(observer) = opt.take()
			{
				match observer.is_closed()
				{
					true => {}

					false =>
					{
						count += 1;
						*opt = Some( observer );
					}
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
	/// Add an observer to the pharos. This will use a bounded channel of the size of `queue_size`.
	/// Note that the use of a bounded channel provides backpressure and can slow down the observed
	/// task.
	//
	fn observe( &mut self, options: ObserveConfig<Event> ) -> Events<Event>
	{
		let (events, sender) = Events::new( options );

		self.observers.push( Some(sender) );

		events
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
		let lighthouse = Pharos::<bool>::default();

		assert_eq!( "pharos::Pharos<bool>", &format!( "{:?}", lighthouse ) );
	}

	// #[test]
	// //
	// fn size_of_sender()
	// {
	// 	dbg!( std::mem::size_of::<Option<Sender<bool>>>() );
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

			assert_eq!( ph.storage_len  (), 0 );
			assert_eq!( ph.num_observers(), 0 );

		let mut a = ph.observe( ObserveConfig::default() );

			assert_eq!( ph.storage_len  (), 1 );
			assert_eq!( ph.num_observers(), 1 );

		let b = ph.observe( ObserveConfig::default() );

			assert_eq!( ph.storage_len  (), 2 );
			assert_eq!( ph.num_observers(), 2 );

		a.close();

			assert_eq!( ph.storage_len  (), 2 );
			assert_eq!( ph.num_observers(), 1 );

		drop( b );

			assert_eq!( ph.storage_len  (), 2 );
			assert_eq!( ph.num_observers(), 0 );
	}
}
