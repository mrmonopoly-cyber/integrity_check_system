pub trait Integer:  Copy + PartialEq + PartialOrd + 
                    From<u8> + From<u16> + From<u32> + From<u64> + From<usize>  {}

#[derive(Debug)]
pub struct ICSMex<IS,PS,const S: usize>
where IS: Integer,
      PS: Integer,
{
    id: IS,
    part: PS,
    err_vec: [u8;S],
}

impl<IS,PS,const S:usize> ICSMex<IS,PS,S>
where IS: Integer,
      PS: Integer,
{
    pub fn new(id: IS,part: PS) -> Self {
        Self{id,part,err_vec: [0;S]}
    }

    pub fn check_err(&self,err_idx: Option<usize>) -> bool{
        if let Some(err_idx) = err_idx{
            match err_idx < (S * 8){
                false => false,
                true =>{
                    let byte_idx = err_idx/S;
                    let bit_idx = err_idx %8;
                    let err_cell = usize::from(self.err_vec[byte_idx]);
                    (err_cell & bit_idx) == 1
                },
            }
        }else{
            false
        }
    }

    pub fn same_id_part(&self,id: IS , part: PS) -> bool {
        self.id == id && self.part == part
    }

    pub fn set_err(&mut self, idx:usize, value: u8){
        if idx > 8 {}
        self.err_vec[idx] = value;
    }
}
