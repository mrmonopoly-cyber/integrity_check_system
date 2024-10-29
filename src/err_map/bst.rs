use alloc::boxed::Box;
use super::ErrMap;

#[derive(Debug)]
pub struct Bst {
    key: Option<usize>,
    min_child: Option<Box<Bst>>,
    max_child: Option<Box<Bst>>,
}

#[allow(unused)]
impl ErrMap for Bst{
    fn insert_err<'a>(&mut self,err_num: usize) -> Result<(),(usize,&'a str)> 
    {
        fn test_node<'a>(n: &mut Option<Box<Bst>>, key: usize) -> Result<(),(usize,&'a str)>
        {
            match n{
                None => {
                    *n = Some(Box::new(Bst{key:Some(key), max_child:None,min_child:None}));
                    Ok(())
                },
                Some(n) => {
                    n.insert_err(key)
                }
            }
        }

        match &self.key{
            None => {
                self.key = Some(err_num);
                Ok(())
            }
            Some(n) =>{
                if *n < err_num {
                    test_node(& mut self.min_child, err_num)
                }else if *n > err_num{
                    test_node(& mut self.max_child, err_num)
                }else{
                    Err((err_num,"error index already taken"))
                }
            },
        }
    }

    fn delete_err<'a>(&mut self,err_num: usize) -> Result<(),&'a str> 
    {
        todo!()
    }

    fn exist_err<'a>(&mut self,err_num: usize) -> bool 
    {
        match self.key{
            None => false,
            Some(k) =>{
                if k == err_num {
                    true
                }else if k > err_num{
                    match & mut self.max_child{
                        None => false,
                        Some(n) => n.exist_err(err_num),
                    }
                }else{
                    match & mut self.min_child{
                        None => false,
                        Some(n) => n.exist_err(err_num),
                    }
                }
            },
        }
    }

    fn new() -> Self 
    {
        Self{key: None,max_child: None,min_child: None}
    }

    fn max(&self) -> usize {
        match (self.key,&self.max_child){
            (None,_) => 0,
            (Some(i),None) => i,
            (_,Some(n)) => {
                n.max()
            },
        }
    }
}

#[cfg(test)]
mod test{
    use crate::err_map::ErrMap;

    use super::Bst;

    #[test]
    fn insertion_bst() {
        let mut bst = Bst::new();
        let elements = [5,3,4,1,2,0];
        for e in elements.iter(){
            let r = bst.insert_err(*e);
            assert_eq!(r,Ok(()));
        }

        for e in elements.iter(){
            let r = bst.exist_err(*e);
            assert_eq!(r,true);
        }
        let r = bst.exist_err(12);
        assert_eq!(r,false);
    }
}
