mod graph;

#[allow(unused)]
pub type Tree = graph::SquareGraphMatx;

#[allow(unused)]
impl Tree{
    pub fn add_child(&mut self, parent_node: usize) {
        if self.node_exist(parent_node) {
            let node_i = self.add_node();
        }
    }
}


#[cfg(test)]
mod test{
}
