use super::ics_trait::internal::*;
use super::ics_trait::generic_check::GenericCheck;
use super::ics_trait::external::ICSDep;
use super::ics_trait::ics_mex::ICSMex;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::result;

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
pub struct ICS<'a,const S:usize>{
    int_vec: Vec<InternalCheck<'a>>,
    ext_vec: Vec<ICSDep<'a,S>>,
    id: usize,
    ps: usize,
}

#[allow(unused)]
impl<'a,const S: usize> ICS<'a,S> {
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
        int_vec: Vec<InternalCheck<'a>>, ext_vec: Vec<ICSDep<'a,S>>) -> Self{
        Self{
            id,ps:parts, int_vec,ext_vec
        }
    }

    pub fn add_internal_check(&mut self, check: InternalCheck<'a>){
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

    pub fn check_general_mex(&mut self, mex: &ICSMex<S>){
        for cond in self.ext_vec.iter_mut() {
            cond.check_mex(mex);
        };
    }


    pub fn check_specific_mex(&mut self,mex: &ICSMex<S>, ext_err_index: usize) -> result::Result<(),&str>{
        if ext_err_index >= self.ext_vec.len() {
            return Err("invalid index range fir ext_vec")
        }

        let ext_check = &mut self.ext_vec[ext_err_index];
        ext_check.check_mex(mex);
        Ok(())
    }

    pub fn get_err_info(&'a self,err_type: ErrorType, err_index: usize) -> Option<&str> {
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
    use crate::debug_check::*;
use core::sync::atomic;
    use external::ICSDep;
    use ics_mex::ICSMex;
    use internal::InternalCheck;

    use crate::ics::ICS;
    use crate::ics_trait::*;
    use core::sync::atomic::AtomicI8;
    use core::sync::atomic::Ordering;

    const MEXSIZE : usize= 8;
    const STR: &str= "internal_check_test";

    #[test]
    fn create_ics() {
        let p = atomic::AtomicU8::new(18);
        let p_1 = atomic::AtomicU8::new(0);
        let mut cp : CheckU8<0, 20, 22, 10> = CheckU8::new(&p);
        let mut cp_1 : CheckU8<0, 10, 15, 0> = CheckU8::new(&p_1);
        let ic = InternalCheck::new(STR, &mut cp);
        let ic_1 = InternalCheck::new(STR, &mut cp_1);

        let mut ics : ICS<MEXSIZE>= ICS::new(1, 1);
        ics.add_internal_check(ic);
        ics.add_internal_check(ic_1);

        ics.internal_check();
        let res = ics.create_ics_messages();

        assert_eq!(ics.id,1);
        assert_eq!(ics.ps,1);
        for m in res{
            assert_eq!(m.check_err(None),false);
        }
    }

    #[test]
    fn locate_internal_fail() {
        todo!()
    }

    #[test]
    fn locate_external_fail_spec_mex() {
        todo!()
    }

    #[test]
    fn locate_external_fail_general_mex() {
        todo!()
    }
}
