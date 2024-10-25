use bit_ops::BitOps;

#[derive(Debug)]
pub struct ICSMex<const S: usize>
{
    id: usize,
    part: usize,
    err_vec: [u8;S],
}

impl<const S:usize> ICSMex<S>
{
    pub fn new(id: usize,part: usize) -> Self {
        Self{id,part,err_vec: [0;S]}
    }

    pub fn check_err(&self,err_idx: Option<usize>) -> bool{
        if let Some(err_idx) = err_idx{
            match err_idx < (S * 8){
                false => false,
                true =>{
                    let byte_idx = err_idx/S;
                    let bit_idx =err_idx %8;
                    let err_cell = usize::from(self.err_vec[byte_idx]);
                    err_cell.is_set(bit_idx)
                },
            }
        }else{
            for i in self.err_vec.iter(){
                if *i > 0 {
                    return true;
                };
            };
            false
        }
    }

    pub fn same_id_part(&self,id: usize, part: usize) -> bool {
        self.id == id && self.part == part
    }

    pub fn set_err(&mut self, idx:usize, bit: u8){
        if idx < 8 {
            self.err_vec[idx] = self.err_vec[idx].set_bit(bit);
        }
    }

    pub fn clear_err(&mut self, idx:usize, bit: u8){
        if idx < 8 {
            self.err_vec[idx] = self.err_vec[idx].clear_bit(bit);
        }
    }
}

#[cfg(test)]
mod test{
    use super::ICSMex;

    #[test]
    fn check_err_index() {
        let mut err : ICSMex<13> = ICSMex::new(12, 1);
        err.set_err(0, 0);
        assert_eq!(err.check_err(Some(0)),true);
    }

    #[test]
    fn check_err_all() {
        let mut err : ICSMex<13> = ICSMex::new(12, 1);
        err.set_err(0, 0);
        err.set_err(0, 1);
        assert_eq!(err.check_err(None),true);
    }

    #[test]
    fn check_err_all_no_one() {
        let mut err : ICSMex<13> = ICSMex::new(12, 1);
        err.set_err(0, 0);
        err.clear_err(0, 0);
        assert_eq!(err.check_err(None),false);
    }

    #[test]
    fn check_err_all_no_index() {
        let mut err : ICSMex<13> = ICSMex::new(12, 1);
        err.set_err(0, 0);
        err.clear_err(0, 0);
        assert_eq!(err.check_err(Some(0)),false);
    }

    #[test]
    fn check_new(){
        let t : ICSMex<2> = ICSMex::new(12, 0);

        let tl = t.err_vec.len();
        assert_eq!(t.check_err(None),false);
        assert_eq!(tl,2);
    }
}
