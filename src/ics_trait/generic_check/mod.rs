pub enum ErrStatus{
    OK,
    ERR,
}

pub type ErrFn= fn() -> ();

pub trait GenericCheck {
    fn get_description(&self) -> &String;
}
