use crate::ics_trait::generic_check::{ErrStatus,GenericCheck};

#[allow(unused)]
pub struct InternalCheck<FC,FF,FR> where
    FC: FnMut() -> bool,
    FF: FnMut() -> (),
    FR: FnMut() -> (),
      {
    description: String,
    check :FC,
    fail: FF,
    restore: FR,
    status: ErrStatus,
}

#[allow(unused)]
impl<FC,FF,FR> GenericCheck  for InternalCheck<FC,FF,FR> where
    FC: FnMut() -> bool,
    FF: FnMut() -> (),
    FR: FnMut() -> (),{
    fn get_description(&self) -> &String{
        &self.description
    }
}

#[allow(unused)]
impl<FC,FF,FR> InternalCheck<FC,FF,FR> where
FC: FnMut() -> bool,
FF: FnMut() -> (),
FR: FnMut() -> (){
    pub fn new(description: String, check: FC,fail: FF,restore: FR) -> Self{
        Self{description,check,fail,restore,status:ErrStatus::OK}
    }

    pub fn run_check(&mut self) -> ErrStatus
    {
        match ((self.check)(),&self.status){
            (false,ErrStatus::OK) =>{
                self.status = ErrStatus::ERR;
                (self.fail)();
                ErrStatus::ERR
            },
            (false,ErrStatus::ERR) =>{
                ErrStatus::ERR
            },
            (true,ErrStatus::ERR) =>{
                self.status = ErrStatus::OK;
                (self.restore)();
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

    use crate::ics_trait::generic_check::GenericCheck;
    use super::InternalCheck;

    static STR: &str= "internal_check_test";

    fn check_fun(var: &atomic::AtomicUsize) -> bool{
        var.load(atomic::Ordering::Relaxed) < 10
    }

    fn fail_fun(var: &atomic::AtomicUsize){
        var.store(99, atomic::Ordering::Relaxed)
    }

    fn rest_fun(var: &atomic::AtomicUsize){
        var.store(9, atomic::Ordering::Relaxed)
    }

    fn run_test(check_seq: &[(usize,usize)]){
        let check_var = core::sync::atomic::AtomicUsize::new(0);
        let str = STR.to_string();
        let check_f = || -> bool {check_fun(&check_var)};
        let fail_f= || -> () {fail_fun(&check_var)};
        let rest_f= || -> () {rest_fun(&check_var)};
        let mut int_check = InternalCheck::new(str, check_f,fail_f,rest_f);
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

    #[test]
    fn valid_description(){
        let v= atomic::AtomicUsize::new(1);
        let check_f = || -> bool {check_fun(&v)};
        let fail_f= || -> () {fail_fun(&v)};
        let rest_f= || -> () {rest_fun(&v)};
        let d = InternalCheck::new(STR.to_string(), check_f,fail_f,rest_f);

        assert_eq!(d.get_description(),STR);
    }
}
