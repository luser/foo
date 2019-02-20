extern {
    fn foo();
}

pub fn do_foo() {
    unsafe { foo() };
}
