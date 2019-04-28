use crate :: { *, import::* };

// This is an implentation detail. Unfortunately orphan rules will not allow us to define this
// default impl in an implementation crate.
//
default impl<T, Event> Observable< Event, UnboundedReceiver<Event>, UnboundedSender<Event> > for T

	where Event: Named + Clone + 'static + Send,
	      //T    : Send,

{
	fn observe( &mut self, name: Arc<str> ) -> UnboundedReceiver<Event>
	{
		let (tx, rx) = mpsc::unbounded();

		self.observers().insert( name, tx );

		rx
	}


	fn notify( &mut self, evt: Event ) -> ReturnNoSend< Result<(), Error> >
	{
		async move
		{
			for (name, ref mut tx) in self.observers()
			{
				let mut e = evt.clone();
				e.set_name( name.clone() );

				// TODO: this is a problem. If one channel fails, the others won't be tried anymore.
				//
				await!( tx.send( e ) )?;
			}

			Ok(())

		}.boxed()
	}
}

