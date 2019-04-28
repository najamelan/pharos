use crate::import::*;

pub trait Named
{
	fn name    ( &self                     ) -> Arc<str> ;
	fn set_name( &mut self, name: Arc<str> )             ;
}
