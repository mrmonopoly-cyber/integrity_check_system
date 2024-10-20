pub trait InternalCheck {
    fn check_var(&self) -> bool;
    fn manage_fail(&self) -> ();
}
