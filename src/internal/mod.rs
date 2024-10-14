#[derive(Debug)]
pub struct InternalCheck{
    pub check_var: fn() -> bool,
    pub manage_fail: fn() -> (),
}

#[cfg(test)]
mod tests {
}
