use crate::ics_trait::generic_check::{ErrStatus,GenericCheck};

use super::generic_check::ObjectCheck;

#[allow(unused)]
pub struct InternalCheck<'a>{
    volatile_par: &'a mut dyn ObjectCheck,
    description: &'a str,
    status: ErrStatus,
}

#[allow(unused)]
impl<'a> GenericCheck<'a>  for InternalCheck<'a>{
    fn get_description(&self) -> &'a str{
        &self.description
    }

    fn get_status(&self) -> ErrStatus {
        self.status.clone()
    }

}

#[allow(unused)]
impl<'a> InternalCheck<'a> where{
    pub fn new(description: &'a str,volatile_par: &'a mut dyn ObjectCheck,) -> Self{
        Self{volatile_par,description,status:ErrStatus::OK}
    }

    pub fn run_check(&mut self) -> ErrStatus
    {
        match (self.volatile_par.check(),&self.status){
            (false,ErrStatus::OK) =>{
                self.status = ErrStatus::ERR;
                self.volatile_par.manage_fail();
                ErrStatus::ERR
            },
            (false,ErrStatus::ERR) =>{
                ErrStatus::ERR
            },
            (true,ErrStatus::ERR) =>{
                self.status = ErrStatus::OK;
                self.volatile_par.restore_fail();
                ErrStatus::OK
            },
            (true,ErrStatus::OK) =>{
                ErrStatus::OK
            },
        }
    }
}

#[allow(unused)]
#[cfg(test)]
mod test{
    use core::sync::atomic;
    use crate::ics_trait::generic_check:: ObjectCheck;
    static STR: &str= "internal_check_test";

    fn run_test(check_seq: &[(usize,usize)]){
        todo!()
    }

    #[test]
    fn valid_check_var(){
        let tv = [(9,9)];
        run_test(&tv);
    }

    #[test]
    fn invalid_check_var(){
        let tv = [(11,99)];
        run_test(&tv);
    }

    #[test]
    fn valid_restore(){
        let tv = [(11,99),(7,9),(2,2)];
        run_test(&tv);
    }

    #[test]
    fn valid_description(){
        todo!()
    }
}
