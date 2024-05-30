pub struct S;

impl S {
    #[cfg(feature = "foo-method")]
    pub fn foo() {}
}
