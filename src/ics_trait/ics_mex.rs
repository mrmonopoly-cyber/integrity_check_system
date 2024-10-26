use bit_ops::BitOps;
use core::result::Result;

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
                    let byte_idx = {
                        if S > 1{
                            err_idx/S
                        }else{
                            0
                        }
                    };
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

    pub fn set_err(&mut self, idx:usize, bit: u8) -> Result<(),&str>{
        let upv = |v: &mut[u8;S]| {v[idx] = v[idx].set_bit(bit)};
        self.update_err_vec(idx, bit, upv)
    }

    pub fn clear_err(&mut self, idx:usize, bit: u8) -> Result<(),&str> {
        let upv = |v: &mut[u8;S]| {v[idx] = v[idx].clear_bit(bit)};
        self.update_err_vec(idx, bit, upv)
    }

    //private
    fn update_err_vec<F: FnMut(&mut[u8;S]) -> ()>(&mut self, idx: usize, bit: u8,mut upf :F )
        -> Result<(),&str>{
        if idx < S && bit < 8 {
            upf(&mut self.err_vec);
            Ok(())
        }else{
            Err("invalid idx or bit")
        }
    }
}

#[cfg(test)]
mod test{
    use super::ICSMex;

    #[test]
    fn test_set_bit() {
        let mut err : ICSMex<13> = ICSMex::new(12, 1);
        let r = err.set_err(0, 6);
        assert_eq!(r,Ok(()));
        assert_eq!(err.check_err(Some(6)),true);
    }

    #[test]
    fn check_err_index() {
        let mut err : ICSMex<13> = ICSMex::new(12, 1);
        let _ = err.set_err(0, 0);
        assert_eq!(err.check_err(Some(0)),true);
    }

    #[test]
    fn check_err_all() {
        let mut err : ICSMex<13> = ICSMex::new(12, 1);
        let _ = err.set_err(0, 0);
        let _ = err.set_err(0, 1);
        assert_eq!(err.check_err(None),true);
    }

    #[test]
    fn check_err_all_no_one() {
        let mut err : ICSMex<13> = ICSMex::new(12, 1);
        let _ = err.set_err(0, 0);
        let _ = err.clear_err(0, 0);
        assert_eq!(err.check_err(None),false);
    }

    #[test]
    fn check_err_all_no_index() {
        let mut err : ICSMex<13> = ICSMex::new(12, 1);
        let _ = err.set_err(0, 0);
        let _ = err.clear_err(0, 0);
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
