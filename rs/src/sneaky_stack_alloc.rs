#[cfg(feature = "never")]
mod will_fail {
    fn id<T>(x : T) -> T { x }

    fn mk_int<'a>() -> &'a mut i32 { let mut x : i32; id(&mut x) }

    fn sum<'a>(lo : i32, hi : i32) -> &'a i32 {
        let acc = mk_int();
        for i in 0..(hi+1) { *acc += i };
        acc
    }
}
