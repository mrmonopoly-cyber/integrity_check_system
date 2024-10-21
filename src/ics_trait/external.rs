use super::generic_check::{GenericCheck,ErrFn,ErrStatus};

type CheckFn<M> = fn(&M) -> bool;

#[allow(unused)]
#[derive(Debug)]
pub struct ExternalCheck<M> {
    description: String,
    check_mex_fn: CheckFn<M>,
    manage_fail: ErrFn,
    reset_fail: ErrFn,
    status: ErrStatus,
}

impl<M> GenericCheck for ExternalCheck<M>{
    fn get_description(&self) -> &String {
        &self.description
    }
}

#[allow(unused)]
impl<M> ExternalCheck<M> {
    pub fn new(description: String, check_mex_fn: CheckFn<M>, 
        manage_fail: ErrFn, reset_fail: ErrFn) -> Self{
        Self{description,check_mex_fn,manage_fail,reset_fail,status: ErrStatus::OK}
    }

    pub fn check_mex(&self, mex: &M) -> ErrStatus
    {
        match ((self.check_mex_fn)(mex),&self.status){
            (true,ErrStatus::OK) => ErrStatus::OK,
            (true,ErrStatus::ERR) => {
                (self.reset_fail)();
                ErrStatus::OK
            },
            (false,ErrStatus::OK) => ErrStatus::OK,
            (false,ErrStatus::ERR) => {
                (self.manage_fail)();
                ErrStatus::ERR
            },
        }
    }
    
}

