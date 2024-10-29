pub mod bst;
pub mod rbt;

pub trait ErrMap {
    fn insert_err<'a>(&mut self,err_num: usize) -> Result<(),(usize,&'a str)>;
    fn delete_err<'a>(&mut self,err_num: usize) -> Result<(),&'a str>;
    fn exist_err<'a>(&mut self,err_num: usize) -> bool;
    fn max(&self) -> usize;
    fn new() -> Self;
}
