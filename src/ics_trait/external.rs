use super::generic_check::{GenericCheck,ErrFn,ErrStatus};

pub type CheckFn<M> = fn(M) -> ErrStatus;

#[derive(Debug)]
struct ExternalCheck<M> {
    description: String,
    check_mex_fn: CheckFn<M>,
    manage_fail: ErrFn,
    reset_fail: ErrFn,
}

impl<M> GenericCheck for ExternalCheck<M>{
    fn get_description(&self) -> &String {
        &self.description
    }
}

impl<M> ExternalCheck<M> {
    fn new(description: String, check_mex_fn: CheckFn<M>, 
        manage_fail: ErrFn, reset_fail: ErrFn) -> Self{
        Self{description,check_mex_fn,manage_fail,reset_fail}
    }

    fn check_mex(&self, mex: M) -> ErrStatus
    {
        (self.check_mex_fn)(mex)
    }
    
}

