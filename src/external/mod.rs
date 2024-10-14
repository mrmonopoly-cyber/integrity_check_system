use crate::bus::BusComumication;


#[derive(Debug)]
pub struct ExternalCheck<E,M: BusComumication<E>> {
    bus:  M,
    error_to_check: E,
}
