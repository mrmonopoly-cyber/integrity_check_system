#[derive(Debug,PartialEq,Clone)]
pub enum ErrStatus{
    OK,
    ERR,
}

pub type ErrFn= fn() -> ();

pub trait GenericCheck<'a> {
    fn get_description(&'a self) -> &'a str;
    fn get_status(&self) -> ErrStatus;
}
