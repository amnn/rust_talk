mod tests {
    use test::Bencher;

    #[bench]
    fn bench_add1(b : &mut Bencher) {
        b.iter(|| {
            let mut res = 0i32;
            for _ in (0..2000) {
                match res {
                    -1 => {}
                    x if x < 1000 => res += 1,
                    _             => res = -1
                }
            };
            res
        })
    }

    #[bench]
    fn bench_add2(b : &mut Bencher) {
        b.iter(|| {
            (0..2000).fold(Some(0), |r, _| {
                r.and_then(|x| {
                    if x < 1000 { Some(x + 1) }
                    else        { None }
                })
            })
        })
    }
}

// test option::tests::bench_add1 ... bench:      2959 ns/iter (+/- 1509)
// test option::tests::bench_add2 ... bench:      3313 ns/iter (+/- 961)
