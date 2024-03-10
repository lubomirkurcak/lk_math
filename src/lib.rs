pub mod prelude;

pub mod aabb;
pub mod arraynd;
pub mod bijection;
pub mod cli;
pub mod explore;
pub mod expr;
pub mod geometric_algebra;
pub mod geometric_traits;
pub mod group;
pub mod interval;
pub mod interval_set;
pub mod line;
pub mod line_iterator;
pub mod linear_index;
pub mod math;
pub mod modular;
pub mod permutations;
pub mod sketch;
pub mod transformations;
pub mod vector;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
