use std::{
    fmt::Display,
    io::{BufRead, BufReader},
    iter,
    str::FromStr,
};

use super::{
    geometric_traits::{IterateNeighbours, IterateNeighboursContext},
    line::Line,
    line_iterator::LineIterator,
    linear_index::LinearIndex,
    vector::Vector,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArrayNd<const N: usize, T> {
    pub data: Vec<T>,
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub dims: [usize; N],
    // #[serde(with = "serde_arrays")]
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub dim_strides: [usize; N],
}

impl<const C: usize, T: Copy> ArrayNd<C, T> {
    pub fn new<U: Copy + TryInto<usize>>(dims: [U; C], default: T) -> Self {
        let mut d = [0; C];
        let mut current_stride = 1;
        let mut dim_strides = [0; C];
        for i in 0..C {
            d[i] = dims[i].try_into().ok().unwrap();
            dim_strides[i] = current_stride;
            current_stride *= d[i];
            assert_ne!(d[i], 0);
        }

        Self {
            data: iter::repeat(default).take(d.iter().product()).collect(),
            dims: d,
            dim_strides,
        }
    }

    pub fn from_slice<U: Copy + TryInto<usize>>(dims: [U; C], slice: &[T]) -> Self {
        let mut d = [0; C];
        let mut current_stride = 1;
        let mut dim_strides = [0; C];
        for i in 0..C {
            d[i] = dims[i].try_into().ok().unwrap();
            dim_strides[i] = current_stride;
            current_stride *= d[i];
            assert_ne!(d[i], 0);
        }
        Self {
            data: slice.to_owned(),
            dims: d,
            dim_strides,
        }
    }

    pub fn resized(&self, new_dims: [usize; C], default: T, offset: Vector<C, i32>) -> Self {
        let mut new = Self::new(new_dims, default);
        new.data
            .iter_mut()
            .enumerate()
            .for_each(|(linear_index, val)| {
                let i: Vector<C, i32> = Vector::new(new_dims)
                    .unindex(linear_index)
                    .unwrap()
                    .try_into()
                    .unwrap();
                if let Some(src) = self.get(i - offset) {
                    *val = *src;
                }
            });

        new
    }
    pub fn padded(&self, padding: i32, default: T) -> Self {
        let mut new_dims = self.dims;
        new_dims.iter_mut().for_each(|x| *x += 2 * padding as usize);

        self.resized(new_dims, default, Vector::all(padding))
    }
}

macro_rules! array_vector_linear_index {
    ($($t:ty),*) => {
        $(
impl<const N: usize, T> LinearIndex<Vector<N, $t>> for ArrayNd<N, T> {
    fn index_unchecked(&self, i: Vector<N, $t>) -> Option<usize> {
        Vector::new(self.dims).index_unchecked(i.try_into().unwrap())
    }
    fn unindex(&self, i: usize) -> Option<Vector<N, $t>> {
        if let Some(a) = Vector::new(self.dims).unindex(i) {
            match a.try_into() {
                Ok(a) => Some(a),
                Err(_) => None,
            }
        } else {
            None
        }
    }
    unsafe fn cardinality(&self) -> Option<usize> {
        Some(self.dims.iter().product())
    }
    fn is_in_bounds(&self, i: &Vector<N, $t>) -> bool {
        if let Ok(a) = (*i).try_into() {
            Vector::new(self.dims).is_in_bounds(&a)
        } else {
            false
        }
    }
}
        )*
    };
}

array_vector_linear_index!(i32, usize);

impl<const N: usize, T: Copy + PartialEq> ArrayNd<N, T> {
    pub fn replace_all(&mut self, from: &T, to: &T) {
        self.data.iter_mut().for_each(|x| {
            if x == from {
                *x = *to;
            }
        })
    }
}

impl<const N: usize, T> ArrayNd<N, T> {
    pub fn get_linear(&self, index: usize) -> &T {
        &self.data[index]
    }
    pub fn get_mut_linear(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }
    pub fn set_linear(&mut self, index: usize, v: T) {
        *self.get_mut_linear(index) = v;
    }

    pub fn get<I>(&self, p: I) -> Option<&T>
    where
        Self: LinearIndex<I>,
    {
        match self.index(p) {
            Some(index) => Some(self.get_linear(index)),
            None => None,
        }
    }
    pub fn get_mut<I>(&mut self, p: I) -> Option<&mut T>
    where
        Self: LinearIndex<I>,
    {
        match self.index(p) {
            Some(index) => Some(self.get_mut_linear(index)),
            None => None,
        }
    }
    pub fn set<I>(&mut self, p: I, v: T) -> bool
    where
        Self: LinearIndex<I>,
    {
        match self.get_mut(p) {
            Some(a) => {
                *a = v;
                true
            }
            None => false,
        }
    }
}

impl<const N: usize, T> ArrayNd<N, T> {
    fn find_internal<F, I>(&self, predicate: F) -> Option<I>
    where
        F: Fn(&T) -> bool,
        Self: LinearIndex<I>,
    {
        match self.data.iter().position(predicate) {
            Some(index) => self.unindex(index),
            None => None,
        }
    }

    fn rfind_internal<F, I>(&self, predicate: F) -> Option<I>
    where
        F: Fn(&T) -> bool,
        Self: LinearIndex<I>,
    {
        match self.data.iter().rposition(predicate) {
            Some(index) => self.unindex(index),
            None => None,
        }
    }

    fn find_item_internal<I>(&self, item: &T) -> Option<I>
    where
        T: PartialEq,
        Self: LinearIndex<I>,
    {
        self.find_internal(|x| x == item)
    }

    fn rfind_item_internal<I>(&self, item: &T) -> Option<I>
    where
        T: PartialEq,
        Self: LinearIndex<I>,
    {
        self.rfind_internal(|x| x == item)
    }

    pub fn find_item(&self, item: &T) -> Option<Vector<N, i32>>
    where
        T: PartialEq,
    {
        self.find_item_internal(item)
    }

    pub fn find_last_item(&self, item: &T) -> Option<Vector<N, i32>>
    where
        T: PartialEq,
    {
        self.rfind_item_internal(item)
    }

    pub fn find_all<F, I>(&self, predicate: F) -> Vec<I>
    where
        F: Fn(&T) -> bool,
        T: PartialEq,
        Self: LinearIndex<I>,
    {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, x)| predicate(x))
            .map(|(i, _)| self.unindex(i).unwrap())
            .collect()
    }

    pub fn find_all_items<I>(&self, item: &T) -> Vec<I>
    where
        T: PartialEq,
        Self: LinearIndex<I>,
    {
        self.find_all(|x| x == item)
    }
}

impl<const N: usize, T> ArrayNd<N, T> {
    pub fn map<F, U>(&self, f: F) -> ArrayNd<N, U>
    where
        F: Fn(&T) -> U,
    {
        let data = self.data.iter().map(f).collect();

        ArrayNd::<N, U> {
            data,
            dims: self.dims,
            dim_strides: self.dim_strides,
        }
    }
}

impl<const N: usize, T> ArrayNd<N, T> {
    pub fn line_iter<const B: bool>(
        &self,
        p0: Vector<N, i32>,
        p1: Vector<N, i32>,
    ) -> LineIterator<B, N> {
        LineIterator::new(p0, p1)
    }
}

impl<const N: usize, T> ArrayNd<N, T> {
    pub fn iter_values_in_line<const B: bool>(
        &'_ self,
        p0: Vector<N, i32>,
        p1: Vector<N, i32>,
    ) -> impl Iterator<Item = &'_ T> {
        // self.line_iter(p0, p1).filter_map(|p| self.get(p))
        self.line_iter::<B>(p0, p1).map(|p| self.get(p).unwrap())
    }
}

impl<const N: usize, T: Copy> ArrayNd<N, T> {
    pub fn draw_line<const B: bool>(&mut self, line: Line<Vector<N, i32>>, v: T) {
        for p in self.line_iter::<B>(line.start, line.end) {
            self.set(p, v);
        }
    }
    // pub fn draw_line_from_points<const B: bool>(
    //     &mut self,
    //     p0: Vector<N, i32>,
    //     p1: Vector<N, i32>,
    //     v: T,
    // ) {
    //     for p in self.line_iter::<B>(p0, p1) {
    //         self.set(p, v);
    //     }
    // }
}

// NOTE(lubo): Choose which slice (index) to paint in each dimension, or pass None to paint all tiles in that dimension.
// Example:
//   Draw a plane at Y = 3 in a 3D array
//   a (: Array3D) .draw_block(&[None, Some(3), None])
impl<const N: usize, T: Copy> ArrayNd<N, T> {
    // TODO(lubo): Block iterator!!
    pub fn iter_block(&mut self, mut matching: [Option<usize>; N]) -> impl Iterator<Item = &T> {
        todo!();
        [].into_iter()
        // let mut index = 0;
        // for i in (0..N).rev() {
        //     match matching[i] {
        //         Some(value) => index += value * self.dim_strides[i],
        //         None => {
        //             for a in 0..self.dims[i] {
        //                 matching[i] = Some(a);
        //                 // self.draw_block(matching, v);
        //             }
        //             // return;
        //         }
        //     }
        // }
        // // self.set_linear(index, v)
    }

    pub fn draw_block(&mut self, mut matching: [Option<usize>; N], v: T) {
        let mut index = 0;
        for i in (0..N).rev() {
            match matching[i] {
                Some(value) => index += value * self.dim_strides[i],
                None => {
                    for a in 0..self.dims[i] {
                        matching[i] = Some(a);
                        self.draw_block(matching, v);
                    }
                    return;
                }
            }
        }
        self.set_linear(index, v)
    }
}

impl<const C: usize, T> IterateNeighboursContext for ArrayNd<C, T> {}

impl<const C: usize, T: IterateNeighbours<()> + Copy, U> IterateNeighbours<ArrayNd<C, U>> for T
where
    ArrayNd<C, U>: LinearIndex<T>,
{
    fn neighbours(&self, _context: &ArrayNd<C, U>) -> Vec<Self> {
        self.neighbours(&())
            .into_iter()
            .filter(|x| _context.is_in_bounds(x))
            .collect()
    }
}

// TODO(lubo): Slices?
// impl<const C: usize, T: Copy> Display for ArrayNd<C, T> {
//     pub fn get_slice(&self, ) {

//     }
// }

impl<const C: usize, T: Display> Display for ArrayNd<C, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_size = self.data.len(); // 0..self.dims.iter().product()
        if C > 1 {
            let mut index = 0;
            while index < total_size {
                if C > 2 {
                    let slice: Vector<C, usize> = self.unindex(index).unwrap();
                    writeln!(f, "Slice = {}", slice)?;
                }

                for _y in 0..self.dims[1] {
                    for _x in 0..self.dims[0] {
                        write!(f, "{}", self.get_linear(index))?;
                        index += 1;
                    }
                    writeln!(f)?;
                }
            }
        } else {
            for x in 0..total_size {
                write!(f, "{}", self.get_linear(x))?;
            }
        }

        write!(f, "")
    }
}

// NOTE(lubo): Specific lower dimensional arrays

pub type Array2d<T> = ArrayNd<2, T>;
pub type Array3d<T> = ArrayNd<3, T>;
impl<T> Array2d<T> {
    pub fn width(&self) -> usize {
        self.dims[0]
    }
    pub fn height(&self) -> usize {
        self.dims[1]
    }
}
impl<T> Array3d<T> {
    pub fn width(&self) -> usize {
        self.dims[0]
    }
    pub fn height(&self) -> usize {
        self.dims[1]
    }
    pub fn depth(&self) -> usize {
        self.dims[2]
    }
}

pub type CharArray2d = Array2d<char>;

impl FromStr for CharArray2d {
    type Err = CharArrayParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cursor = std::io::Cursor::new(s);
        Self::from_read(cursor)
    }
}

#[derive(Debug)]
pub enum CharArrayParseError {
    InconsistentLineWidth(usize, usize, usize, usize),
    Io(std::io::Error),
}

impl Display for CharArrayParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharArrayParseError::InconsistentLineWidth(l1, w1, l2, w2) => write!(f, "Inconsistent line width. On line {l1} the width is {w1}, while on line {l2} the width is {w2}."),
            CharArrayParseError::Io(io) => write!(f, "IO error: {io}"),
        }
    }
}

impl CharArray2d {
    pub fn from_buffer<R: std::io::Read>(
        reader: BufReader<R>,
    ) -> Result<Self, CharArrayParseError> {
        let mut array2d_width_line_number = 0;
        let mut array2d_width = 0;

        let mut data = vec![];
        let mut height = 0;

        for (line_number, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(l) => l,
                Err(e) => return Err(CharArrayParseError::Io(e)),
            };
            let line_width = line.len();
            if array2d_width == 0 {
                array2d_width = line_width;
                array2d_width_line_number = line_number;
            } else if array2d_width != line_width && line_width != 0 {
                return Err(CharArrayParseError::InconsistentLineWidth(
                    array2d_width_line_number,
                    array2d_width,
                    line_number,
                    line_width,
                ));
            }
            if line_width > 0 {
                height += 1;
                data.extend(line.chars());
            }
        }

        Ok(Self {
            data,
            dims: [array2d_width, height],
            dim_strides: [1, array2d_width],
        })
    }

    pub fn from_read<R: std::io::Read>(reader: R) -> Result<Self, CharArrayParseError> {
        Self::from_buffer(BufReader::new(reader))
    }
}

impl<T: Copy> Array2d<T> {
    pub fn with_dimensions(width: usize, height: usize, default: T) -> Self {
        Self {
            data: iter::repeat(default).take(width * height).collect(),
            dims: [width, height],
            dim_strides: [1, width],
        }
    }

    pub fn shift_n_rows_down(&mut self, n: usize, default: T) {
        self.data.drain(..self.width() * n);
        self.data.extend(
            iter::repeat(default)
                .take(self.width() * n)
                .collect::<Vec<T>>(),
        );
    }
}

impl<T: Copy> Array3d<T> {
    pub fn with_dimensions(width: usize, height: usize, depth: usize, default: T) -> Self {
        Self {
            data: iter::repeat(default).take(width * height * depth).collect(),
            dims: [width, height, depth],
            dim_strides: [1, width, width * height],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::V2i32;

    use super::*;

    const EXAMPLE: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn parse_chararray2d() {
        let map: CharArray2d = EXAMPLE.parse().unwrap();
        assert_eq!(10, map.width());
        assert_eq!(10, map.height());
        assert_eq!(Some(&'4'), map.get(V2i32::from_xy(0, 0)));
    }

    #[test]
    fn fail_parse_chararray2d_inconsistent_line_width() {
        let bad = r#"
line 1
line 2
line 3+
"#;
        if let Err(CharArrayParseError::InconsistentLineWidth(l1, w1, l2, w2)) =
            bad.parse::<CharArray2d>()
        {
            assert!((1..=2).contains(&l1));
            assert_eq!(w1, 6);
            assert_eq!(l2, 3);
            assert_eq!(w2, 7);
        } else {
            panic!();
        }
    }
}
