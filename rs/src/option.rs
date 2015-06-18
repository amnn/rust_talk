pub fn maybe_add1(x : i32) -> i32 {
    // Pre: x >= 0
    if x >= 1000 { -1 }
    else         { x + 1 }
}

pub fn maybe_add2(x : u32) -> Option<u32> {
    if x >= 1000 { None }
    else         { Some(x + 1) }
}

mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add1(b : &mut Bencher) {
        b.iter(|| {
            let mut res = 0i32;
            for _ in (0..2000) {
                if res != -1 {
                    res = maybe_add1(res)
                }
            };
            res
        })
    }

    #[bench]
    fn bench_add2(b : &mut Bencher) {
        b.iter(|| {
            let mut res = Some(0);
            for _ in (0..2000) {
                res = res.and_then(maybe_add2);
            };
            res
        })
    }
}

// test option::tests::bench_add1 ... bench:      2840 ns/iter (+/- 323)
// test option::tests::bench_add2 ... bench:      3221 ns/iter (+/- 436)
