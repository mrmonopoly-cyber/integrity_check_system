#[derive(Debug)]
pub struct InternalCheck<T>{
    var: &T,
    pub check_var: fn() -> bool
}

impl InternalCheck<T> {
    fn new(var: &T, chek_var: fn() -> bool) -> Self {
        Self { var, check_var}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
