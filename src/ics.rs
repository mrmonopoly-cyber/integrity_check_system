use core::result;
use alloc::vec::Vec;
use crate::err_map::bst::Bst;
use crate::err_map::ErrMap;
use crate::ics_trait::generic_check::ErrStatus;
use crate::ics_trait::ics_mex::ICSMexFull;
use super::ics_trait::internal::*;
use super::ics_trait::generic_check::GenericCheck;
use super::ics_trait::external::ICSDep;
use super::ics_trait::ics_mex::ICSMex;

pub trait ICSTemplate<'a,const S:usize>
where 
    Self: Sized
{
    type TID : Copy + PartialEq;
    type M : ErrMap;

    fn new(id:Self::TID) -> Result<Self,&'a str>;

    fn with_capacity(int_err_cap: usize, ext_err_cap: usize, error_cap: usize, id:Self::TID) -> Self;

    fn full_spec(id:Self::TID, int_vec: Vec<(usize,InternalCheck<'a>)>, 
    ext_vec: Vec<(usize,ICSDep<'a,S,Self::TID>)>) -> Self;

    fn add_internal_check(&mut self, check: InternalCheck<'a>, err_index: usize)
    -> Result<(), (usize, &str)>;

    fn add_external_check(&mut self, check: ICSDep<'a,S,Self::TID>, err_index: usize) 
    -> Result<(),(usize,&'a str)>;

    fn internal_check(&mut self);

    fn check_general_mex<TPART>(&mut self, mex: &ICSMex<S,Self::TID,TPART>)
    where TPART: Into<usize> + Copy + From<usize>;

    fn check_specific_mex<TPART>(&mut self,mex: &ICSMex<S,Self::TID,TPART>, ext_err_index: usize) 
    -> result::Result<(),&str>
    where TPART: Into<usize> + Copy + From<usize>;

    fn get_err_info(&'a self,err_type: ErrorType, err_index: usize) -> Option<&str>;

    fn create_ics_messages<TPART>(&mut self) -> ICSMexFull<S,Self::TID,TPART>
    where TPART: Into<usize> + Copy + From<usize>;
}


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
#[derive(Debug)]
pub struct ICS<'a,M,const S:usize,TID> 
where 
    M : ErrMap,
    TID: Copy + core::cmp::PartialEq,
{
    int_vec: Vec<(usize,InternalCheck<'a>)>,
    ext_vec: Vec<(usize,ICSDep<'a,S,TID>)>,
    err_map: M,
    id: TID,
}

#[allow(unused)]
impl<'a,M,const S: usize,TID> ICSTemplate<'a,S> for ICS<'a,M,S ,TID>
    where 
        M : ErrMap,
        TID: Copy + core::cmp::PartialEq,
{
    type TID = TID;
    type M = Bst;
    // add code here
    fn new(id:TID) -> Result<Self,&'a str> {
        Ok(Self {
            int_vec: Vec::new(),
            ext_vec: Vec::new(),
            err_map: M::new(),
            id,
        })
    }

    fn with_capacity(
        int_err_cap: usize, 
        ext_err_cap: usize, 
        error_cap: usize, 
        id:TID) -> Self 
    {
        let ie = Vec::with_capacity(int_err_cap);
        let ee = Vec::with_capacity(ext_err_cap);
        Self {int_vec: ie,ext_vec: ee, err_map: M::new(),id}
    }

    fn full_spec(id:TID, int_vec: Vec<(usize,InternalCheck<'a>)>, 
        ext_vec: Vec<(usize,ICSDep<'a,S,TID>)>) -> Self
    {
        Self{
            id, int_vec,ext_vec,err_map: M::new(),
        }
    }

    fn add_internal_check(&mut self, check: InternalCheck<'a>, err_index: usize)
        -> Result<(), (usize, &str)>
    {
        match self.err_map.insert_err(err_index){
            Ok(_) => {
                self.int_vec.push((err_index,check));
                Ok(())
            },
            e => e
        }
    }

    fn add_external_check(&mut self, check: ICSDep<'a,S,TID>, err_index: usize) 
        -> Result<(),(usize,&'a str)>
    {
        match self.err_map.insert_err(err_index){
            Ok(_) => {
                self.ext_vec.push((err_index,check));
                Ok(())
            },
            e => e
        }
    }

    fn internal_check(&mut self) 
    {
        for (_,int_check) in &mut self.int_vec{
            int_check.run_check();
        }
    }

    fn check_general_mex<TPART>(&mut self, mex: &ICSMex<S,TID,TPART>)
    where 
        TPART: Copy +  Into<usize> + TryFrom<usize>
    {
        for (_,cond) in self.ext_vec.iter_mut() {
            cond.check_mex(mex);
        };
    }


    fn check_specific_mex<TPART>(&mut self,mex: &ICSMex<S,TID,TPART>, ext_err_index: usize) -> result::Result<(),&str>
    where 
        TPART: Copy +  Into<usize> + TryFrom<usize>
    {
        if ext_err_index >= self.ext_vec.len() {
            return Err("invalid index range fir ext_vec")
        }

        let (_,ext_check) = &mut self.ext_vec[ext_err_index];
        ext_check.check_mex(mex);
        Ok(())
    }

    fn get_err_info(&'a self,err_type: ErrorType, err_index: usize) -> Option<&str> 
    {
        fn get_dscr<'a,G: GenericCheck<'a>>(vc : &'a Vec<(usize,G)>, idx: usize) -> Option<&'a str>{
                if idx < vc.len(){
                    let (_,err) = &vc[idx];
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

    fn create_ics_messages<TPART>(&mut self) -> ICSMexFull<S,TID,TPART>
    where 
        TPART: Copy +  Into<usize> + TryFrom<usize>
    {
        let err_num = self.err_map.max() + 1;
        let mut r : ICSMexFull<S, TID, TPART> = ICSMexFull::new(self.id, err_num);
        for (err_index,int_err) in self.int_vec.iter_mut(){
            if int_err.run_check() == ErrStatus::ERR{
                r.set_err(*err_index);
            }
        }

        for (err_index,ext_err) in self.ext_vec.iter_mut(){
            if ext_err.get_status() == ErrStatus::ERR{
                r.set_err(*err_index);
            }
        }

        r
    }
}

#[allow(unused)]
#[cfg(test)]
mod test{
    use crate::err_map::bst::Bst;
    use crate::debug_check::*;
    use crate::ics::ICSTemplate;
    use core::sync::atomic;
    use core::sync::atomic::AtomicU8;
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
        let mut ics : ICS<Bst,MEXSIZE,usize>= ICS::new(1).unwrap();
        ics.internal_check();
        let res : ics_mex::ICSMexFull<8, usize, usize> = ics.create_ics_messages();

        assert_eq!(ics.id,1);
        for m in res.iter(){
            assert_eq!(m.check_error(None),false);
        }
    }

    #[test]
    fn locate_internal_fail() {
        let mut ics : ICS<Bst, MEXSIZE,usize> = ICS::new(12).unwrap();

        let at_u8 = AtomicU8::new(12);
        let mut che_u8 :CheckU8<0, 15, 99, 0> = CheckU8::new(&at_u8);
        let it_ch = InternalCheck::new(STR, &mut che_u8);

        ics.add_internal_check(it_ch, 1);
        at_u8.store(101, Ordering::Relaxed);
        ics.internal_check();
        let mex : ics_mex::ICSMexFull<8, usize, usize> = ics.create_ics_messages();
        let mut i = 0;
        for m in mex.iter(){
            i+=1;
            assert_eq!(m.check_error(None),true);
        }
        assert_eq!(i,1);
    }

    #[test]
    fn locate_external_fail_spec_mex() {
        let mut ics : ICS<Bst, MEXSIZE,usize> = ICS::new(12).unwrap();

        let at_u8 = AtomicU8::new(12);
        let mut che_u8 :CheckU8<0, 15, 99, 0> = CheckU8::new(&at_u8);
        let it_ch = InternalCheck::new(STR, &mut che_u8);

        ics.add_internal_check(it_ch, 1);
        at_u8.store(101, Ordering::Relaxed);
        ics.internal_check();
        let mex : ics_mex::ICSMexFull<8, usize, usize> = ics.create_ics_messages();
        let mut i = 0;
        for m in mex.iter(){
            i+=1;
            assert_eq!(m.check_error( Some(1) ),true);
        }
        assert_eq!(i,1);
    }

    #[test]
    fn locate_external_fail_general_mex() {
        let mut ics : ICS<Bst, MEXSIZE,usize> = ICS::new(12).unwrap();

        let at_u8 = AtomicU8::new(12);
        let mut che_u8 :CheckU8<0, 15, 99, 0> = CheckU8::new(&at_u8);
        let it_ch = InternalCheck::new(STR, &mut che_u8);

        ics.add_internal_check(it_ch, 1);
        at_u8.store(101, Ordering::Relaxed);
        ics.internal_check();
        let mex : ics_mex::ICSMexFull<8, usize, usize> = ics.create_ics_messages();
        let mut i = 0;
        for m in mex.iter(){
            i+=1;
            assert_eq!(m.check_error( None ),true);
        }
        assert_eq!(i,1);
    }
}
