use core::fmt::Debug;

#[derive(Debug,PartialEq,Clone)]
pub enum ErrStatus{
    OK,
    ERR,
}

#[allow(unused)]
pub trait GenericCheck<'a> {
    fn get_description(&'a self) -> &'a str;
    fn get_status(&self) -> ErrStatus;
}

pub trait MexConseguence: Debug{
    fn manage_fail(&mut self) -> ();
    fn restore_fail(&mut self) -> ();
}

pub trait ObjectCheck : MexConseguence{
    fn check(&self) -> bool;
}


