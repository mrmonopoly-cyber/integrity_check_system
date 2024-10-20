mod internal;
mod external;
mod bus;

use bus::BusComumication;
use external::ExternalCheck;
use internal::InternalCheck;

pub trait ICS1<MEX,BUS,I,ERR> 
where BUS: BusComumication<MEX>, I: InternalCheck{
    fn new(node_id: u8,bus :BUS) -> Self;
    fn add_internal_check(&mut self, check: I) -> ();
    fn add_external_check(&mut self, check: ExternalCheck<MEX,BUS>) -> ();
    fn run_check(& mut self) -> (bool,ERR);
}
