#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_normal(b: &mut Bencher) {
        let h = HashMap::from([('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4)]);
        b.iter(|| {
            ('a'..='e').for_each(|x| {
                h.get(&x).unwrap();
            });
        });
    }

    #[bench]
    fn bench_vec(b: &mut Bencher) {
        let v = ['a', 'b', 'c', 'd', 'e'];
        b.iter(|| {
            ('a'..='e').for_each(|x| {
                let y = x.to_digit(16).unwrap() - 10;
                v.get(y as usize).unwrap();
            })
        });
    }
}
