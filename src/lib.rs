mod internal;
mod external;
mod bus;

use bus::BusComumication;
use external::ExternalCheck;
use internal::InternalCheck;

#[derive(Debug)]
pub enum ErrorType {
    INTERNAL,
    EXTERNAL,
}

#[allow(unused)]
pub struct ICSError{
    e_type: ErrorType,
    e_desc: String,
    e_id: usize,
}

#[allow(unused)]
pub struct ICS<MEX,BUS> where 
BUS: BusComumication<MEX>
{
    bus: BUS,
    int_vec: Vec<InternalCheck>,
    ext_vec: Vec<ExternalCheck<MEX,BUS>>,
}

#[allow(unused)]
impl<MEX,BUS> ICST<MEX,BUS,ICSError> for ICS<MEX,BUS>
where BUS: BusComumication<MEX>{
    fn add_internal_check(&mut self, check: InternalCheck) -> ()
    {
        self.int_vec.push(check);
    }
    fn add_external_check(&mut self, check: ExternalCheck<MEX,BUS>) -> ()
    {
        self.ext_vec.push(check);
    }
    fn run_check(& mut self) -> Option<Vec<ICSError>>
    {
        let mut res = vec![];
        let mut i =0;

        for int_check in &mut self.int_vec{
            match int_check.run_check() {
                internal::IntErrStatus::ERR => 
                {
                    res.push(ICSError{
                            e_type: ErrorType::INTERNAL,
                            e_desc: int_check.get_description().clone(),
                            e_id: i,
                        }
                    );
                },
                _ => {},
            }
            i = i + 1;
        }

        match res.len(){
            0 => None,
            _ => Some(res),
        }
    }
}

pub trait ICST<MEX,BUS,ERR> 
where BUS: BusComumication<MEX>{
    fn add_internal_check(&mut self, check: InternalCheck) -> ();
    fn add_external_check(&mut self, check: ExternalCheck<MEX,BUS>) -> ();
    fn run_check(& mut self) -> Option<Vec<ERR>>;
}
