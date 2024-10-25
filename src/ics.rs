use super::ics_trait::generic_check::ErrStatus;
use super::ics_trait::internal::*;
use super::ics_trait::generic_check::GenericCheck;
use super::ics_trait::external::ICSDep;
use super::ics_trait::ics_mex::ICSMex;

#[derive(Debug,Clone)]
pub enum ErrorType {
    INTERNAL,
    EXTERNAL,
}

#[allow(unused)]
#[derive(Debug,Clone)]
pub struct ICSError<'a>{
    e_type: ErrorType,
    e_desc: &'a str,
}

#[allow(unused)]
pub struct ICS<'a,FC,FF,FR,const S:usize>
where FC: FnMut() -> bool,
      FF: FnMut() -> (),
      FR: FnMut() -> (),
{
    int_vec: Vec<(usize,InternalCheck<'a,FC,FF,FR>)>,
    ext_vec: Vec<(usize,ICSDep<'a,S>)>,
    err_vec: Vec<Option<ICSError<'a>>>,
    id: usize,
    ps: usize,
}

#[allow(unused)]
impl<'a,FC,FF,FR,const S: usize> ICS<'a,FC,FF,FR,S> 
where FC : FnMut() -> bool,
      FF : FnMut() -> (),
      FR : FnMut() -> (),
{
    pub fn new(id:usize, parts: usize) -> Self {
        
        Self {
            int_vec: Vec::new(),
            ext_vec: Vec::new(),
            err_vec: Vec::new(),
            id,
            ps: parts
        }
    }

    pub fn with_capacity(
        int_err_cap: usize, 
        ext_err_cap: usize, 
        error_cap: usize, 
        id:usize, 
        parts: usize) -> Self {
        let ev = Vec::with_capacity(error_cap);
        let ie = Vec::with_capacity(int_err_cap);
        let ee = Vec::with_capacity(ext_err_cap);
        Self {int_vec: ie,ext_vec: ee, err_vec: ev,id, ps: parts}
    }

    pub fn add_internal_check(&mut self, check: InternalCheck<'a,FC,FF,FR>){
        let l = self.err_vec.len();
        self.int_vec.push((l,check));
        self.err_vec.push(None)
    }

    pub fn add_external_check(&mut self, check: ICSDep<'a,S>) -> usize{
        let l = self.err_vec.len();
        self.ext_vec.push((l,check));
        self.err_vec.push(None);
        l
    }

    pub fn internal_check(&'a mut self) {
        for int_check in &mut self.int_vec{
            let (err_i,int_check) = int_check;
            let mut err_cel = &mut self.err_vec[*err_i];
            match int_check.run_check() {
                ErrStatus::ERR => 
                {
                    let err = ICSError{
                        e_type: ErrorType::INTERNAL,
                        e_desc: int_check.get_description(),
                    };

                    *err_cel = Some(err);
                },
                _ => *err_cel = None,
            }
        }
    }


    pub fn check_specific_mex(&'a mut self,mex: &ICSMex<S>, ext_err_index: usize) -> Result<(),&str>{
        if ext_err_index >= self.ext_vec.len() {
            return Err("invalid index range fir ext_vec")
        }

        let (ch_index,ext_check) = &mut self.ext_vec[ext_err_index];
        let mut err_cel = &mut self.err_vec[*ch_index];
        match ext_check.check_mex(mex) {
            Ok(ErrStatus::ERR)=> {
                    let err = ICSError{
                        e_type: ErrorType::EXTERNAL,
                        e_desc: ext_check.get_description(),
                    };

                    *err_cel = Some(err);
                    Ok(())
            },
            Ok(ErrStatus::OK) => Ok(()),
            Err(_) => Err("comparing wrong messages"),
        }
    }

    pub fn get_err_info(&self,err_type: ErrorType, err_index: usize) -> Option<&str> {
        fn get_dscr<'a,G: GenericCheck<'a>>(vc : &'a Vec<(usize,G)>, idx: usize) -> Option<&'a str>{
                if idx < vc.len(){
                    let (_,err) = &vc[idx];
                    Some(err.get_description())
                }else{
                    None
                }
        }
        match err_type {
            ErrorType::INTERNAL => get_dscr(&self.int_vec, err_index),
            ErrorType::EXTERNAL => get_dscr(&self.ext_vec, err_index),
        }
    }

    pub fn create_ics_messages(&self) -> Box<[ICSMex<S>]>{
        let num_mex = {
            match (self.err_vec.len()/S, self.err_vec.len()%S){
                (i,0) => i,
                (i,_) => i +1
            }
        };
        let mut res = Vec::with_capacity(num_mex);
        for i in 0..num_mex{
            let mut mex: ICSMex<S> = ICSMex::new(self.id, self.ps);
            for j in 0_u8..8_u8{
                match self.err_vec[(i*8)+ usize::from(j)] {
                    None => (),
                    Some(_) => {
                        mex.set_err(i, j);
                    },
                };
            }
            res.push(mex);
        }

        res.into_boxed_slice()
    }
}

#[cfg(test)]
mod test{
}
