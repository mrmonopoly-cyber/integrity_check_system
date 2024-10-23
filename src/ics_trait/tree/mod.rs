#[allow(unused)]
#[derive(Debug)]
struct Node<D>
where D: Eq{
    data: D,
    lchild: Option<usize>,
    rchild: Option<usize>,

}

#[allow(unused)]
#[derive(Debug)]
struct BTree<D> 
where D: Eq{
    nodes: Vec<Node<D>>, 
    root : usize,
}


#[allow(unused)]
impl<M> BTree<M>
where M:Eq{
}

#[cfg(test)]
mod test{
}
