use bit_ops::BitOps;
use core::result::Result;

#[derive(Debug)]
pub struct ICSMex<const S: usize> {
    num_errors: usize,
    id: usize,
    part: usize,
    err_vec: [u8;S],
}

impl<const S:usize> ICSMex<S> {
    pub fn check_error(&self, err_index: Option<usize>) -> bool{
        match err_index{
            None => self.num_errors > 0,
            Some(i) =>{
                let clean_index = {
                    if i > S*8 {
                        i - (S*8)
                    }else{
                        0
                    }
                };
                if clean_index < S*8{
                    let num_idx = clean_index/8;
                    let bit_index = u8::try_from(clean_index%8).unwrap();
                    return self.err_vec[num_idx].is_set(bit_index)
                }
                false
            },
        }
    }

    pub fn same_id_part(&self,id:usize,part:usize) -> bool {
        self.id == id && self.part == part
    }
    
}


#[allow(unused)]
#[derive(Debug)]
pub struct ICSMexFull<const S: usize>
{
    parts: Vec<ICSMex<S>>,
}

#[allow(unused)]
impl<const S:usize> ICSMexFull<S>
{
    pub fn new(id: usize,err_num: usize) -> Self {
        let err_in_one_packet = S*8;
        let integer_part = err_num/err_in_one_packet;
        let aux_part = {
            match err_num%(8){
                0 => 0,
                _ => 1,
            }
        };
        let num_packets = {
            match integer_part + aux_part{
                0 => 1,
                i => i,
            }
        };
        let parts = {
            let mut res = Vec::with_capacity(num_packets);
            for i in 0..num_packets{
                res.push(ICSMex{id,num_errors: 0, part: i,err_vec: [0;S]});
            }
            res
        };
        Self{parts}
    }

    pub fn get_part(&self, part: usize) -> Result<&ICSMex<S>,(usize,&str)>{
        if part < self.parts.len(){
            return Ok(&self.parts[part])
        }
        Err((part,"invalid part index"))
    }

    pub fn set_err(&mut self, err_idx: usize) -> Result<(),(&str,usize)>{
        
        fn up_f<const S: usize>(se: &mut ICSMexFull<S>, err_part: usize,cell_index: usize,bit_index: u8){
            se.parts[err_part].err_vec[cell_index].set_bit(bit_index);
            se.parts[err_part].num_errors+= 1;
        }
        self.err_prop_set(err_idx, up_f)
    }

    pub fn clear_err(&mut self, err_idx:usize) -> Result<(),(&str,usize)> {
        fn up_f<const S: usize>(se: &mut ICSMexFull<S>, err_part: usize,cell_index: usize,bit_index: u8){
            se.parts[err_part].err_vec[cell_index].clear_bit(bit_index);
            se.parts[err_part].num_errors-=1; 
        }
        self.err_prop_set(err_idx, up_f)
    }

    //private
    fn err_prop_set<F>(&mut self,err_idx: usize,update_f : F) -> Result<(), (&str, usize)> 
        where F: Fn(& mut ICSMexFull<S>, usize,usize,u8) -> (),{
        let err_in_one_packet = S*8;
        let num_parts = self.parts.len();
        if err_in_one_packet * num_parts <= err_idx {
           return  Err(("out of bounds index", err_idx))
        }
        let err_part = err_idx / err_in_one_packet;
        let clear_err_index = err_idx - (err_part * err_in_one_packet);
        let cell_index = clear_err_index / 8;
        let bit_index = u8::try_from(clear_err_index % 8).unwrap();
        update_f(self, err_part,cell_index,bit_index);
        Ok(())
    }
}

#[cfg(test)]
mod test{
    use super::ICSMexFull;

    #[test]
    fn test_set_bit() {
        let mut err : ICSMexFull<13> = ICSMexFull::new(12, 1);
        err.set_err(0).ok();
        let p = err.get_part(0).ok().unwrap();
        assert_eq!(p.err_vec,[0;13]);
        assert_eq!(p.check_error(Some(0)),true);
    }

    #[test]
    fn check_err_index() {
        let mut err : ICSMexFull<13> = ICSMexFull::new(12, 1);
        err.set_err(0).ok();
        let p = err.get_part(0).ok().unwrap();
        assert_eq!(p.check_error(Some(0)),true);
    }

    #[test]
    fn check_err_all() {
        let mut err : ICSMexFull<13> = ICSMexFull::new(12, 1);
        err.set_err(0).ok();
        err.set_err(1).ok();
        let p = err.get_part(0).ok().unwrap();
        assert_eq!(p.check_error(None),true);
    }

    #[test]
    fn check_err_all_no_one() {
        let mut err : ICSMexFull<13> = ICSMexFull::new(12, 1);
        err.set_err(0).ok();
        let p = err.get_part(0).ok().unwrap();
        assert_eq!(p.check_error(None),true);
    }

    #[test]
    fn check_err_all_no_index() {
        let mut err : ICSMexFull<13> = ICSMexFull::new(12, 1);
        err.set_err(5).ok();
        let p = err.get_part(0).ok().unwrap();
        assert_eq!(p.check_error(Some(0)),false);
    }

    #[test]
    fn check_new_multiple_parts(){
        let t : ICSMexFull<1> = ICSMexFull::new(12, 9);
        let mut i = 0;
        for p in t.parts.iter(){
            i+=1;
            assert_eq!(p.check_error(None),false);
        }
        assert_eq!(i,2);
    }

    #[test]
    fn check_new_one_part(){
        let t : ICSMexFull<1> = ICSMexFull::new(12,0);
        let mut i = 0;
        for p in t.parts.iter(){
            i+=1;
            assert_eq!(p.check_error(None),false);
        }
        assert_eq!(i,1);
    }
}
