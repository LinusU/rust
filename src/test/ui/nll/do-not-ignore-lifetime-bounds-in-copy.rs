// Test that the 'static bound from the Copy impl is respected. Regression test for #29149.

#![feature(nll)]

#[derive(Clone)] struct Foo<'a>(&'a u32);
impl Copy for Foo<'static> {}

fn main() {
    let s = 2;
    let a = Foo(&s); //~ ERROR `s` does not live long enough [E0597]
    drop(a);
    drop(a);
}
