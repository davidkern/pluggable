use pluggable::pluggable;

pluggable! {
    interface IFoo: 0x0abcdef0_0abcdef0_0abcdef0_0abcdef0 {
        fn foo(&self);
    }
}

#[test]
fn works() {
}
