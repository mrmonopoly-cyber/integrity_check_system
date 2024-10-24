use crate::ics_trait::generic_check::{ErrStatus,GenericCheck};

#[derive(Debug)]
pub enum OpAct{
    CHECK,
    FAIL,
    RESTORE,
}

#[allow(unused)]
pub struct InternalCheck<F>
where F: FnMut(OpAct) -> bool,{
    description: String,
    check :F,
    status: ErrStatus,
}

#[allow(unused)]
impl<F> GenericCheck  for InternalCheck<F>
where F: FnMut(OpAct) -> bool,{
    fn get_description(&self) -> &String{
        &self.description
    }
}

#[allow(unused)]
impl<F> InternalCheck<F>
where F: FnMut(OpAct) -> bool{
    pub fn new(description: String, check: F) -> Self{
        Self{description,check,status:ErrStatus::OK}
    }

    pub fn run_check(&mut self) -> ErrStatus
    {
        match ((self.check)(OpAct::CHECK),&self.status){
            (false,ErrStatus::OK) =>{
                self.status = ErrStatus::ERR;
                (self.check)(OpAct::FAIL);
                ErrStatus::ERR
            },
            (false,ErrStatus::ERR) =>{
                ErrStatus::ERR
            },
            (true,ErrStatus::ERR) =>{
                self.status = ErrStatus::OK;
                (self.check)(OpAct::RESTORE);
                ErrStatus::OK
            },
            (true,ErrStatus::OK) =>{
                ErrStatus::OK
            },
        }
    }
}

#[cfg(test)]
mod test{
    use core::sync::atomic;

    use super::InternalCheck;
    use super::OpAct;

    fn check_fun(act: OpAct, var: &atomic::AtomicUsize) -> bool{
        match act {
            OpAct::CHECK => {
                var.load(atomic::Ordering::Relaxed) < 10
            },
            OpAct::FAIL => {
                var.store(99, atomic::Ordering::Relaxed);
                true
            },
            OpAct::RESTORE =>{
                var.store(9, atomic::Ordering::Relaxed);
                true
            },
        }
    }

    fn run_test(check_seq: &[(usize,usize)]){
        let check_var = core::sync::atomic::AtomicUsize::new(0);
        let str = String::from("a");
        let check_f = |act : OpAct | -> bool {check_fun(act, &check_var)};
        let mut int_check = InternalCheck::new(str, check_f);
        for (i,d) in check_seq.iter(){
            check_var.store(*i, atomic::Ordering::Relaxed);
            int_check.run_check();
            assert_eq!(check_var.load(atomic::Ordering::Relaxed),*d);
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
}
