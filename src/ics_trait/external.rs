use super::generic_check::{ErrStatus, GenericCheck, MexConseguence};
use super::ics_mex::ICSMex;

#[allow(unused)]
#[derive(Debug)]
pub struct ICSDep<'a,const S: usize,TID>
where 
TID: Copy + core::cmp::PartialEq,
{
    description: &'a str,
    id: TID,
    error_idx: Option<usize>,
    fail_par: &'a mut dyn MexConseguence,
    status: ErrStatus,
}

impl<'a,const S :usize,TID> GenericCheck<'a> for ICSDep<'a,S,TID>
where 
TID: Copy + core::cmp::PartialEq,
{
    fn get_description(&'a self) -> &'a str{
        &self.description
    }

    fn get_status(&self) -> ErrStatus {
        self.status.clone()
    }
}

#[allow(unused)]
impl<'a,const S:usize,TID> ICSDep<'a,S,TID>
where 
TID:  Copy + core::cmp::PartialEq,
{
    pub fn new(
        description: &'a str,
        id: TID,
        error_idx: Option<usize>,
        fail_par: &'a mut dyn MexConseguence,
        ) -> Self
    {
        ICSDep { 
            description, 
            id, 
            error_idx, 
            fail_par,
            status: ErrStatus::OK 
        }
    }

    pub fn check_mex<TPART>(&mut self, mex: &ICSMex<S,TID,TPART>) -> Result<ErrStatus,&str>
    where 
        TPART: Copy +  Into<usize> + TryFrom<usize>
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
    use crate::{debug_check::CheckU8, ics_trait::{external::ICSDep, generic_check::{ErrStatus, GenericCheck}, ics_mex::{ICSMex, ICSMexFull}}};
    use core::{result, sync::atomic::{AtomicI8, Ordering}, usize};
    use core::sync::atomic::AtomicU8;

    const STR : &str = "dep test";
    const ID : usize = 1;
    const PART : usize = 0;
    const ERR_IDX: usize = 0;

    #[test]
    fn create_dep() {
        let var = AtomicU8::new(6);
        let mut fail_par : CheckU8<0, 10, 99, 0> = CheckU8::new(&var);
        let dep : ICSDep<2,usize> = ICSDep::new(STR, ID, Some(0), &mut fail_par);
        assert_eq!(dep.get_description(),STR);
        assert_eq!(dep.get_status(),ErrStatus::OK);
    }

    #[test]
    fn discard_wrong_mex_wrong_id() {
        let var = AtomicU8::new(6);
        let mut fail_par : CheckU8<0, 10, 99, 0> = CheckU8::new(&var);
        let mut dep : ICSDep<2,usize> = ICSDep::new(STR, ID, Some(0), &mut fail_par);
        let mut m : ICSMexFull<2, usize, usize> = ICSMexFull::new(ID+1, 2);
        let wrong_id = m.get_part(0).ok().unwrap();
        let r = dep.check_mex(wrong_id);
        assert_eq!(r, Err("invalid id mex or part mex") );
    }

    #[test]
    fn recognize_err_in_mex() {
        let var = AtomicU8::new(6);
        let mut fail_par : CheckU8<0, 10, 99, 0> = CheckU8::new(&var);
        let mut dep : ICSDep<2,usize> = ICSDep::new(STR, ID, Some(5), &mut fail_par);
        let mut m : ICSMexFull<2, usize, usize>= ICSMexFull::new(ID, 2);
        let err_mex = m.get_part(0).ok().unwrap();
        err_mex.set_err(0, 5);
        let r = dep.check_mex(&err_mex);
        assert_eq!(r, Ok(ErrStatus::ERR) );
    }

    #[test]
    fn recognize_err_in_any_pos() {
        let var = AtomicU8::new(6);
        let mut fail_par : CheckU8<0, 10, 99, 0> = CheckU8::new(&var);
        let mut dep : ICSDep<2,usize> = ICSDep::new(STR, ID, None, &mut fail_par);
        let mut m : ICSMexFull<2, usize, usize> = ICSMexFull::new(ID, 10);
        let err_mex = m.get_part(0).ok().unwrap();
        err_mex.set_err(0, 5);
        let r = dep.check_mex(&err_mex);
        assert_eq!(r, Ok(ErrStatus::ERR) );
    }

    #[test]
    fn recognize_ok_mex() {
        let var = AtomicU8::new(6);
        let mut fail_par : CheckU8<0, 10, 99, 0> = CheckU8::new(&var);
        let mut dep : ICSDep<2,usize> = ICSDep::new(STR, ID, Some(1), &mut fail_par);
        let mut m : ICSMexFull<2, usize, usize> = ICSMexFull::new(ID, 10);
        let err_mex = m.get_part(0).ok().unwrap();
        err_mex.set_err(0, 5);
        let r = dep.check_mex(&err_mex);
        assert_eq!(r, Ok(ErrStatus::OK) );
    }
}
