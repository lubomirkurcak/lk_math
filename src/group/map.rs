pub type GroupElement<const M: usize> = [usize; M];

pub struct Group<const M: usize>;

impl<const M: usize> Group<M> {
    pub fn identity() -> GroupElement<M> {
        let mut result = [0; M];
        result
            .iter_mut()
            .enumerate()
            .take(M)
            .for_each(|(i, x)| *x = i);
        result
    }

    pub fn group_element(mut id: usize) -> GroupElement<M> {
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
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Group;

    fn test_gen<const M: usize>() {
        for i in 0..(1..=M).product() {
            println!(
                "assert_eq!({:?}, Group::<{M}>::group_element({i}));",
                Group::<M>::group_element(i),
            );
        }
    }

    #[test]
    fn test1() {
        assert_eq!([0], Group::<1>::group_element(0));
    }

    #[test]
    fn test2() {
        assert_eq!([0, 1], Group::<2>::group_element(0));
        assert_eq!([1, 0], Group::<2>::group_element(1));
    }

    #[test]
    fn test3() {
        assert_eq!([0, 1, 2], Group::<3>::group_element(0));
        assert_eq!([0, 2, 1], Group::<3>::group_element(1));
        assert_eq!([1, 0, 2], Group::<3>::group_element(2));
        assert_eq!([1, 2, 0], Group::<3>::group_element(3));
        assert_eq!([2, 0, 1], Group::<3>::group_element(4));
        assert_eq!([2, 1, 0], Group::<3>::group_element(5));
    }

    #[test]
    fn test4() {
        assert_eq!([0, 1, 2, 3], Group::<4>::group_element(0));
        assert_eq!([0, 1, 3, 2], Group::<4>::group_element(1));
        assert_eq!([0, 2, 1, 3], Group::<4>::group_element(2));
        assert_eq!([0, 2, 3, 1], Group::<4>::group_element(3));
        assert_eq!([0, 3, 1, 2], Group::<4>::group_element(4));
        assert_eq!([0, 3, 2, 1], Group::<4>::group_element(5));
        assert_eq!([1, 0, 2, 3], Group::<4>::group_element(6));
        assert_eq!([1, 0, 3, 2], Group::<4>::group_element(7));
        assert_eq!([1, 2, 0, 3], Group::<4>::group_element(8));
        assert_eq!([1, 2, 3, 0], Group::<4>::group_element(9));
        assert_eq!([1, 3, 0, 2], Group::<4>::group_element(10));
        assert_eq!([1, 3, 2, 0], Group::<4>::group_element(11));
        assert_eq!([2, 0, 1, 3], Group::<4>::group_element(12));
        assert_eq!([2, 0, 3, 1], Group::<4>::group_element(13));
        assert_eq!([2, 1, 0, 3], Group::<4>::group_element(14));
        assert_eq!([2, 1, 3, 0], Group::<4>::group_element(15));
        assert_eq!([2, 3, 0, 1], Group::<4>::group_element(16));
        assert_eq!([2, 3, 1, 0], Group::<4>::group_element(17));
        assert_eq!([3, 0, 1, 2], Group::<4>::group_element(18));
        assert_eq!([3, 0, 2, 1], Group::<4>::group_element(19));
        assert_eq!([3, 1, 0, 2], Group::<4>::group_element(20));
        assert_eq!([3, 1, 2, 0], Group::<4>::group_element(21));
        assert_eq!([3, 2, 0, 1], Group::<4>::group_element(22));
        assert_eq!([3, 2, 1, 0], Group::<4>::group_element(23));
    }

    #[test]
    fn test5() {
        assert_eq!([0, 1, 2, 3, 4], Group::<5>::group_element(0));
        assert_eq!([0, 1, 2, 4, 3], Group::<5>::group_element(1));
        assert_eq!([0, 1, 3, 2, 4], Group::<5>::group_element(2));
        assert_eq!([0, 1, 3, 4, 2], Group::<5>::group_element(3));
        assert_eq!([0, 1, 4, 2, 3], Group::<5>::group_element(4));
        assert_eq!([0, 1, 4, 3, 2], Group::<5>::group_element(5));
        assert_eq!([0, 2, 1, 3, 4], Group::<5>::group_element(6));
        assert_eq!([0, 2, 1, 4, 3], Group::<5>::group_element(7));
        assert_eq!([0, 2, 3, 1, 4], Group::<5>::group_element(8));
        assert_eq!([0, 2, 3, 4, 1], Group::<5>::group_element(9));
        assert_eq!([0, 2, 4, 1, 3], Group::<5>::group_element(10));
        assert_eq!([0, 2, 4, 3, 1], Group::<5>::group_element(11));
        assert_eq!([0, 3, 1, 2, 4], Group::<5>::group_element(12));
        assert_eq!([0, 3, 1, 4, 2], Group::<5>::group_element(13));
        assert_eq!([0, 3, 2, 1, 4], Group::<5>::group_element(14));
        assert_eq!([0, 3, 2, 4, 1], Group::<5>::group_element(15));
        assert_eq!([0, 3, 4, 1, 2], Group::<5>::group_element(16));
        assert_eq!([0, 3, 4, 2, 1], Group::<5>::group_element(17));
        assert_eq!([0, 4, 1, 2, 3], Group::<5>::group_element(18));
        assert_eq!([0, 4, 1, 3, 2], Group::<5>::group_element(19));
        assert_eq!([0, 4, 2, 1, 3], Group::<5>::group_element(20));
        assert_eq!([0, 4, 2, 3, 1], Group::<5>::group_element(21));
        assert_eq!([0, 4, 3, 1, 2], Group::<5>::group_element(22));
        assert_eq!([0, 4, 3, 2, 1], Group::<5>::group_element(23));
        assert_eq!([1, 0, 2, 3, 4], Group::<5>::group_element(24));
        assert_eq!([1, 0, 2, 4, 3], Group::<5>::group_element(25));
        assert_eq!([1, 0, 3, 2, 4], Group::<5>::group_element(26));
        assert_eq!([1, 0, 3, 4, 2], Group::<5>::group_element(27));
        assert_eq!([1, 0, 4, 2, 3], Group::<5>::group_element(28));
        assert_eq!([1, 0, 4, 3, 2], Group::<5>::group_element(29));
        assert_eq!([1, 2, 0, 3, 4], Group::<5>::group_element(30));
        assert_eq!([1, 2, 0, 4, 3], Group::<5>::group_element(31));
        assert_eq!([1, 2, 3, 0, 4], Group::<5>::group_element(32));
        assert_eq!([1, 2, 3, 4, 0], Group::<5>::group_element(33));
        assert_eq!([1, 2, 4, 0, 3], Group::<5>::group_element(34));
        assert_eq!([1, 2, 4, 3, 0], Group::<5>::group_element(35));
        assert_eq!([1, 3, 0, 2, 4], Group::<5>::group_element(36));
        assert_eq!([1, 3, 0, 4, 2], Group::<5>::group_element(37));
        assert_eq!([1, 3, 2, 0, 4], Group::<5>::group_element(38));
        assert_eq!([1, 3, 2, 4, 0], Group::<5>::group_element(39));
        assert_eq!([1, 3, 4, 0, 2], Group::<5>::group_element(40));
        assert_eq!([1, 3, 4, 2, 0], Group::<5>::group_element(41));
        assert_eq!([1, 4, 0, 2, 3], Group::<5>::group_element(42));
        assert_eq!([1, 4, 0, 3, 2], Group::<5>::group_element(43));
        assert_eq!([1, 4, 2, 0, 3], Group::<5>::group_element(44));
        assert_eq!([1, 4, 2, 3, 0], Group::<5>::group_element(45));
        assert_eq!([1, 4, 3, 0, 2], Group::<5>::group_element(46));
        assert_eq!([1, 4, 3, 2, 0], Group::<5>::group_element(47));
        assert_eq!([2, 0, 1, 3, 4], Group::<5>::group_element(48));
        assert_eq!([2, 0, 1, 4, 3], Group::<5>::group_element(49));
        assert_eq!([2, 0, 3, 1, 4], Group::<5>::group_element(50));
        assert_eq!([2, 0, 3, 4, 1], Group::<5>::group_element(51));
        assert_eq!([2, 0, 4, 1, 3], Group::<5>::group_element(52));
        assert_eq!([2, 0, 4, 3, 1], Group::<5>::group_element(53));
        assert_eq!([2, 1, 0, 3, 4], Group::<5>::group_element(54));
        assert_eq!([2, 1, 0, 4, 3], Group::<5>::group_element(55));
        assert_eq!([2, 1, 3, 0, 4], Group::<5>::group_element(56));
        assert_eq!([2, 1, 3, 4, 0], Group::<5>::group_element(57));
        assert_eq!([2, 1, 4, 0, 3], Group::<5>::group_element(58));
        assert_eq!([2, 1, 4, 3, 0], Group::<5>::group_element(59));
        assert_eq!([2, 3, 0, 1, 4], Group::<5>::group_element(60));
        assert_eq!([2, 3, 0, 4, 1], Group::<5>::group_element(61));
        assert_eq!([2, 3, 1, 0, 4], Group::<5>::group_element(62));
        assert_eq!([2, 3, 1, 4, 0], Group::<5>::group_element(63));
        assert_eq!([2, 3, 4, 0, 1], Group::<5>::group_element(64));
        assert_eq!([2, 3, 4, 1, 0], Group::<5>::group_element(65));
        assert_eq!([2, 4, 0, 1, 3], Group::<5>::group_element(66));
        assert_eq!([2, 4, 0, 3, 1], Group::<5>::group_element(67));
        assert_eq!([2, 4, 1, 0, 3], Group::<5>::group_element(68));
        assert_eq!([2, 4, 1, 3, 0], Group::<5>::group_element(69));
        assert_eq!([2, 4, 3, 0, 1], Group::<5>::group_element(70));
        assert_eq!([2, 4, 3, 1, 0], Group::<5>::group_element(71));
        assert_eq!([3, 0, 1, 2, 4], Group::<5>::group_element(72));
        assert_eq!([3, 0, 1, 4, 2], Group::<5>::group_element(73));
        assert_eq!([3, 0, 2, 1, 4], Group::<5>::group_element(74));
        assert_eq!([3, 0, 2, 4, 1], Group::<5>::group_element(75));
        assert_eq!([3, 0, 4, 1, 2], Group::<5>::group_element(76));
        assert_eq!([3, 0, 4, 2, 1], Group::<5>::group_element(77));
        assert_eq!([3, 1, 0, 2, 4], Group::<5>::group_element(78));
        assert_eq!([3, 1, 0, 4, 2], Group::<5>::group_element(79));
        assert_eq!([3, 1, 2, 0, 4], Group::<5>::group_element(80));
        assert_eq!([3, 1, 2, 4, 0], Group::<5>::group_element(81));
        assert_eq!([3, 1, 4, 0, 2], Group::<5>::group_element(82));
        assert_eq!([3, 1, 4, 2, 0], Group::<5>::group_element(83));
        assert_eq!([3, 2, 0, 1, 4], Group::<5>::group_element(84));
        assert_eq!([3, 2, 0, 4, 1], Group::<5>::group_element(85));
        assert_eq!([3, 2, 1, 0, 4], Group::<5>::group_element(86));
        assert_eq!([3, 2, 1, 4, 0], Group::<5>::group_element(87));
        assert_eq!([3, 2, 4, 0, 1], Group::<5>::group_element(88));
        assert_eq!([3, 2, 4, 1, 0], Group::<5>::group_element(89));
        assert_eq!([3, 4, 0, 1, 2], Group::<5>::group_element(90));
        assert_eq!([3, 4, 0, 2, 1], Group::<5>::group_element(91));
        assert_eq!([3, 4, 1, 0, 2], Group::<5>::group_element(92));
        assert_eq!([3, 4, 1, 2, 0], Group::<5>::group_element(93));
        assert_eq!([3, 4, 2, 0, 1], Group::<5>::group_element(94));
        assert_eq!([3, 4, 2, 1, 0], Group::<5>::group_element(95));
        assert_eq!([4, 0, 1, 2, 3], Group::<5>::group_element(96));
        assert_eq!([4, 0, 1, 3, 2], Group::<5>::group_element(97));
        assert_eq!([4, 0, 2, 1, 3], Group::<5>::group_element(98));
        assert_eq!([4, 0, 2, 3, 1], Group::<5>::group_element(99));
        assert_eq!([4, 0, 3, 1, 2], Group::<5>::group_element(100));
        assert_eq!([4, 0, 3, 2, 1], Group::<5>::group_element(101));
        assert_eq!([4, 1, 0, 2, 3], Group::<5>::group_element(102));
        assert_eq!([4, 1, 0, 3, 2], Group::<5>::group_element(103));
        assert_eq!([4, 1, 2, 0, 3], Group::<5>::group_element(104));
        assert_eq!([4, 1, 2, 3, 0], Group::<5>::group_element(105));
        assert_eq!([4, 1, 3, 0, 2], Group::<5>::group_element(106));
        assert_eq!([4, 1, 3, 2, 0], Group::<5>::group_element(107));
        assert_eq!([4, 2, 0, 1, 3], Group::<5>::group_element(108));
        assert_eq!([4, 2, 0, 3, 1], Group::<5>::group_element(109));
        assert_eq!([4, 2, 1, 0, 3], Group::<5>::group_element(110));
        assert_eq!([4, 2, 1, 3, 0], Group::<5>::group_element(111));
        assert_eq!([4, 2, 3, 0, 1], Group::<5>::group_element(112));
        assert_eq!([4, 2, 3, 1, 0], Group::<5>::group_element(113));
        assert_eq!([4, 3, 0, 1, 2], Group::<5>::group_element(114));
        assert_eq!([4, 3, 0, 2, 1], Group::<5>::group_element(115));
        assert_eq!([4, 3, 1, 0, 2], Group::<5>::group_element(116));
        assert_eq!([4, 3, 1, 2, 0], Group::<5>::group_element(117));
        assert_eq!([4, 3, 2, 0, 1], Group::<5>::group_element(118));
        assert_eq!([4, 3, 2, 1, 0], Group::<5>::group_element(119));
    }

    #[test]
    fn test6() {
        test_gen::<6>();
        // panic!();
    }
}
