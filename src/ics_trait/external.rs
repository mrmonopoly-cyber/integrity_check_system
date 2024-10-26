use super::generic_check::{ErrStatus, GenericCheck, MexConseguence};
use super::ics_mex::ICSMex;
use core::result;


#[allow(unused)]
pub struct ICSDep<'a,const S: usize>{
    description: &'a str,
    id: usize,
    part: usize,
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
        part: usize,
        error_idx: Option<usize>,
        fail_par: &'a mut dyn MexConseguence,
        ) -> Self{
        ICSDep { 
            description, 
            id, 
            part, 
            error_idx, 
            fail_par,
            status: ErrStatus::OK 
        }
    }

    pub fn check_mex(&mut self, mex: &ICSMex<S>) -> result::Result<ErrStatus,&str>
    {
        if  mex.same_id_part(self.id, self.part) {
            match (mex.check_err(self.error_idx),&self.status){
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
    use crate::ics_trait::{external::ICSDep, generic_check::ErrStatus, ics_mex::ICSMex};
    use core::{result, sync::atomic::{AtomicI8, Ordering}};

    const STR : &str = "dep test";
    const ID : usize = 1;
    const PART : usize = 0;
    const ERR_IDX: usize = 0;

    fn fail_f(p: &AtomicI8) {
        p.store(-1, Ordering::Relaxed);
    }

    fn rest_f(p: &AtomicI8) {
        p.store(0, Ordering::Relaxed);
    }

    #[test]
    fn create_dep() {
        todo!()
    }

    #[test]
    fn discard_wrong_mex_wrong_id() {
        todo!()
    }

    #[test]
    fn discard_wrong_mex_wrong_part() {
        todo!()
    }

    #[test]
    fn discard_wrong_mex_wrong_id_part() {
        todo!()
    }

    #[test]
    fn recognize_err_in_mex() {
        todo!()
    }

    #[test]
    fn recognize_err_in_any_pos() {
        todo!()
    }

    #[test]
    fn recognize_ok_mex() {
        todo!()
    }
}
