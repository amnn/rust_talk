#[cfg(feature="never")]
mod will_fail {
    use std::cmp::max;

    unsafe fn make_room<T>(v : &mut Vec<T>, req : usize) {
        let new_size = max(req, v.len());
        v.reserve(new_size);
        v.set_len(new_size);
    }

    fn array_copy<T : Clone>(
        src  : &Vec<T>,
        dest : &mut Vec<T>,

        src_off  : usize,
        dest_off : usize,

        len : usize) {
        let iter =
            src[src_off..src_off+len]
              .iter().enumerate();

        unsafe {
            make_room(dest, dest_off + len);
            for (i, x) in iter {
                dest.insert(dest_off + i, x.clone());
            }
        }
    }

    fn test() {
        let mut v = vec![1, 2, 3];
        array_copy(&mut v, &mut v, 0, 1, 3);
    }
}
