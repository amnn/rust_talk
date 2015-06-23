fn mk_int<'a>() -> Box<i32> { Box::new(0) }

fn sum<'a>(lo : i32, hi : i32) -> Box<i32> {
    let mut acc = mk_int();
    for i in 0..(hi+1) { *acc += i };
    acc
}
