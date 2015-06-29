#[cfg(feature="never")]
mod will_fail {
    fn main() {
        let mut v = vec![1, 2, 3];
        for i in &v {
            for j in 0..(i+1) {
                println!("{}, {}", i, j);
                v.insert(0, i + j);
            }
        }
    }
}
