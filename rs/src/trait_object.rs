extern crate test;

pub trait Foo {
    fn bar(&self) -> i32;
}

pub struct A;
impl Foo for A {
    fn bar(&self) -> i32 { 0 }
}

pub struct B;
impl Foo for B {
    fn bar(&self) -> i32 { 1 }
}

pub fn raw_a_foo(a : A) -> i32 { a.bar() }
pub fn raw_b_foo(b : B) -> i32 { b.bar() }

pub fn gen_foo<T : Foo>(x : T) -> i32 { x.bar() }
pub fn trt_foo(x : &Foo) -> i32 { x.bar() }

mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_raw(b : &mut Bencher) {
        b.iter(|| black_box(raw_a_foo(A)));
        b.iter(|| black_box(raw_b_foo(B)));
    }

    #[bench]
    fn bench_gen(b : &mut Bencher) {
        b.iter(|| black_box(gen_foo(A)));
        b.iter(|| black_box(gen_foo(B)));
    }

    #[bench]
    fn bench_trt(b : &mut Bencher) {
        b.iter(|| black_box(trt_foo(&A)));
        b.iter(|| black_box(trt_foo(&B)));
    }
}
