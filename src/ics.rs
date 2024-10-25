use super::ics_trait::internal::*;
use super::ics_trait::generic_check::GenericCheck;
use super::ics_trait::external::ICSDep;
use super::ics_trait::ics_mex::ICSMex;
use alloc::vec::Vec;
use alloc::boxed::Box;

#[derive(Debug,Clone)]
pub enum ErrorType {
    INTERNAL,
    EXTERNAL,
}

#[allow(unused)]
#[derive(Debug,Clone)]
pub struct ICSError<'a>{
    e_type: ErrorType,
    e_desc: &'a str,
}

#[allow(unused)]
pub struct ICS<'a,FC,FF,FR,const S:usize>
where FC: FnMut() -> bool,
      FF: FnMut() -> (),
      FR: FnMut() -> (),
{
    int_vec: Vec<InternalCheck<'a,FC,FF,FR>>,
    ext_vec: Vec<ICSDep<'a,S>>,
    id: usize,
    ps: usize,
}

#[allow(unused)]
impl<'a,FC,FF,FR,const S: usize> ICS<'a,FC,FF,FR,S> 
where FC : FnMut() -> bool,
      FF : FnMut() -> (),
      FR : FnMut() -> (),
{
    pub fn new(id:usize, parts: usize) -> Self {
        Self {
            int_vec: Vec::new(),
            ext_vec: Vec::new(),
            id,
            ps: parts
        }
    }

    pub fn with_capacity(
        int_err_cap: usize, 
        ext_err_cap: usize, 
        error_cap: usize, 
        id:usize, 
        parts: usize) -> Self {
        let ie = Vec::with_capacity(int_err_cap);
        let ee = Vec::with_capacity(ext_err_cap);
        Self {int_vec: ie,ext_vec: ee, id, ps: parts}
    }

    pub fn full_spec(
        id:usize, parts: usize,
        int_vec: Vec<InternalCheck<'a,FC,FF,FR>>, ext_vec: Vec<ICSDep<'a,S>>) -> Self{
        Self{
            id,ps:parts, int_vec,ext_vec
        }
    }

    pub fn add_internal_check(&mut self, check: InternalCheck<'a,FC,FF,FR>){
        self.int_vec.push(check);
    }

    pub fn add_external_check(&mut self, check: ICSDep<'a,S>) -> usize{
        self.ext_vec.push(check);
        self.ext_vec.len() -1
    }

    pub fn internal_check(&mut self) {
        for int_check in &mut self.int_vec{
            int_check.run_check();
        }
    }


    pub fn check_specific_mex(&'a mut self,mex: &ICSMex<S>, ext_err_index: usize) -> Result<(),&str>{
        if ext_err_index >= self.ext_vec.len() {
            return Err("invalid index range fir ext_vec")
        }

        let ext_check = &mut self.ext_vec[ext_err_index];
        ext_check.check_mex(mex);
       Ok(())
    }

    pub fn get_err_info(&self,err_type: ErrorType, err_index: usize) -> Option<&str> {
        fn get_dscr<'a,G: GenericCheck<'a>>(vc : &'a Vec<G>, idx: usize) -> Option<&'a str>{
                if idx < vc.len(){
                    let err = &vc[idx];
                    Some(err.get_description())
                }else{
                    None
                }
        }
        match err_type {
            ErrorType::INTERNAL => get_dscr(&self.int_vec, err_index),
            ErrorType::EXTERNAL => get_dscr(&self.ext_vec, err_index),
        }
    }

    pub fn create_ics_messages(&self) -> Box<[ICSMex<S>]>{

        fn test_ch<'a,C: GenericCheck<'a>, const S:usize>
            (res: &mut Vec<ICSMex<S>>,cl: &Vec<C>,bit_s: &mut u8,mex_part: &mut usize,id:usize){
            for int_check in cl.iter(){
                if  usize::from(*bit_s) >= 8 * S {
                    res.push(ICSMex::new(id, *mex_part));
                    *mex_part+=1;
                    *bit_s =0;
                }

                match int_check.get_status(){
                    crate::ics_trait::generic_check::ErrStatus::ERR =>{
                        res[*mex_part].set_err(usize::from(*bit_s/8), *bit_s%8);
                    },
                    _ => (),
                };

                *bit_s+=1;
            }
        }

        let tot_bit = self.ext_vec.len() + self.int_vec.len();
        let tot_byte = tot_bit/8 + tot_bit % 2;
        let tot_array = tot_byte / S + tot_byte%2;
        let mut res = Vec::with_capacity(tot_array);
        let mut bit_s : u8= 0;
        let mut mex_part = 0;

        res.push(ICSMex::new(self.id, mex_part));
        test_ch(&mut res, &self.int_vec, &mut bit_s, &mut mex_part, self.id);
        test_ch(&mut res, &self.ext_vec, &mut bit_s, &mut mex_part, self.id);
        res.into_boxed_slice()
    }
}

#[allow(unused)]
#[cfg(test)]
mod test{
    use core::sync::atomic;
    use internal::InternalCheck;

    use crate::ics::ICS;
    use crate::ics_trait::*;

    const MEXSIZE : usize= 8;

    #[test]
    fn create_ics() {
        let mut ic : ICS<fn()-> bool,fn () -> (),fn () ->(), MEXSIZE> = ICS::new(12, 3);
        ic.internal_check();
        let err_mex_arr = ic.create_ics_messages();

        for mex in err_mex_arr.iter() {
            assert_eq!(mex.check_err(None),false);
        }
    }

    #[test]
    fn locate_internal_fail() {
        let mut var_to_check = atomic::AtomicI8::new(12);
        let mut ic : ICS<_, _, _, MEXSIZE> = ICS::new(12, 3);
        let ck = || var_to_check.load(atomic::Ordering::Relaxed) < 10;
        let ff = || var_to_check.store(-1, atomic::Ordering::Relaxed);
        let fr = || var_to_check.store(8,atomic::Ordering::Relaxed);
        ic.add_internal_check(InternalCheck::new("dummy check", ck, ff, fr));
        ic.internal_check();
        let res = ic.create_ics_messages();
        let mut mex_c = 0;
        for mex_p in res.iter(){
            mex_c+=1;
            assert_eq!(mex_p.check_err(Some(0)),true);
        }
        assert_eq!(mex_c,1);
    }
}
