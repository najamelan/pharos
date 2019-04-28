use crate :: { *, import::* };

pub trait Observable<Event, Rx, Tx>

	where Event: Named + Clone + 'static + Send ,
	      Tx   : Sink<Event>                    ,
	      Rx   : Stream<Item=Event>             ,
{
	fn observers(&mut self) -> &mut HashMap< Arc<str>, Tx >;

	fn observe( &mut self, name: Arc<str> ) -> Rx;

	fn notify( &mut self, evt: Event ) -> ReturnNoSend< Result<(), Error> >;
}
