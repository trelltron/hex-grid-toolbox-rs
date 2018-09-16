use super::coords::{CoordType, OffsetOddCoords, OffsetEvenCoords, CubeCoords};
use super::coords::alg::get_range_cube;
use super::coords::utils::Orientation;

/*
    Iterators that generate some common map shapes to initialize a grid
*/

// Square Offset Odd Grid

pub struct SquareFlatOffsetOddGridIterator<C: CoordType> {
    origin: C,
    width: i32,
    height: i32,
    x: i32,
    y: i32
}

impl<C: CoordType> SquareFlatOffsetOddGridIterator<C> {
    pub fn new(origin: C, width: i32, height: i32) -> Self {
        Self {
            origin,
            width,
            height,
            x: 0,
            y: 0
        }
    }
}

impl<C: CoordType + From<OffsetOddCoords>> Iterator for SquareFlatOffsetOddGridIterator<C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width || self.y >= self.height {
            return None
        }
        let res = C::from(OffsetOddCoords::new((self.x, self.y), self.origin.orientation()));
        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        Some(res)
    }
}

// Square Offset Even Grid

pub struct SquareFlatOffsetEvenGridIterator<C: CoordType> {
    origin: C,
    width: i32,
    height: i32,
    x: i32,
    y: i32
}

impl<C: CoordType> SquareFlatOffsetEvenGridIterator<C> {
    pub fn new(origin: C, width: i32, height: i32) -> Self {
        Self {
            origin,
            width,
            height,
            x: 0,
            y: 0
        }
    }
}

impl<C: CoordType + From<OffsetEvenCoords>> Iterator for SquareFlatOffsetEvenGridIterator<C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width || self.y >= self.height {
            return None
        }
        let res = C::from(OffsetEvenCoords::new((self.x, self.y), self.origin.orientation()));
        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        Some(res)
    }
}

// Hexagonal Centered Grid

pub struct HexagonalCenteredGridIterator<C: CoordType> {
    origin: (i32, i32, i32),
    orient: Orientation,
    range: i32,
    store: Option<Vec<C>>,
}

impl<C: CoordType> HexagonalCenteredGridIterator<C> {
    pub fn new(range: i32, orient: Orientation) -> Self {
        Self {
            origin: (0, 0, 0),
            orient,
            range,
            store: None
        }
    }
}

impl<C: CoordType + From<CubeCoords>> Iterator for HexagonalCenteredGridIterator<C> {
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        match self.store {
            Some(ref mut list) => list.pop(),
            None => {
                let mut list: Vec<C> = get_range_cube(self.origin, self.range).iter().map(|t| C::from(CubeCoords::new(*t, self.orient))).collect();
                let result = list.pop();
                self.store = Some(list);
                result
            }
        }
    }
}