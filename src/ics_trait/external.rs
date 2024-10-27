use super::generic_check::{ErrStatus, GenericCheck, MexConseguence};
use super::ics_mex::ICSMex;
use core::result;


#[allow(unused)]
pub struct ICSDep<'a,const S: usize>{
    description: &'a str,
    id: usize,
    error_idx: Option<usize>,
    fail_par: &'a mut dyn MexConseguence,
    status: ErrStatus,
}

impl<'a,const S :usize> GenericCheck<'a> for ICSDep<'a,S>{
    fn get_description(&'a self) -> &'a str{
        &self.description
    }

    fn get_status(&self) -> ErrStatus {
        self.status.clone()
    }
}

#[allow(unused)]
impl<'a,const S:usize> ICSDep<'a,S>{
    pub fn new(
        description: &'a str,
        id: usize,
        error_idx: Option<usize>,
        fail_par: &'a mut dyn MexConseguence,
        ) -> Self{
        ICSDep { 
            description, 
            id, 
            error_idx, 
            fail_par,
            status: ErrStatus::OK 
        }
    }

    pub fn check_mex(&mut self, mex: &ICSMex<S>) -> result::Result<ErrStatus,&str>
    {
        if  mex.same_id(self.id) {
            match (mex.check_error(self.error_idx),&self.status){
                (true, ErrStatus::OK) => {
                    self.fail_par.manage_fail();
                    self.status = ErrStatus::ERR;
                },
                (false, ErrStatus::ERR) => {
                    self.fail_par.restore_fail();
                    self.status = ErrStatus::OK;
                },
                _ => (),
            };
            Ok(self.status.clone())
        }else{
            Err("invalid id mex or part mex")
        }
    }
    
}

#[allow(unused)]
#[cfg(test)]
mod test{
    use crate::{debug_check::CheckU8, ics_trait::{external::ICSDep, generic_check::{ErrStatus, GenericCheck}, ics_mex::ICSMex}};
    use core::{result, sync::atomic::{AtomicI8, Ordering}};
    use core::sync::atomic::AtomicU8;

    const STR : &str = "dep test";
    const ID : usize = 1;
    const PART : usize = 0;
    const ERR_IDX: usize = 0;

    #[test]
    fn create_dep() {
        let var = AtomicU8::new(6);
        let mut fail_par : CheckU8<0, 10, 99, 0> = CheckU8::new(&var);
        let dep : ICSDep<2> = ICSDep::new(STR, ID, Some(0), &mut fail_par);
        assert_eq!(dep.get_description(),STR);
        assert_eq!(dep.get_status(),ErrStatus::OK);
    }

    #[test]
    fn discard_wrong_mex_wrong_id() {
        let var = AtomicU8::new(6);
        let mut fail_par : CheckU8<0, 10, 99, 0> = CheckU8::new(&var);
        let mut dep : ICSDep<2> = ICSDep::new(STR, ID, Some(0), &mut fail_par);
        let wrong_id = ICSMex::new(ID+1, 0, [0;2]);
        let r = dep.check_mex(&wrong_id);
        assert_eq!(r, Err("invalid id mex or part mex") );
    }

    #[test]
    fn recognize_err_in_mex() {
        let var = AtomicU8::new(6);
        let mut fail_par : CheckU8<0, 10, 99, 0> = CheckU8::new(&var);
        let mut dep : ICSDep<2> = ICSDep::new(STR, ID, Some(5), &mut fail_par);
        let mut err_mex = ICSMex::new(ID, 0, [0;2]);
        err_mex.set_err(0, 5);
        let r = dep.check_mex(&err_mex);
        assert_eq!(r, Ok(ErrStatus::ERR) );
    }

    #[test]
    fn recognize_err_in_any_pos() {
        let var = AtomicU8::new(6);
        let mut fail_par : CheckU8<0, 10, 99, 0> = CheckU8::new(&var);
        let mut dep : ICSDep<2> = ICSDep::new(STR, ID, None, &mut fail_par);
        let mut err_mex = ICSMex::new(ID, 0, [0;2]);
        err_mex.set_err(0, 5);
        let r = dep.check_mex(&err_mex);
        assert_eq!(r, Ok(ErrStatus::ERR) );
    }

    #[test]
    fn recognize_ok_mex() {
        let var = AtomicU8::new(6);
        let mut fail_par : CheckU8<0, 10, 99, 0> = CheckU8::new(&var);
        let mut dep : ICSDep<2> = ICSDep::new(STR, ID, Some(1), &mut fail_par);
        let mut err_mex = ICSMex::new(ID, 0, [0;2]);
        err_mex.set_err(0, 5);
        let r = dep.check_mex(&err_mex);
        assert_eq!(r, Ok(ErrStatus::OK) );
    }
}
