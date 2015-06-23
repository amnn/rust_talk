#[cfg(feature = "never")]
mod will_fail {
    fn mk_int<'a>() -> &'a mut i32 { let mut x : i32; &mut x }

    fn sum<'a>(lo : i32, hi : i32) -> &'a i32 {
        let acc = mk_int();
        for i in lo..(hi+1) { *acc += i };
        acc
    }
}
