mod internal;
mod external;
mod bus;

use bus::BusComumication;
use external::ExternalCheck;
use internal::InternalCheck;

pub struct ICSError{
}

#[allow(unused)]
pub struct ICS<MEX,INT,BUS> where 
BUS: BusComumication<MEX>,INT:InternalCheck
{
    bus: BUS,
    int_vec: Vec<INT>,
    ext_vec: Vec<ExternalCheck<MEX,BUS>>,
}

pub trait ICST<MEX,BUS,I,ERR> 
where BUS: BusComumication<MEX>, I: InternalCheck{
    fn add_internal_check(&mut self, check: I) -> ();
    fn add_external_check(&mut self, check: ExternalCheck<MEX,BUS>) -> ();
    fn run_check(& mut self) -> Option<ERR>;
}


#[allow(unused)]
impl<MEX,BUS,INT> ICST<MEX,BUS,INT,ICSError> for ICS<MEX,INT,BUS>
where BUS: BusComumication<MEX>, INT: InternalCheck{
    fn add_internal_check(&mut self, check: INT) -> ()
    {
    }
    fn add_external_check(&mut self, check: ExternalCheck<MEX,BUS>) -> ()
    {
    }
    fn run_check(& mut self) -> Option<ICSError>
    {
        None
    }
}
