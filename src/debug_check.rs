use crate::ics_trait::generic_check::*;
use core::{fmt::Debug, sync::atomic};

#[allow(unused)]
pub struct CheckWithEnv<'a,FC,FF,FR> where 
FC : Fn () -> bool,
FF : FnMut () -> (),
FR : FnMut () -> (),
{
    v: &'a atomic::AtomicU8,
    check_f: FC,
    fail_f : FF,
    restore_f : FR,
}

impl<'a,FC,FF,FR> Debug for CheckWithEnv<'a,FC,FF,FR>where 
FC : Fn () -> bool,
FF : FnMut () -> (),
FR : FnMut () -> (),
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CheckWithEnv")
            .field("v", &self.v)
            .field("check_f", &"<function>")
            .field("fail_f", &"<function>")
            .field("restore_f", &"<function>")
            .finish()
    }
}


impl<'a,FC,FF,FR> ObjectCheck for CheckWithEnv<'a,FC,FF,FR>where 
FC : Fn () -> bool,
FF : FnMut () -> (),
FR : FnMut () -> (),
{
    fn check(&self) -> bool {
        self.v.load(atomic::Ordering::Relaxed) < 10 && (self.check_f)()
    }
}

impl<'a,FC,FF,FR> MexConseguence for CheckWithEnv<'a,FC,FF,FR> where 
FC : Fn () -> bool,
FF : FnMut () -> (),
FR : FnMut () -> (),
{
    fn manage_fail(&mut self) -> () {
        self.v.store(9, atomic::Ordering::Relaxed);
        (self.fail_f)();
    }

    fn restore_fail(&mut self) -> () {
        todo!()
    }
}

#[allow(unused)]
impl<'a,FC,FF,FR> CheckWithEnv<'a,FC,FF,FR> where 
FC : Fn () -> bool,
FF : FnMut () -> (),
FR : FnMut () -> (),
{
    pub fn new(v: &'a atomic::AtomicU8, check_f: FC, fail_f : FF, restore_f: FR) -> Self{
        Self{v,check_f,fail_f,restore_f}
    }
}

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
