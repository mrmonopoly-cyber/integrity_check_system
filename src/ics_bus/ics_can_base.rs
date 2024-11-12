use core::usize;

use crate::ics::{ICSTemplate, ICS}; 
use crate::err_map::ErrMap;
use crate::ics_trait::external::ICSDep;
use crate::ics_trait::internal::InternalCheck;
use crate::ics_trait::ics_mex::ICSMex;
use crate::ics_trait::ics_mex::ICSMexFull;
use embedded_can::{Frame,Id};

const ICS_PAYLOAD_SIZE : usize = 7;

pub type SendCanFun<F> = fn(&F) -> Result<(),()>;

#[allow(unused)]
#[derive(Debug)]
pub struct ICSCanBase<'a,M,F> 
where 
    M: ErrMap,
    F: Frame,
{
    ics: ICS<'a,M,ICS_PAYLOAD_SIZE,u8>,
    can_id: Id,
    send_f: SendCanFun<F>,
}

#[allow(unused)]
impl<'a,M,F> ICSTemplate<'a,ICS_PAYLOAD_SIZE> for ICSCanBase<'a,M,F>
where
    M : ErrMap,
    Self: Sized,
    F: Frame,
{
    type TID = u8;
    type M = M;

    fn new(id:Self::TID) -> Result<Self,&'a str>{
        todo!()
    }

    fn with_capacity(int_err_cap: usize, ext_err_cap: usize, error_cap: usize, id:Self::TID) -> Self {
        todo!()
    }

    fn full_spec(id:Self::TID, int_vec: alloc::vec::Vec<(usize,InternalCheck<'a>)>, 
    ext_vec: alloc::vec::Vec<(usize,ICSDep<'a,ICS_PAYLOAD_SIZE,Self::TID>)>) -> Self {
        todo!()
    }

    fn add_internal_check(&mut self, check: InternalCheck<'a>, err_index: usize)
    -> Result<(), (usize, &str)> {
        todo!()
    }

    fn add_external_check(&mut self, check: ICSDep<'a,ICS_PAYLOAD_SIZE,Self::TID>, err_index: usize) 
    -> Result<(),(usize,&'a str)> {
        todo!()
    }

    fn internal_check(&mut self) {
        todo!()
    }

    fn check_general_mex<TPART>(&mut self, mex: &ICSMex<ICS_PAYLOAD_SIZE,Self::TID,TPART>)
    where TPART: Into<usize> + Copy + From<usize> {
        todo!()
    }

    fn check_specific_mex<TPART>(&mut self,mex: &ICSMex<ICS_PAYLOAD_SIZE,Self::TID,TPART>, ext_err_index: usize) 
    -> core::result::Result<(),&str>
    where TPART: Into<usize> + Copy + From<usize> {
        todo!()
    }

    fn get_err_info(&'a self,err_type: crate::ics::ErrorType, err_index: usize) -> Option<&str> {
        todo!()
    }

    fn create_ics_messages<TPART>(&mut self) -> ICSMexFull<ICS_PAYLOAD_SIZE,Self::TID,TPART>
    where TPART: Into<usize> + Copy + From<usize> {
        todo!()
    }

    fn get_id(&self) -> Self::TID {
        todo!()
    }
}

#[cfg(test)]
mod test{

}
