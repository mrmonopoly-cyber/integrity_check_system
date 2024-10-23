#[allow(unused)]
#[derive(Debug)]
pub struct  SquareGraphMatx {
    matrx: Vec<usize>,
    size: usize,
}

#[allow(unused)]
impl SquareGraphMatx {
    pub fn new() -> Self{
        Self{matrx: Vec::new(), size: 0}
    }
    pub fn with_capacity(size: usize) -> Self{
        let mut num_nodes =0;
        for i in 0..size{num_nodes= num_nodes+ i};
        SquareGraphMatx { matrx: Vec::with_capacity(num_nodes), size }
    }

    pub fn add_node(&mut self) -> usize{
        let extra_cells = (self.size+1) - (self.size);
        self.size+=1;
        for _ in 0..extra_cells{
            self.matrx.push(0);
        }
        self.size-1
    }

    pub fn add_link(&mut self, start_x: usize,start_y: usize, dest_x: usize,dest_y: usize){
        let m_size = self.matrx.len();
    }

    pub fn node_exist(&self, node: usize) -> bool{
        self.get_node_pos(node)< self.matrx.len()
    }

    //private

    fn get_node_pos(&self, node_i: usize) -> usize{
        let mut node_row =0;
        for i in 0..(node_i-1){node_row= node_row+ i};
        node_row
    }
}
