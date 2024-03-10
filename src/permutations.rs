use std::ops::Mul;

#[derive(Debug, PartialEq, Eq)]
struct Perm<const M: usize>([usize; M]);

impl<const M: usize> Perm<M> {
    fn chain(&self, other: &Self) -> Self
    where
        Self: Sized,
    {
        let mut result = [0; M];
        // for (i, x) in self.0.iter().cloned().enumerate() {
        //     result[i] = other.0[x];
        // }
        for (i, x) in other.0.iter().cloned().enumerate() {
            result[i] = self.0[x];
        }
        Self(result)
    }
}

impl<const M: usize> Mul for Perm<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.chain(&rhs)
    }
}

#[derive(Debug)]
struct PermId<const M: usize>(usize);

impl<const M: usize> PermId<M> {
    pub fn perm(self) -> Perm<M> {
        let mut id = self.0;
        let mut factoriadic = vec![0];
        for base in 2..=M {
            factoriadic.push(id % base);
            id /= base;
        }
        assert_eq!(0, id, "id was larger than group size");
        factoriadic.reverse();

        let mut elements: Vec<usize> = (0..M).collect();
        let mut result = [0; M];
        for i in 0..M {
            result[i] = elements.remove(factoriadic[i]);
        }
        Perm(result)
    }
}

impl<const M: usize> From<PermId<M>> for Perm<M> {
    fn from(value: PermId<M>) -> Self {
        value.perm()
    }
}

impl<const M: usize> From<Perm<M>> for PermId<M> {
    fn from(_value: Perm<M>) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Perm, PermId};

    #[test]
    fn test1() {
        assert_eq!(Perm([0]), PermId::<1>(0).into());
    }

    #[test]
    fn test2() {
        assert_eq!(Perm([0, 1]), PermId::<2>(0).into());
        assert_eq!(Perm([1, 0]), PermId::<2>(1).into());
    }

    #[test]
    fn test3() {
        assert_eq!(Perm([0, 1, 2]), PermId::<3>(0).into());
        assert_eq!(Perm([0, 2, 1]), PermId::<3>(1).into());
        assert_eq!(Perm([1, 0, 2]), PermId::<3>(2).into());
        assert_eq!(Perm([1, 2, 0]), PermId::<3>(3).into());
        assert_eq!(Perm([2, 0, 1]), PermId::<3>(4).into());
        assert_eq!(Perm([2, 1, 0]), PermId::<3>(5).into());
    }

    #[test]
    fn test2_chain() {
        let e = Perm([0, 1]);
        let a = Perm([1, 0]);
        assert_eq!(e, e.chain(&e));
        assert_eq!(a, e.chain(&a));
        assert_eq!(a, a.chain(&e));
        assert_eq!(e, a.chain(&a));
    }

    #[test]
    fn test3_chain() {
        let e = Perm([0, 1, 2]);
        let f = Perm([0, 2, 1]);
        let fr = Perm([1, 0, 2]);
        let r = Perm([1, 2, 0]);
        let rr = Perm([2, 0, 1]);
        let y = Perm([2, 1, 0]);
        assert_eq!(e, f.chain(&f));
        assert_eq!(e, fr.chain(&fr));
        assert_eq!(e, y.chain(&y));
        assert_eq!(e, r.chain(&rr));
        assert_eq!(e, rr.chain(&r));
        assert_eq!(rr, r.chain(&r));
        assert_eq!(r, rr.chain(&rr));
        assert_eq!(fr, r.chain(&f));
        // assert_eq!(fr, f.chain(&r));
        assert_eq!(e, rr.chain(&rr).chain(&rr));
        assert_eq!(e, r.chain(&r).chain(&r));
    }

    #[test]
    #[should_panic]
    fn test1_oob() {
        PermId::<1>(1).perm();
    }

    #[test]
    #[should_panic]
    fn test2_oob() {
        PermId::<2>(2).perm();
    }

    #[test]
    #[should_panic]
    fn test3_oob() {
        PermId::<3>(6).perm();
    }
}
