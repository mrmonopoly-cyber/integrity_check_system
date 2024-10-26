use crate::err_map::ErrMap;
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
pub struct ICS<'a,M,const S:usize> where 
M : ErrMap{
    int_vec: Vec<InternalCheck<'a>>,
    ext_vec: Vec<ICSDep<'a,S>>,
    err_map: M,
    id: usize,
}

#[allow(unused)]
impl<'a,M,const S: usize> ICS<'a,M,S> where 
M : ErrMap{
    pub fn new(id:usize) -> Result<Self,&'a str> {
        Ok(Self {
            int_vec: Vec::new(),
            ext_vec: Vec::new(),
            err_map: M::new(),
            id,
        })
    }

    pub fn with_capacity(
        int_err_cap: usize, 
        ext_err_cap: usize, 
        error_cap: usize, 
        id:usize) -> Self {
        let ie = Vec::with_capacity(int_err_cap);
        let ee = Vec::with_capacity(ext_err_cap);
        Self {int_vec: ie,ext_vec: ee, err_map: M::new(),id}
    }

    pub fn full_spec(id:usize, int_vec: Vec<InternalCheck<'a>>, ext_vec: Vec<ICSDep<'a,S>>) -> Self{
        Self{
            id, int_vec,ext_vec,err_map: M::new(),
        }
    }

    pub fn add_internal_check(&mut self, check: InternalCheck<'a>, err_index: usize)-> Result<(), (usize, &str)>{
        match self.err_map.insert_err(err_index){
            Ok(_) => {
                self.int_vec.push(check);
                Ok(())
            },
            e => e
        }
    }

    pub fn add_external_check(&mut self, check: ICSDep<'a,S>, err_index: usize) -> Result<(),(usize,&'a str)>{
        match self.err_map.insert_err(err_index){
            Ok(_) => {
                self.ext_vec.push(check);
                Ok(())
            },
            e => e
        }
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
        let int_check_num = self.int_vec.len();
        let ext_check_num = self.ext_vec.len();
        let tot_check = int_check_num + ext_check_num;

        let mut res : Vec<ICSMex<S>> = Vec::with_capacity(tot_check/S);

        for i in 0..tot_check{
            let mex_part = i/(S*8);
            let i_buffer = i - mex_part * S;
            let err_pos = i_buffer/S;
            let bit_pos : u8 =  u8::try_from(err_pos%8).ok().unwrap();
            res[mex_part].set_err(err_pos, bit_pos);
        };

        res.into_boxed_slice()
    }
}

#[allow(unused)]
#[cfg(test)]
mod test{
    use crate::err_map::bst::Bst;
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

        let mut ics : ICS<Bst,MEXSIZE>= ICS::new(1).unwrap();
        ics.add_internal_check(ic,0);
        ics.add_internal_check(ic_1,1);

        ics.internal_check();
        let res = ics.create_ics_messages();

        assert_eq!(ics.id,1);
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
