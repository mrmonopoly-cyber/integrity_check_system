use crate::bus::BusComumication;


#[allow(dead_code)]
#[derive(Debug)]
pub struct ExternalCheck<E,B: BusComumication<E>> {
    bus:  B,
    error_to_check: E,
}
