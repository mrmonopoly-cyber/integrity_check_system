use super::generic_check::{GenericCheck,ErrFn,ErrStatus};
use super::ics_mex::ICSMex;
use core::result;


#[allow(unused)]
#[derive(Debug)]
pub struct ICSDep<const S: usize>{
    description: String,
    id: usize,
    part: usize,
    error_idx: Option<usize>,
    manage_fail: ErrFn,
    reset_fail: ErrFn,
    status: ErrStatus,
}

impl<const S :usize> GenericCheck for ICSDep<S>{
    fn get_description(&self) -> &String {
        &self.description
    }
}

#[allow(unused)]
impl<const S:usize> ICSDep<S>{

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
}
