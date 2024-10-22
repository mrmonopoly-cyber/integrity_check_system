use super::ics_trait::generic_check::ErrStatus;
use super::ics_trait::internal::*;
use super::ics_trait::generic_check::GenericCheck;
use super::ics_trait::external::ExternalCheck;

#[derive(Debug,Clone)]
pub enum ErrorType {
    INTERNAL,
    EXTERNAL,
}

#[allow(unused)]
#[derive(Debug,Clone)]
pub struct ICSError<'a>{
    e_type: ErrorType,
    e_desc: &'a String,
}

#[allow(unused)]
pub struct ICS<'a,M>{
    int_vec: Vec<(usize,InternalCheck)>,
    ext_vec: Vec<(usize,ExternalCheck<M>)>,
    err_vec: Vec<Option<ICSError<'a>>>,
}

#[allow(unused)]
impl<'a,M> ICS<'a,M> {
    pub fn new(int_err_cap: usize, ext_err_cap: usize, error_cap: usize) -> Self {
        let ev = Vec::with_capacity(error_cap);
        let ie = Vec::with_capacity(int_err_cap);
        let ee = Vec::with_capacity(ext_err_cap);
        Self {int_vec: ie,ext_vec: ee, err_vec: ev}
    }

    pub fn add_internal_check(&mut self, check: InternalCheck){
        let l = self.err_vec.len();
        self.int_vec.push((l,check));
        self.err_vec.push(None)
    }

    pub fn add_external_check(&mut self, check: ExternalCheck<M>) -> usize{
        let l = self.err_vec.len();
        self.ext_vec.push((l,check));
        self.err_vec.push(None);
        l
    }

    pub fn internal_check(&'a mut self) {
        for int_check in &mut self.int_vec{
            let (err_i,int_check) = int_check;
            let mut err_cel = &mut self.err_vec[*err_i];
            match int_check.run_check() {
                ErrStatus::ERR => 
                {
                    let err = ICSError{
                        e_type: ErrorType::INTERNAL,
                        e_desc: int_check.get_description(),
                    };

                    *err_cel = Some(err);
                },
                _ => *err_cel = None,
            }
        }
    }

    fn check_generic_mex(&'a mut self,mex: M){
        for ext_check in &mut self.ext_vec{
            let (err_i,ext_check) = ext_check;
            let mut err_cel = &mut self.err_vec[*err_i];
            match ext_check.check_mex(&mex) {
                ErrStatus::ERR => 
                {
                    let err = ICSError{
                        e_type: ErrorType::EXTERNAL,
                        e_desc: ext_check.get_description(),
                    };

                    *err_cel = Some(err);
                },
                _ => *err_cel = None,
            }
        }
    }

    pub fn check_specific_mex(&'a mut self,mex: &M, ext_err_index: usize){
        if ext_err_index >= self.ext_vec.len() {
            ()
        }

        let (ch_index,ext_check) = &self.ext_vec[ext_err_index];
        let mut err_cel = &mut self.err_vec[*ch_index];
        match ext_check.check_mex(mex) {
            ErrStatus::ERR=> {
                    let err = ICSError{
                        e_type: ErrorType::EXTERNAL,
                        e_desc: ext_check.get_description(),
                    };

                    *err_cel = Some(err);
            },
            ErrStatus::OK=> (),
            
        }
    }

    pub fn errors(&self) -> Vec<Option<ICSError>>{
        self.err_vec.clone()
    }
}

#[cfg(test)]
mod test{
}
