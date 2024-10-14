#[derive(Debug)]
pub enum BusError {
    SendFailed,
    WriteFailed,
}

pub trait BusComumication<T>{
    fn send(self: &Self, mex: &T ) -> Option<BusError>;
    fn read(self:&Self) -> (T,Option<BusError>);
}

