use core::usize;

use crate::ics_trait::internal::InternalCheck;
use crate::{ics::ICS, ics_trait::external::ICSDep};
use crate::ics_trait::ics_mex::ICSMexFull;
use crate::err_map::ErrMap;
use crate::ics_trait::ics_mex::ICSMex;
use embedded_can::{Frame,Id,StandardId};

const ICS_PAYLOAD_SIZE : usize = 7;
type IcsPartType = u8;


pub type SendCanFun<F> = fn(&F) -> Result<(),()>;

#[derive(Debug)]
pub struct ICSCanBase<'a,M,F> 
where 
    M: ErrMap,
    F: Frame,
{
    ics: ICS<'a,M,ICS_PAYLOAD_SIZE,u8>,
    can_id: Id,
    send_f: SendCanFun<F>,
}

impl<'a,M: ErrMap,F:Frame> ICSCanBase<'a,M,F>
{
    pub fn new(ics_can_id: u16, ics_internal_id: u8, send_f: SendCanFun<F>) -> Self
    {
        let ics : ICS<'a,M,ICS_PAYLOAD_SIZE,u8> = ICS::new(ics_internal_id).ok().unwrap();
        let ics_can_id = Id::Standard(StandardId::new(ics_can_id).unwrap());

        Self{ics,can_id: ics_can_id,send_f}
    }

    pub fn check_mex_general(&mut self, mex: &F) -> Result<(),()>
    {
        let fc = |ics: &mut ICS<M,ICS_PAYLOAD_SIZE,u8>, ics_mex: &ICSMex<ICS_PAYLOAD_SIZE,u8,u8>|
        {
            ics.check_general_mex(ics_mex)
        };

        self.private_check_mex(mex, fc)
    }

    pub fn check_mex_specific_err(&mut self, mex: &F,err_index:&'a [usize]) 
        -> Result<(),()>
    {
        let fc = |ics: &mut ICS<M,ICS_PAYLOAD_SIZE,u8>, ics_mex: &ICSMex<ICS_PAYLOAD_SIZE,u8,u8>|
        {
            for i in err_index.iter(){
                let _ = ics.check_specific_mex(ics_mex,*i);
            }
        };
        self.private_check_mex(mex, fc)
    }

    pub fn add_dep(&mut self, dep : ICSDep<'a,ICS_PAYLOAD_SIZE,u8>, dep_index: usize) 
        -> Result<(), (usize, &str)> 
    {
        self.ics.add_external_check(dep, dep_index)
    }

    pub fn add_check(&mut self, check: InternalCheck<'a>,check_index :usize) 
        -> Result<(), (usize, &str)>
    {
        self.ics.add_internal_check(check, check_index)
    }

    pub fn run_check(&mut self)
    {
        self.ics.internal_check();
        let errs : ICSMexFull<ICS_PAYLOAD_SIZE, u8, IcsPartType> =
            self.ics.create_ics_messages();
        for ics_mex in errs.iter(){
            let mut buff = [0;8];
            buff[0] = ics_mex.id() << 4;
            buff[0] |= ics_mex.part() & 0x0F;
            let mut i = 1;
            for c in ics_mex.iter_data(){
                buff[i] = *c;
                i+=1;
            }
            let can_frame = F::new(self.can_id, &buff).unwrap();
            while (self.send_f)(&can_frame).is_err() {}
        }
    }
    
    //private

    fn private_check_mex<FUN>(&mut self, mex: &F, mut f:FUN) -> Result<(),()>
    where FUN: FnMut (&mut ICS<M,ICS_PAYLOAD_SIZE,u8>,&ICSMex<ICS_PAYLOAD_SIZE,u8,u8>) -> (),
    {
        if mex.id() == self.can_id{
            let mex_data = mex.data();
            let id_part = mex_data[0];
            let id = id_part & 0xF0;
            let part = id_part & 0x0F;
            let mut err_vec = [0;ICS_PAYLOAD_SIZE];
            for i in 0..ICS_PAYLOAD_SIZE{
                err_vec[i] = mex_data[i+1];
            }
            let ics_mex = ICSMex::new(id, part, err_vec);
            f(&mut self.ics,&ics_mex);
            Ok(())
        }else{
            Err(())
        }
    }
}

#[cfg(test)]
mod test{

}
