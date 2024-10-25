use super::generic_check::{GenericCheck,ErrFn,ErrStatus};
use super::ics_mex::ICSMex;
use core::result;


#[allow(unused)]
#[derive(Debug)]
pub struct ICSDep<'a,const S: usize>{
    description: &'a str,
    id: usize,
    part: usize,
    error_idx: Option<usize>,
    manage_fail: ErrFn,
    reset_fail: ErrFn,
    status: ErrStatus,
}

impl<'a,const S :usize> GenericCheck<'a> for ICSDep<'a,S>{
    fn get_description(&'a self) -> &'a str{
        &self.description
    }
}

#[allow(unused)]
impl<'a,const S:usize> ICSDep<'a,S>{

    pub fn new(
        description: &'a str,
        id: usize,
        part: usize,
        error_idx: Option<usize>,
        manage_fail: ErrFn,
        reset_fail: ErrFn,
        ) -> Self{
        ICSDep { 
            description, 
            id, 
            part, 
            error_idx, 
            manage_fail,
            reset_fail, 
            status: ErrStatus::OK 
        }
    }

    pub fn check_mex(&mut self, mex: &ICSMex<S>) -> result::Result<ErrStatus,&str>
    {
        if  mex.same_id_part(self.id, self.part) {
            match (mex.check_err(self.error_idx),&self.status){
                (true, ErrStatus::OK) => {
                    (self.manage_fail)();
                    self.status = ErrStatus::ERR;
                },
                (false, ErrStatus::ERR) => {
                    (self.reset_fail)();
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

#[cfg(test)]
mod test{
    use crate::ics_trait::{external::ICSDep, generic_check::ErrStatus};

    const STR : &str = "dep test";
    const ID : usize = 1;
    const PART : usize = 0;
    const ERR_IDX: usize = 0;

    #[test]
    fn create_dep() {
        let mf = || -> () {};
        let rf = || -> () {};
        let t : ICSDep<1>= ICSDep::new(STR, ID, PART, Some(ERR_IDX), mf, rf);

        assert_eq!(t.description,STR);
        assert_eq!(t.id,ID);
        assert_eq!(t.part,PART);
        assert_eq!(t.status,ErrStatus::OK);

    }
}
