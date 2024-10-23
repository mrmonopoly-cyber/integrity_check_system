use crate::ics_trait::generic_check::{ErrStatus,GenericCheck};

#[derive(Debug)]
pub enum OpAct{
    CHECK,
    FAIL,
    RESTORE,
}

#[allow(unused)]
pub struct InternalCheck<F>
where F: FnMut(OpAct) -> bool,{
    description: String,
    check :F,
    status: ErrStatus,
}

#[allow(unused)]
impl<F> GenericCheck  for InternalCheck<F>
where F: FnMut(OpAct) -> bool,{
    fn get_description(&self) -> &String{
        &self.description
    }
}

#[allow(unused)]
impl<F> InternalCheck<F>
where F: FnMut(OpAct) -> bool{
    pub fn new(description: String, check: F) -> Self{
        Self{description,check,status:ErrStatus::OK}
    }

    pub fn run_check(&mut self) -> ErrStatus
    {
        match ((self.check)(OpAct::CHECK),&self.status){
            (true,ErrStatus::OK) =>{
                self.status = ErrStatus::ERR;
                (self.check)(OpAct::FAIL);
                ErrStatus::ERR
            },
            (true,ErrStatus::ERR) =>{
                ErrStatus::ERR
            },
            (false,ErrStatus::ERR) =>{
                self.status = ErrStatus::OK;
                (self.check)(OpAct::RESTORE);
                ErrStatus::OK
            },
            (false,ErrStatus::OK) =>{
                ErrStatus::OK
            },
        }
    }
}

#[cfg(test)]
mod test{
    use crate::ics_trait::generic_check::ErrStatus;

    use super::InternalCheck;
    use super::OpAct;

    #[test]
    fn create_internal_check(){
        let mut check_var = 10;
        let check_f = |act : OpAct | -> bool {
            match act {
                OpAct::CHECK => check_var < 10,
                OpAct::FAIL => {
                    check_var = 99; 
                    true
                },
                OpAct::RESTORE =>{
                    check_var = 10;
                    true
                },
            }
        };
        let str = String::from("test init");

        let mut int_check = InternalCheck::new(str, check_f);
        
        assert_eq!(int_check.run_check(),ErrStatus::OK);

    }
}
