use std::slice;
#[allow(dead_code)]
trait RefEx {
    fn split_at_mut<T>(&mut self, index: usize) -> (&mut Self, &mut Self);
}

impl<K> RefEx for [K] {
    fn split_at_mut<T>(&mut self, index: usize) -> (&mut Self, &mut Self)
    where
        T: Sized,
    {
        let len = self.len();
        let ptr = self.as_mut_ptr();

        assert!(index <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, index),
                slice::from_raw_parts_mut(ptr.add(index), len - index),
            )
        }
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn mutate_refs() {
        let mut v = vec![2, 54, 56, 765, 1, 35];
        let (r, l) = v[..].split_at_mut(3);
        r[0] = 0;
        l[0] = 0;
        assert_eq!(vec![0, 54, 56, 0, 1, 35], v);
    }
}
