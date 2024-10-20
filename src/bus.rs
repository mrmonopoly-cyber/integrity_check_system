#[allow(dead_code)]
#[derive(Debug)]
pub enum BusError {
    SendFailed,
    WriteFailed,
}

#[allow(dead_code)]
pub trait BusComumication<T>{
    fn send(self: &Self, mex: &T ) -> Option<BusError>;
    fn read(self:&Self) -> (T,Option<BusError>);
}

