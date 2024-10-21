use super::generic_check::ErrStatus;
use super::internal::*;
use super::generic_check::GenericCheck;
use super::external::ExternalCheck;

#[derive(Debug,Clone)]
pub enum ErrorType {
    INTERNAL,
    EXTERNAL,
}

#[allow(unused)]
#[derive(Debug,Clone)]
pub struct ICSError{
    e_type: ErrorType,
    e_desc: String,
    e_id: usize,
}

#[allow(unused)]
pub struct ICS<M>{
    int_vec: Vec<(usize,InternalCheck)>,
    ext_vec: Vec<(usize,ExternalCheck<M>)>,
    err_vec: Vec<Option<ICSError>>,
}

#[allow(unused)]
impl<M> ICS<M> {
    fn add_internal_check(&mut self, check: InternalCheck) -> usize{
        let l = self.err_vec.len();
        self.int_vec.push((l,check));
        self.err_vec.push(None);
        l
    }

    fn add_external_check(&mut self, check: ExternalCheck<M>) -> usize{
        let l = self.err_vec.len();
        self.ext_vec.push((l,check));
        self.err_vec.push(None);
        l
    }

    fn internal_check(& mut self) {
        for int_check in &mut self.int_vec{
            let (err_i,int_check) = int_check;
            let mut err_cel = &mut self.err_vec[*err_i];
            match int_check.run_check() {
                ErrStatus::ERR => 
                {
                    let err = ICSError{
                        e_type: ErrorType::INTERNAL,
                        e_desc: int_check.get_description().clone(),
                        e_id: *err_i,
                    };

                    *err_cel = Some(err);
                },
                _ => *err_cel = None,
            }
        }
    }

    fn check_generic_mex(&mut self,mex: M){
        for ext_check in &mut self.ext_vec{
            let (err_i,ext_check) = ext_check;
            let mut err_cel = &mut self.err_vec[*err_i];
            match ext_check.check_mex(&mex) {
                ErrStatus::ERR => 
                {
                    let err = ICSError{
                        e_type: ErrorType::EXTERNAL,
                        e_desc: ext_check.get_description().clone(),
                        e_id: *err_i,
                    };

                    *err_cel = Some(err);
                },
                _ => *err_cel = None,
            }
        }
    }

    fn check_specific_mex(&mut self,mex: &M, ext_err_index: usize){
        if ext_err_index >= self.ext_vec.len() {
            ()
        }
        let (ch_index,ext_check) = &self.ext_vec[ext_err_index];
        let mut err_cel = &mut self.err_vec[*ch_index];
        match ext_check.check_mex(mex) {
            ErrStatus::ERR=> {
                    let err = ICSError{
                        e_type: ErrorType::EXTERNAL,
                        e_desc: ext_check.get_description().clone(),
                        e_id: *ch_index,
                    };

                    *err_cel = Some(err);
            },
            ErrStatus::OK=> (),
            
        }
    }
}


#[cfg(test)]
mod test{
}
