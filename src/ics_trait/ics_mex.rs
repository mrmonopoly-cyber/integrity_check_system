use alloc::vec::Vec;
use bit_ops::BitOps;
use core::result::Result;
use core::slice::Iter;
use num::{integer::Integer, Unsigned};

#[derive(Debug)]
pub struct ICSMex<const S: usize,TID,TPART>where 
TID: Integer,
TPART: Copy + Unsigned +  Into<usize> + TryFrom<usize>
{
    id: TID,
    part: TPART,
    err_vec: [u8;S],
}

impl<const S:usize,TID,TPART> ICSMex<S,TID,TPART> where  
TID: Integer + Copy,
TPART: Copy + Unsigned +  Into<usize> + TryFrom<usize>
{
    pub fn new(id:TID,part:TPART, err_vec: [u8;S]) -> Self{
        Self { id,part, err_vec }
    }

    pub fn id(&self) -> TID{
        self.id
    }

    pub fn part(&self) -> TPART{
        self.part
    }

    pub fn iter_data(&self) -> Iter<u8> {
        self.err_vec.iter()
    }

    pub fn check_error(&self, err_index: Option<usize>) -> bool{
        match err_index{
            None => {
                for e in &self.err_vec{
                    if *e > 0{
                        return true;
                    }
                }
                false
            },
            Some(i) =>{
                let clean_index = {
                    let other_pack_buff = self.part.into() * S * 8;
                    if i > other_pack_buff {
                        i - other_pack_buff
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

    pub fn same_id(&self,id:TID) -> bool {
        self.id == id
    }

    pub fn set_err(&mut self,cell_idx: usize, bit_id: u8) -> Result<(),()>{
        if  cell_idx < S && bit_id < 8{
            let c = &mut self.err_vec[cell_idx];
            *c = c.set_bit(bit_id);
            return Ok(())
        }
        Err(())
    }

    pub fn clear_err(&mut self,cell_idx: usize, bit_id: u8) -> Result<(),()>{
        if  cell_idx < S && bit_id < 8{
            let c = &mut self.err_vec[cell_idx];
            *c = c.clear_bit(bit_id);
            return Ok(())
        }
        Err(())
    }
    
}


#[allow(unused)]
#[derive(Debug)]
pub struct ICSMexFull<const S: usize,TID,TPART> where 
TID: Integer,
TPART: Copy + Unsigned +  Into<usize> + TryFrom<usize>
{
    parts: Vec<ICSMex<S,TID,TPART>>,
}

#[allow(unused)]
impl<const S:usize,TID,TPART> ICSMexFull<S,TID,TPART>where 
TID: Integer + Copy,
TPART: Copy + Unsigned +  Into<usize> + TryFrom<usize>
{
    pub fn new(id: TID,err_num: usize) -> Self {
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
                res.push(ICSMex{id, part: TPART::try_from(i).ok().unwrap(),err_vec: [0;S]});
            }
            res
        };
        Self{parts}
    }

    pub fn iter(&self) -> core::slice::Iter<ICSMex<S,TID,TPART>>{
        self.parts.iter()
    }

    pub fn get_part(&mut self, part: usize) -> Result<&mut ICSMex<S,TID,TPART>,(usize,&str)>{
        if part < self.parts.len(){
            return Ok(&mut self.parts[part])
        }
        Err((part,"invalid part index"))
    }

    pub fn set_err(&mut self, err_idx: usize) -> Result<(),(&str,usize)>{
        fn up_f<const S: usize,TID,TPART>(se: &mut ICSMex<S,TID,TPART>,reg_idx: usize,bit_index: u8)
        where
            TID: Integer + Copy,
            TPART: Copy + Unsigned +  Into<usize> + TryFrom<usize>,
        {
            se.set_err(reg_idx, bit_index);
        }
        self.err_prop_set(err_idx, up_f)
    }

    pub fn clear_err(&mut self, err_idx:usize) -> Result<(),(&str,usize)> {
        fn up_f<const S: usize,TID,TPART>(se: &mut ICSMex<S,TID,TPART>,reg_idx: usize,bit_index: u8)
        where
            TID: Integer + Copy,
            TPART: Copy + Unsigned +  Into<usize> + TryFrom<usize>,
        {
            se.clear_err(reg_idx, bit_index);
        }
        self.err_prop_set(err_idx, up_f)
    }

    //private
    fn err_prop_set<F>(&mut self,err_idx: usize,update_f : F) -> Result<(), (&str, usize)> 
        where F: Fn(&mut ICSMex<S,TID,TPART>,usize,u8) -> (),{
        let err_in_one_packet = S*8;
        let num_parts = self.parts.len();
        if  err_idx >= err_in_one_packet * num_parts {
           return  Err(("out of bounds index", err_idx))
        }
        let err_part = err_idx / err_in_one_packet;
        let clear_err_index = err_idx - (err_part * err_in_one_packet);
        let cell_index = clear_err_index / 8;
        let bit_index = u8::try_from(clear_err_index % 8).unwrap();
        update_f(&mut self.parts[err_part],cell_index,bit_index);
        Ok(())
    }
}

#[cfg(test)]
mod test{
    use core::usize;

    use super::ICSMexFull;

    #[test]
    fn test_set_bit() {
        let mut err : ICSMexFull<13,usize,usize> = ICSMexFull::new(12, 1);
        err.set_err(0).ok();
        let p = err.get_part(0).ok().unwrap();
        assert_eq!(p.check_error(Some(0)),true);
    }

    #[test]
    fn check_err_index() {
        let mut err : ICSMexFull<2,usize,usize> = ICSMexFull::new(12, 1);
        err.set_err(0).ok();
        let p = err.get_part(0).ok().unwrap();
        assert_eq!(p.check_error(Some(0)),true);
    }

    #[test]
    fn check_err_all() {
        let mut err : ICSMexFull<13,usize,usize> = ICSMexFull::new(12, 1);
        err.set_err(0).ok();
        err.set_err(1).ok();
        let p = err.get_part(0).ok().unwrap();
        assert_eq!(p.check_error(None),true);
    }

    #[test]
    fn check_err_all_no_one() {
        let mut err : ICSMexFull<13,usize,usize> = ICSMexFull::new(12, 1);
        err.set_err(0).ok();
        let p = err.get_part(0).ok().unwrap();
        assert_eq!(p.check_error(None),true);
    }

    #[test]
    fn check_err_all_no_index() {
        let mut err : ICSMexFull<13,usize,usize> = ICSMexFull::new(12, 1);
        err.set_err(5).ok();
        let p = err.get_part(0).ok().unwrap();
        assert_eq!(p.check_error(Some(0)),false);
    }

    #[test]
    fn check_new_multiple_parts(){
        let t : ICSMexFull<2,usize,usize> = ICSMexFull::new(12, 33);
        let mut i = 0;
        for p in t.parts.iter(){
            i+=1;
            assert_eq!(p.check_error(None),false);
        }
        assert_eq!(i,3);
    }

    #[test]
    fn check_new_one_part(){
        let t : ICSMexFull<1,usize,u8> = ICSMexFull::new(12,0);
        let mut i = 0;
        for p in t.parts.iter(){
            i+=1;
            assert_eq!(p.check_error(None),false);
        }
        assert_eq!(i,1);
    }
}
