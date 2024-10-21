use crate::ics_trait::generic_check::{ErrStatus,ErrFn,GenericCheck};

pub type CheckFn = fn() -> bool;

#[allow(unused)]
pub struct InternalCheck{
    description: String,
    check :CheckFn,
    manage_fail: ErrFn,
    reset_fail: ErrFn,
    status: ErrStatus,
}

#[allow(unused)]
impl GenericCheck  for InternalCheck{
    fn get_description(&self) -> &String{
        &self.description
    }
}

impl InternalCheck{
    pub fn new(description: String, check: CheckFn, 
        manage_fail: ErrFn, reset_fail: ErrFn) -> Self{
        Self{description,check,manage_fail,reset_fail,status:ErrStatus::OK}
    }

    pub fn run_check(&mut self) -> ErrStatus
    {
        match ((self.check)(),&self.status){
            (true,ErrStatus::OK) =>{
                self.status = ErrStatus::ERR;
                (self.manage_fail)();
                ErrStatus::ERR
            },
            (true,ErrStatus::ERR) =>{
                ErrStatus::ERR
            },
            (false,ErrStatus::ERR) =>{
                self.status = ErrStatus::OK;
                (self.reset_fail)();
                ErrStatus::OK
            },
            (false,ErrStatus::OK) =>{
                ErrStatus::OK
            },
        }
    }
}

