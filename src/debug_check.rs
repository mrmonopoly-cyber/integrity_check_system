use crate::ics_trait::generic_check::*;
use core::sync::atomic;

#[derive(Debug)]
pub struct CheckU8<'a,const MIN :u8,const MAX : u8,const FV:u8,const DF: u8> {
    vp : &'a atomic::AtomicU8,
}

impl<'a,const MIN :u8,const MAX :u8, const FV: u8, const DV :u8 > ObjectCheck  
for CheckU8<'a,MIN,MAX,FV,DV>{
    fn check(&self) -> bool {
        let vp = self.vp.load(atomic::Ordering::Relaxed);
        vp >= MIN && vp <= MAX
    }

}

impl<'a,const MIN :u8,const MAX :u8, const FV: u8, const DV :u8 > MexConseguence 
for CheckU8<'a,MIN,MAX,FV,DV>{
    fn manage_fail(&mut self) -> () {
        self.vp.store(FV, atomic::Ordering::Relaxed)
    }

    fn restore_fail(&mut self) -> () {
        self.vp.store(DV, atomic::Ordering::Relaxed)
    }
}
#[allow(unused)]
impl<'a,const MIN :u8,const MAX :u8, const FV: u8, const DV :u8 > CheckU8<'a,MIN,MAX,FV,DV>{
    pub fn new(v: &'a atomic::AtomicU8) -> Self {
        Self{vp: v}
    }
}

#[derive(Debug)]
pub struct CheckVr<'a> {
    vp : &'a str,
}

impl<'a> ObjectCheck 
for CheckVr<'a>{
    fn check(&self) -> bool {
        self.vp == "hello"
    }
}

impl<'a> MexConseguence for CheckVr<'a>{
    fn manage_fail(&mut self) -> () {
        self.vp = "fail"
    }

    fn restore_fail(&mut self) -> () {
        self.vp = "restored"
    }
}

#[allow(unused)]
impl<'a> CheckVr<'a> {
    pub fn new(vp: &'a str) -> Self{
        Self{vp}
    }

    pub fn update(&mut self,n: &'a str) {
        self.vp = n;
    }
}
