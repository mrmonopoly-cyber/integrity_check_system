use alloc::boxed::Box;

#[allow(unused)]
#[derive(Debug)]
struct FreqNode {
    freq: usize,
    node_id: usize,
    min_child: Option<Box<FreqNode>>,
    max_child: Option<Box<FreqNode>>,
}

#[allow(unused)]
impl FreqNode {
    fn new(freq: usize,node_id:usize) -> Self{
        Self{ freq, node_id, min_child: None, max_child: None }
    }

    fn insert(&mut self,(freq,node_id):(usize,usize)) -> Result<(),()>{
        fn check_child(se: &mut Option<Box<FreqNode>>,node_id:usize, freq: usize,) -> Result<(), ()>{
            match se{
                None => {
                    *se = Some(Box::new(FreqNode::new(freq, node_id)));
                    Ok(())
                },
                Some(n) =>{
                    n.insert((freq,node_id))
                },
            }
        }
        if node_id < self.node_id{
            check_child(&mut self.min_child, node_id, freq)
        }else if node_id > self.node_id{
            check_child(&mut self.max_child, node_id, freq)
        }else{
            Err(())
        }
    }

    pub fn min(&self) -> Option<(usize,usize)>{
        match &self.min_child{
            None => Some((self.node_id, self.freq)),
            Some(n) => n.min()
        }
    }
    
}

#[allow(unused)]
#[derive(Debug)]
pub struct FreqTree {
    root: Option<FreqNode>,
}

#[allow(unused)]
impl FreqTree {
    pub fn new() -> Self{
        Self{root:None}
    }

    pub fn min(&self) -> Option<(usize,usize)>{
        todo!();
    }

    pub fn delete_node(&mut self, node_id: usize) -> Result<(),()>{
        todo!();
    }
    
    pub fn insert_node(&mut self,node_id: usize,freq:usize) -> Result<(),()>{
        todo!();
    }
}
