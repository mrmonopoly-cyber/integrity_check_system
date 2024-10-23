use super::generic_check::{GenericCheck,ErrFn,ErrStatus};
use super::ics_mex::{ICSMex,Integer};


#[allow(unused)]
#[derive(Debug)]
pub struct ICSDep<IS,PS,const S: usize> 
where IS: Integer,
      PS: Integer,{
    description: String,
    id: IS,
    part: PS,
    error_idx: Option<usize>,
    manage_fail: ErrFn,
    reset_fail: ErrFn,
    status: ErrStatus,
}

impl<IS,PS,const S :usize> GenericCheck for ICSDep<IS,PS,S>
where IS: Integer,
      PS: Integer,{
    fn get_description(&self) -> &String {
        &self.description
    }
}

#[allow(unused)]
impl<IS,PS,const S:usize> ICSDep<IS,PS,S>
where IS: Integer,
      PS: Integer,{
    pub fn check_mex(&mut self, mex: &ICSMex<IS,PS,S>) -> ErrStatus
    {
        if  mex.same_id_part(self.id, self.part) {
            match (mex.check_err(self.error_idx),&self.status){
                (true, ErrStatus::OK) => {
                    (self.manage_fail)();
                    self.status = ErrStatus::ERR;
                    ErrStatus::ERR
                },
                (true, ErrStatus::ERR) =>ErrStatus::ERR,
                (false, ErrStatus::OK) => ErrStatus::OK,
                (false, ErrStatus::ERR) => {
                    (self.reset_fail)();
                    self.status = ErrStatus::OK;
                    ErrStatus::OK
                },
            }
        }else{
            ErrStatus::OK
        }
    }
    
}

#[cfg(test)]
mod test{
}
