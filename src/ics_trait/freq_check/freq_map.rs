use alloc::{boxed::Box, vec::Vec};
use embedded_timers::{clock::Clock, instant::Instant};

#[allow(unused)]
#[derive(Debug)]
struct FreqNode<I> 
where 
    I: Instant,
{
    timeline: I,
    node_id: usize,
    min_child: Option<Box<FreqNode<I>>>,
    max_child: Option<Box<FreqNode<I>>>,
}

#[allow(unused)]
impl<I> FreqNode<I>
where 
    I: Instant,
{
    fn new(timeline: I,node_id:usize) -> Self{
        Self{ timeline, node_id, min_child: None, max_child: None}
    }

    fn insert(& mut self,(timeline,node_id):(I,usize)) -> Result<(),()>{
        fn check_child<I>(
            se: &mut Option<Box<FreqNode<I>>>,
            node_id:usize, 
            timeline: I,)-> Result<(), ()>
        where
            I: Instant,
        {
            match se{
                None => {
                    *se = Some(Box::new(FreqNode::new(timeline, node_id)));
                    Ok(())
                },
                Some(n) =>{
                    n.insert((timeline,node_id))
                },
            }
        }
        if timeline < self.timeline{
            check_child(&mut self.min_child, node_id, timeline)
        }else if timeline > self.timeline{
            check_child(&mut self.max_child, node_id, timeline)
        }else{
            Err(())
        }
    }

    fn delete(&mut self,timeline:I){
        fn check_child<I>(se: &mut Option<Box<FreqNode<I>>>, timeline: I,)
        where
            I: Instant,
        {
            match se{
                None => (),
                Some(n) =>{
                    n.delete(timeline)
                },
            }
        }
        if timeline > self.timeline{
            check_child(&mut self.max_child, timeline)
        }else if timeline < self.timeline{
            check_child(&mut self.min_child, timeline)
        }else{
            todo!()
        }
    }

    pub fn min(&self) -> Option<(I,usize)>{
        match &self.min_child{
            None => Some((self.timeline,self.node_id)),
            Some(n) => n.min()
        }
    }
    
}

#[allow(unused)]
#[derive(Debug)]
pub struct FreqTree<I,FClk>
where 
    I:Instant,
    FClk: Clock<Instant = I>,
{
    root: Option<FreqNode<I>>,
    clk: FClk
}

#[allow(unused)]
impl<I,FClk> FreqTree<I,FClk>
where 
    I:Instant,
    FClk: Clock<Instant = I>,
{
    pub fn new(clk: FClk) -> Self{
        Self{root:None,clk}
    }

    pub fn min(&self) -> Option<(I,usize)>{
        match &self.root{
            None => None,
            Some(n) => n.min()
        }
    }

    pub fn delete_node(&mut self, timeline: I){
        todo!()
    }
    
    pub fn insert_node(&mut self,node_id: usize,timeline:I) -> Result<(),()>{
        match &mut self.root{
            None => {
                self.root = Some(FreqNode::new(timeline, node_id));
                Ok(())
            },
            Some(n) =>n.insert((timeline,node_id)),
        }
    }

    pub fn chek_freq<FClock>(&mut self,) -> Vec<usize>
    {
        let mut res = Vec::new();

        loop {
            match self.min(){
                None => break,
                Some((timeline,id)) => {
                    if self.clk.now() < timeline{
                        self.delete_node(timeline);
                        res.push(id)
                    }
                },
            }
        }

        res
    }
}
