use crate::ics_trait::generic_check::IntErrStatus;
use crate::ics_trait::internal::*;

#[derive(Debug,Clone)]
pub enum ErrorType {
    INTERNAL,
}

#[allow(unused)]
#[derive(Debug,Clone)]
pub struct ICSError{
    e_type: ErrorType,
    e_desc: String,
    e_id: usize,
}

pub type CheckMexFn<M> = fn(M) -> bool;

#[allow(unused)]
pub struct ICS<M>{
    int_vec: Vec<(usize,InternalCheck)>,
    ext_vec: Vec<(usize,CheckMexFn<M>)>,
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

    fn add_external_check(&mut self, check: CheckMexFn<M>) -> usize{
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
                IntErrStatus::ERR => 
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
    }

    fn check_specific_mex(&mut self,mex: M, ext_err_index: usize){
        if ext_err_index >= self.ext_vec.len() {
            ()
        }
        let (ch_index,ext_check) = &self.ext_vec[ext_err_index];
        match ext_check(mex) {
            true => {
            },
            false => (),
            
        }
    }


    fn print_internal_check(&self) -> ()
    {
        let mut first = true;
        print!("internal checks: [");
        for (i,int_check) in &self.int_vec{
            if !first {
                print!(",");
            }
            print!("{} : {}",i,int_check.get_description());
            first=false;
        }
        print!("]");
    }
}


#[cfg(test)]
mod test{
}
