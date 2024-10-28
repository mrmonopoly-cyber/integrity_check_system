use crate::ics_trait::generic_check::{ErrStatus,GenericCheck};

use super::generic_check::ObjectCheck;

#[allow(unused)]
#[derive(Debug)]
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
    pub fn new(description: &'a str,volatile_par: &'a mut dyn ObjectCheck,) -> Self
    {
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
    use crate::debug_check::{CheckU8,CheckVr, CheckWithEnv};
    use crate::ics_trait::generic_check::MexConseguence;
    use crate::ics_trait::{generic_check:: ObjectCheck};
    use crate::ics_trait::internal::*;
    static STR: &str= "internal_check_test";

    fn run_test(check_seq: &[(u8,u8)]){
        let p = atomic::AtomicU8::new(18);
        let mut cp : CheckU8<0, 10, 99, 9> = CheckU8::new(&p);
        let mut ic = InternalCheck::new(STR, &mut cp);
        for (before,after) in check_seq.iter(){
            p.store(*before, atomic::Ordering::Relaxed);
            ic.run_check();
            assert_eq!(p.load(atomic::Ordering::Relaxed),*after);
        }
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
        let p = atomic::AtomicU8::new(18);
        let p_1 = atomic::AtomicU8::new(0);
        let p_2 = atomic::AtomicU8::new(5);
        let mut cp : CheckU8<0, 20, 22, 10> = CheckU8::new(&p);
        let mut cp_1 : CheckU8<0, 10, 15, 0> = CheckU8::new(&p_1);

        let check_f = || true;
        let fail_f = || ();
        let restore_f = || ();
        let mut cp_1  = CheckWithEnv::new(&p_2, check_f, fail_f, restore_f);
        let mut cs = CheckVr::new("hello");
        let ic = InternalCheck::new(STR, &mut cp);
        let ic_1 = InternalCheck::new(STR, &mut cp_1);
        let ic_2 = InternalCheck::new(STR, &mut cs);
        cs.update("fail");
        assert_eq!(ic.get_description(),STR);
        assert_eq!(ic_1.get_description(),STR);
    }
}
