mod internal;
mod external;
mod bus;

use bus::BusComumication;
use internal::InternalCheck;
use external::ExternalCheck;


#[allow(dead_code)]
#[derive(Debug)]
pub struct ErrorMessage{
    node_id: u8,
    err_vec: [u8;7],
}

#[derive(Debug)]
pub struct ICS<T : BusComumication<ErrorMessage>> {
    node_id: u8,
    internal_check: Vec<InternalCheck>,
    external_check: Vec<ExternalCheck<ErrorMessage,T>>,
    bus:T,
}

impl<T: BusComumication<ErrorMessage>> ICS<T> {
    pub fn new(node_id: u8,bus :T) -> Self {
        Self {
            node_id, 
            internal_check: Vec::new(), 
            external_check: Vec::new(),
            bus}
    }

    pub fn add_check(&mut self, check: InternalCheck) -> (){
        self.internal_check.push(check)
    }

    pub fn run_check(& mut self) -> (bool,ErrorMessage){
        let mut err_mex : ErrorMessage = ErrorMessage {node_id: self.node_id, err_vec: [0;7] };
        let mut i = 0;
        for err_instance in self.internal_check.iter(){
            let err_cel = i/7;
            let err_bit = i%7;
            match (err_instance.check_var)() {
                true => {
                    (err_instance.manage_fail)();
                    err_mex.err_vec[err_cel] |= 1 << err_bit;
                },
                false => (),
            }
            if (err_instance.check_var)() {
            };
            i = i + 1;
        };
        if let Some(_) = self.bus.send(&err_mex) {
            return (false,err_mex);
        }
        (true,err_mex)
    }
}

#[cfg(test)]
mod tests {
}
