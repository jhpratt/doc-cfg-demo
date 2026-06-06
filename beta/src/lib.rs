pub struct S;

impl S {
    #[cfg(feature = "wrong-feature-name")]
    pub fn foo() {}
}
