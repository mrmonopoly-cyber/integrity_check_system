type CheckFn = fn() -> bool;
type IntErrFn= fn() -> ();

pub enum IntErrStatus{
    OK,
    ERR,
}

#[allow(unused)]
pub struct InternalCheck{
    description: String,
    check :CheckFn,
    manage_fail: IntErrFn,
    reset_fail: IntErrFn,
    status: IntErrStatus,
}

#[allow(unused)]
impl InternalCheck{
    pub fn new(description: String, check: CheckFn, 
        manage_fail: IntErrFn, reset_fail: IntErrFn) -> Self{
        Self{description,check,manage_fail,reset_fail,status:IntErrStatus::OK}
    }
    pub fn run_check(&mut self) -> IntErrStatus
    {
        match ((self.check)(),&self.status){
            (true,IntErrStatus::OK) =>{
                self.status = IntErrStatus::ERR;
                (self.manage_fail)();
                IntErrStatus::ERR
            },
            (true,IntErrStatus::ERR) =>{
                IntErrStatus::ERR
            },
            (false,IntErrStatus::ERR) =>{
                self.status = IntErrStatus::OK;
                (self.reset_fail)();
                IntErrStatus::OK
            },
            (false,IntErrStatus::OK) =>{
                IntErrStatus::OK
            },
        }
    }

    pub fn get_description(&self) -> &String{
        &self.description
    }
}
