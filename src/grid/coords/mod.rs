mod convert;
pub mod alg;
pub mod utils;

use self::utils::{Orientation, HexShape};

/*
    Arbretrary 'Key' type for use in dictionary storage
    Must be unique for every coordinate within a given system
*/

#[derive(Clone, Copy, Hash, Eq, PartialEq, Default)]
pub struct CoordKey(i32, i32, i32);

impl From<(i32, i32)> for CoordKey {
    fn from((x, y): (i32, i32)) -> CoordKey {
        CoordKey(x, y, 0)
    }
}

impl From<(i32, i32, i32)> for CoordKey {
    fn from((x, y, z): (i32, i32, i32)) -> CoordKey {
        CoordKey(x, y, z)
    }
}


/*
    Pixel position structs/methods for mapping onto a screen
*/

pub struct PixelCoord { x: f32, y: f32, shape: HexShape }

impl PixelCoord {
    pub fn new((x, y): (f32, f32), shape: HexShape) -> Self { Self { x, y, shape } }
    pub fn get(&self) -> (f32, f32) { (self.x, self.y) }

    pub fn corners(&self) -> [(f32, f32); 6] {
        self.shape.corners(self.get())
    }
}

/*
    Coordinate system definitions
*/

// Flat systems
pub struct OffsetOddCoords { x: i32, y: i32, orientation: Orientation }
pub struct OffsetEvenCoords { x: i32, y: i32, orientation: Orientation }
pub struct DoubledCoords { x: i32, y: i32, orientation: Orientation }
pub struct CubeCoords { x: i32, y: i32, z: i32, orientation: Orientation }
pub struct AxialCoords { q: i32, r: i32, orientation: Orientation }

/*
    Coordinate type traits
*/

pub trait CoordType: Sized {
    type TupleRep;

    fn new(Self::TupleRep, Orientation) -> Self;
    fn get(&self) -> Self::TupleRep;
    fn get_key(&self) -> CoordKey;

    fn from_pixel(p: &PixelCoord) -> Self;

    fn to_pixel(&self, radius: f32) -> PixelCoord;

    fn orientation(&self) -> Orientation;
}

/*
    Coordinate core method definitions
*/

macro_rules! impl_coord_type_trait {
    ($type:ident, $tuple_rep:ty, ($($value_name:ident),*), 
     $from_pixel_fn:ident, $to_pixel_fn:ident) => {
        impl CoordType for $type {
            type TupleRep = $tuple_rep;

            fn new(($($value_name),*): $tuple_rep, orientation: Orientation) -> Self { 
                Self { $($value_name),*, orientation } 
            }
            fn get(&self) -> $tuple_rep { ($(self.$value_name),*) }
            fn get_key(&self) -> CoordKey { CoordKey::from(($(self.$value_name),*)) }

            fn from_pixel(p: &PixelCoord) -> Self {
                Self::new(convert::$from_pixel_fn(p.get(), p.shape), p.shape.orient())
            }

            fn to_pixel(&self, radius: f32) -> PixelCoord {
                let shape = HexShape::new(radius, self.orientation);
                PixelCoord::new(convert::$to_pixel_fn(self.get(), shape), shape)
            }

            fn orientation(&self) -> Orientation {
                self.orientation
            }
        }
    }
}

impl_coord_type_trait!(OffsetOddCoords, (i32, i32), (x, y),
    pixel_to_offset_odd, offset_odd_to_pixel);

impl_coord_type_trait!(OffsetEvenCoords, (i32, i32), (x, y),
    pixel_to_offset_even, offset_even_to_pixel);

impl_coord_type_trait!(DoubledCoords, (i32, i32), (x, y),
    pixel_to_doubled, doubled_to_pixel);

impl_coord_type_trait!(CubeCoords, (i32, i32, i32), (x, y, z),
    pixel_to_cube, cube_to_pixel);

impl_coord_type_trait!(AxialCoords, (i32, i32), (q, r),
    pixel_to_axial, axial_to_pixel);

/*
    Define type conversions with from/into syntax
*/


macro_rules! impl_from_traits {
    ($T:ident, $(($from_type:ident, $f:ident)),*) => {
        $(
            impl<'a> From<&'a $from_type> for $T {
                fn from(input: &'a $from_type) -> Self {
                    Self::new(convert::$f(input.get(), input.orientation), input.orientation)
                }
            }
        )*

        impl<'a> From<&'a PixelCoord> for $T {
            fn from(input: &'a PixelCoord) -> Self {
                Self::from_pixel(input)
            }
        }

        // impl<'a> From<&'a $T> for PixelCoord {
        //     fn from(input: &'a $T) -> PixelCoord {
        //         $T::to_relative_pixel(input)
        //     }
        // }
    }
}

impl_from_traits!(OffsetOddCoords, (OffsetEvenCoords, offset_even_to_offset_odd),
    (DoubledCoords, doubled_to_offset_odd), (CubeCoords, cube_to_offset_odd), (AxialCoords, axial_to_offset_odd));

impl_from_traits!(OffsetEvenCoords, (OffsetOddCoords, offset_odd_to_offset_even),
    (DoubledCoords, doubled_to_offset_even), (CubeCoords, cube_to_offset_even), (AxialCoords, axial_to_offset_even));

impl_from_traits!(DoubledCoords, (OffsetOddCoords, offset_odd_to_doubled),
    (OffsetEvenCoords, offset_even_to_doubled), (CubeCoords, cube_to_doubled), (AxialCoords, axial_to_doubled));

impl_from_traits!(CubeCoords, (OffsetOddCoords, offset_odd_to_cube),
    (OffsetEvenCoords, offset_even_to_cube), (DoubledCoords, doubled_to_cube), (AxialCoords, axial_to_cube_orient));

impl_from_traits!(AxialCoords, (OffsetOddCoords, offset_odd_to_axial),
    (OffsetEvenCoords, offset_even_to_axial), (DoubledCoords, doubled_to_axial), (CubeCoords, cube_to_axial_orient));

/*
    TESTING!!!!
*/

#[cfg(test)]
mod tests {
    use super::*;

    /*
        Type Conversion Tests
    */
    #[test]
    fn test_from_conversions() {
        let input: OffsetOddCoords = OffsetOddCoords::new((5, 8), Orientation::Flat);
        let output: OffsetEvenCoords = OffsetEvenCoords::from(&input);
        assert_eq!(input.x, output.x);

        let input: OffsetEvenCoords = OffsetEvenCoords::new((5, 8), Orientation::Flat);
        let output: OffsetOddCoords = OffsetOddCoords::from(&input);
        assert_eq!(input.x, output.x);
    }

    #[test]
    fn test_into_conversions() {
        let input: OffsetOddCoords = OffsetOddCoords::new((5, 8), Orientation::Flat);
        let output: OffsetEvenCoords = (&input).into();
        assert_eq!(input.x, output.x);

        let input: OffsetEvenCoords = OffsetEvenCoords::new((5, 8), Orientation::Flat);
        let output: OffsetOddCoords = (&input).into();
        assert_eq!(input.x, output.x);
    }
}
