use super::utils::{self, Orientation, HexShape};

/*
    Identity functions for 'converting' an item to itself
*/

// pub fn identity<T>(arg: T) -> T {
//     arg
// }

// pub fn identity_ignore<T, I>(arg: T, ignore: I) -> T {
//     arg
// }

/*
    Special rounding function (possibly move to algorithm file)
*/

pub fn cube_round((fx, fy, fz): (f32, f32, f32)) -> (i32, i32, i32) {
    let (rx, ry, rz) = (fx.round(), fy.round(), fz.round());

    let (dx, dy, dz) = ((rx - fx).abs(), (ry - fy).abs(), (rz - fz).abs());

    let (x, y, z) = match true {
        _ if dx > dy && dx > dz => (-ry - rz, ry, rz),
        _ if dy > dz => (rx, -rx - rz, rz),
        _ => (rx, ry, -rx - ry)
    };
    (x as i32, y as i32, z as i32)
}

/*
    Public API
*/

// To Pixel

pub fn offset_odd_to_pixel(input: (i32, i32), shape: HexShape) -> (f32, f32) {
    match shape {
        HexShape::FlatTop(radius) => utils::scale_2d_tuple(flat_offset_odd_to_pixel(input), radius),
        HexShape::PointyTop(radius) => utils::scale_2d_tuple(pointy_offset_odd_to_pixel(input), radius)
    }
}

pub fn offset_even_to_pixel(input: (i32, i32), shape: HexShape) -> (f32, f32) {
    match shape {
        HexShape::FlatTop(radius) => utils::scale_2d_tuple(flat_offset_even_to_pixel(input), radius),
        HexShape::PointyTop(radius) => utils::scale_2d_tuple(pointy_offset_even_to_pixel(input), radius)
    }
}

pub fn doubled_to_pixel(input: (i32, i32), shape: HexShape) -> (f32, f32) {
    match shape {
        HexShape::FlatTop(radius) => utils::scale_2d_tuple(flat_doubled_to_pixel(input), radius),
        HexShape::PointyTop(radius) => utils::scale_2d_tuple(pointy_doubled_to_pixel(input), radius)
    }
}

pub fn cube_to_pixel(input: (i32, i32, i32), shape: HexShape) -> (f32, f32) {
    match shape {
        HexShape::FlatTop(radius) => utils::scale_2d_tuple(flat_cube_to_pixel(input), radius),
        HexShape::PointyTop(radius) => utils::scale_2d_tuple(pointy_cube_to_pixel(input), radius)
    }
}

pub fn axial_to_pixel(input: (i32, i32), shape: HexShape) -> (f32, f32) {
    match shape {
        HexShape::FlatTop(radius) => utils::scale_2d_tuple(flat_axial_to_pixel(input), radius),
        HexShape::PointyTop(radius) => utils::scale_2d_tuple(pointy_axial_to_pixel(input), radius)
    }
}

// From Pixel

pub fn pixel_to_offset_odd(input: (f32, f32), shape: HexShape) -> (i32, i32) {
    match shape {
        HexShape::FlatTop(radius) => pixel_to_flat_offset_odd(utils::scale_2d_tuple(input, 1.0/radius)),
        HexShape::PointyTop(radius) => pixel_to_pointy_offset_odd(utils::scale_2d_tuple(input, 1.0/radius))
    }
}

pub fn pixel_to_offset_even(input: (f32, f32), shape: HexShape) -> (i32, i32) {
    match shape {
        HexShape::FlatTop(radius) => (pixel_to_flat_offset_even(utils::scale_2d_tuple(input, 1.0/radius))),
        HexShape::PointyTop(radius) => (pixel_to_pointy_offset_even(utils::scale_2d_tuple(input, 1.0/radius)))
    }
}

pub fn pixel_to_doubled(input: (f32, f32), shape: HexShape) -> (i32, i32) {
    match shape {
        HexShape::FlatTop(radius) => (pixel_to_flat_doubled(utils::scale_2d_tuple(input, 1.0/radius))),
        HexShape::PointyTop(radius) => (pixel_to_pointy_doubled(utils::scale_2d_tuple(input, 1.0/radius)))
    }
}

pub fn pixel_to_cube(input: (f32, f32), shape: HexShape) -> (i32, i32, i32) {
    match shape {
        HexShape::FlatTop(radius) => (pixel_to_flat_cube(utils::scale_2d_tuple(input, 1.0/radius))),
        HexShape::PointyTop(radius) => (pixel_to_pointy_cube(utils::scale_2d_tuple(input, 1.0/radius)))
    }
}

pub fn pixel_to_axial(input: (f32, f32), shape: HexShape) -> (i32, i32) {
    match shape {
        HexShape::FlatTop(radius) => (pixel_to_flat_axial(utils::scale_2d_tuple(input, 1.0/radius))),
        HexShape::PointyTop(radius) => (pixel_to_pointy_axial(utils::scale_2d_tuple(input, 1.0/radius)))
    }
}

// To Offset Odd

pub fn offset_even_to_offset_odd(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_offset_even_to_offset_odd(input),
        Orientation::Pointy => pointy_offset_even_to_offset_odd(input)
    }
}

pub fn doubled_to_offset_odd(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_doubled_to_offset_odd(input),
        Orientation::Pointy => pointy_doubled_to_offset_odd(input)
    }
}

pub fn cube_to_offset_odd(input: (i32, i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_cube_to_offset_odd(input),
        Orientation::Pointy => pointy_cube_to_offset_odd(input)
    }
}

pub fn axial_to_offset_odd(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_axial_to_offset_odd(input),
        Orientation::Pointy => pointy_axial_to_offset_odd(input)
    }
}

// To Offset Even

pub fn offset_odd_to_offset_even(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_offset_odd_to_offset_even(input),
        Orientation::Pointy => pointy_offset_odd_to_offset_even(input)
    }
}

pub fn doubled_to_offset_even(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_doubled_to_offset_even(input),
        Orientation::Pointy => pointy_doubled_to_offset_even(input)
    }
}

pub fn cube_to_offset_even(input: (i32, i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_cube_to_offset_even(input),
        Orientation::Pointy => pointy_cube_to_offset_even(input)
    }
}

pub fn axial_to_offset_even(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_axial_to_offset_even(input),
        Orientation::Pointy => pointy_axial_to_offset_even(input)
    }
}

// To Doubled

pub fn offset_odd_to_doubled(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_offset_odd_to_doubled(input),
        Orientation::Pointy => pointy_offset_odd_to_doubled(input)
    }
}

pub fn offset_even_to_doubled(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_offset_even_to_doubled(input),
        Orientation::Pointy => pointy_offset_even_to_doubled(input)
    }
}

pub fn cube_to_doubled(input: (i32, i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_cube_to_doubled(input),
        Orientation::Pointy => pointy_cube_to_doubled(input)
    }
}

pub fn axial_to_doubled(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_axial_to_doubled(input),
        Orientation::Pointy => pointy_axial_to_doubled(input)
    }
}

// To Cube

pub fn offset_odd_to_cube(input: (i32, i32), orientation: Orientation) -> (i32, i32, i32) {
    match orientation {
        Orientation::Flat => flat_offset_odd_to_cube(input),
        Orientation::Pointy => pointy_offset_odd_to_cube(input)
    }
}

pub fn offset_even_to_cube(input: (i32, i32), orientation: Orientation) -> (i32, i32, i32) {
    match orientation {
        Orientation::Flat => flat_offset_even_to_cube(input),
        Orientation::Pointy => pointy_offset_even_to_cube(input)
    }
}

pub fn doubled_to_cube(input: (i32, i32), orientation: Orientation) -> (i32, i32, i32) {
    match orientation {
        Orientation::Flat => flat_doubled_to_cube(input),
        Orientation::Pointy => pointy_doubled_to_cube(input)
    }
}

// To Axial

pub fn offset_odd_to_axial(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_offset_odd_to_axial(input),
        Orientation::Pointy => pointy_offset_odd_to_axial(input)
    }
}

pub fn offset_even_to_axial(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_offset_even_to_axial(input),
        Orientation::Pointy => pointy_offset_even_to_axial(input)
    }
}

pub fn doubled_to_axial(input: (i32, i32), orientation: Orientation) -> (i32, i32) {
    match orientation {
        Orientation::Flat => flat_doubled_to_axial(input),
        Orientation::Pointy => pointy_doubled_to_axial(input)
    }
}


// Cube <-> Axial conversion

pub fn cube_to_axial_orient((x, _y, z): (i32, i32, i32), _o: Orientation) -> (i32, i32) {
    (x, z)
}

pub fn axial_to_cube_orient((q, r): (i32, i32), _o: Orientation) -> (i32, i32, i32) {
    (q, -q - r, r)
}

pub fn cube_to_axial((x, _y, z): (i32, i32, i32)) -> (i32, i32) {
    (x, z)
}

pub fn axial_to_cube((q, r): (i32, i32)) -> (i32, i32, i32) {
    (q, -q - r, r)
}


/*
    To Pixel
*/

// Offset

fn flat_offset_odd_to_pixel((x, y): (i32, i32)) -> (f32, f32) {
    let bitand_check = (x & 1) as f32;
    let (x, y) = (x as f32, y as f32);
    let px =  x * 3.0/2.0;
    let py =  3f32.sqrt() * (y + (0.5 * bitand_check));
    (px, py)
}

fn pointy_offset_odd_to_pixel((x, y): (i32, i32)) -> (f32, f32) {
    let bitand_check = (y & 1) as f32;
    let (x, y) = (x as f32, y as f32);
    let px = 3f32.sqrt() * (x + (0.5 * bitand_check));
    let py = y * 3.0/2.0;
    (px, py)
}

fn flat_offset_even_to_pixel((x, y): (i32, i32)) -> (f32, f32) {
    let bitand_check = (x & 1) as f32;
    let (x, y) = (x as f32, y as f32);
    let px = x * 3.0/2.0;
    let py = 3f32.sqrt() * (y - (0.5 * bitand_check));
    (px, py)
}

fn pointy_offset_even_to_pixel((x, y): (i32, i32)) -> (f32, f32) {
    let bitand_check = (y & 1) as f32;
    let (x, y) = (x as f32, y as f32);
    let px =  3f32.sqrt() * (x - (0.5 * bitand_check));
    let py =  y * 3.0/2.0;
    (px, py)
}

// Doubled

fn flat_doubled_to_pixel((x, y): (i32, i32)) -> (f32, f32) {
    let (x, y) = (x as f32, y as f32);
    let px = x * 3.0/2.0;
    let py = y * 3f32.sqrt()/2.0;
    (px, py)
}

fn pointy_doubled_to_pixel((x, y): (i32, i32)) -> (f32, f32) {
    let (x, y) = (x as f32, y as f32);
    let px = x * 3f32.sqrt()/2.0;
    let py = y * 3.0/2.0;
    (px, py)
}

// Cube

fn flat_cube_to_pixel((x, _y, z): (i32, i32, i32)) -> (f32, f32) {
    flat_axial_to_pixel((x, z))
}

fn pointy_cube_to_pixel((x, _y, z): (i32, i32, i32)) -> (f32, f32) {
    pointy_axial_to_pixel((x, z))
}

// Axial

fn flat_axial_to_pixel((q, r): (i32, i32)) -> (f32, f32) {
    let (q, r) = (q as f32, r as f32);
    let px = q * 3.0/2.0;
    let py = (q * 3f32.sqrt()/2.0) + (r * 3f32.sqrt());
    (px, py)
}

fn pointy_axial_to_pixel((q, r): (i32, i32)) -> (f32, f32) {
    let (q, r) = (q as f32, r as f32);
    let px = (q * 3f32.sqrt()) + (r * 3f32.sqrt()/2.0);
    let py = r * 3.0/2.0;
    (px, py)
}


/*
    From Pixel
*/

// Offset 

fn pixel_to_flat_offset_even((px, py): (f32, f32)) -> (i32, i32) {
    flat_cube_to_offset_even(pixel_to_flat_cube((px, py)))
}

fn pixel_to_flat_offset_odd((px, py): (f32, f32)) -> (i32, i32) {
    flat_cube_to_offset_odd(pixel_to_flat_cube((px, py)))
}

fn pixel_to_pointy_offset_even((px, py): (f32, f32)) -> (i32, i32) {
    pointy_cube_to_offset_even(pixel_to_pointy_cube((px, py)))
}

fn pixel_to_pointy_offset_odd((px, py): (f32, f32)) -> (i32, i32) {
    pointy_cube_to_offset_odd(pixel_to_pointy_cube((px, py)))
}

// Doubled

fn pixel_to_flat_doubled((px, py): (f32, f32)) -> (i32, i32) {
    flat_cube_to_doubled(pixel_to_flat_cube((px, py)))
}

fn pixel_to_pointy_doubled((px, py): (f32, f32)) -> (i32, i32) {
    pointy_cube_to_doubled(pixel_to_pointy_cube((px, py)))
}


// Cube

fn pixel_to_flat_cube((px, py): (f32, f32)) -> (i32, i32, i32) {
    let x =  px * 2.0/3.0;
    let z = (px * -1.0/3.0) + (py * 3f32.sqrt()/3.0);
    cube_round((x, -x - z, z))
}

fn pixel_to_pointy_cube((px, py): (f32, f32)) -> (i32, i32, i32) {
    let x = (px * 3f32.sqrt()/3.0) - (py * 1.0/3.0);
    let z = py * 2.0/3.0;
    cube_round((x, -x - z, z))
}

// Axial

fn pixel_to_flat_axial((px, py): (f32, f32)) -> (i32, i32) {
    cube_to_axial(pixel_to_flat_cube((px, py)))
}

fn pixel_to_pointy_axial((px, py): (f32, f32)) -> (i32, i32) {
    cube_to_axial(pixel_to_pointy_cube((px, py)))
}

/*
    To Offset
*/

// Offset to Offset 

fn flat_offset_even_to_offset_odd((x, y): (i32, i32)) -> (i32, i32) {
    (x, match x % 2 { 0 => y, _ => y - 1 })
}

fn flat_offset_odd_to_offset_even((x, y): (i32, i32)) -> (i32, i32) {
    (x, match x % 2 { 0 => y, _ => y + 1})
}

fn pointy_offset_even_to_offset_odd((x, y): (i32, i32)) -> (i32, i32) {
    (match y % 2 { 0 => x, _ => x - 1}, y)
}

fn pointy_offset_odd_to_offset_even((x, y): (i32, i32)) -> (i32, i32) {
    (match y % 2 { 0 => x, _ => x + 1}, y)
}

// Doubled

fn flat_doubled_to_offset_odd((x, y): (i32, i32)) -> (i32, i32) {
    (x, match x % 2 { 0 => y/2, _ => (y - 1)/2 })
}

fn flat_doubled_to_offset_even((x, y): (i32, i32)) -> (i32, i32) {
    (x, match x % 2 { 0 => y/2, _ => (y + 1)/2 })
}

fn pointy_doubled_to_offset_odd((x, y): (i32, i32)) -> (i32, i32) {
    (match y % 2 { 0 => x/2, _ => (x - 1)/2 }, y)
}

fn pointy_doubled_to_offset_even((x, y): (i32, i32)) -> (i32, i32) {
    (match y % 2 { 0 => x/2, _ => (x + 1)/2 }, y)
}


// Cube

fn flat_cube_to_offset_odd((x, _y, z): (i32, i32, i32)) -> (i32, i32) {
    (x, z + ((x - (x & 1)) / 2))
}

fn flat_cube_to_offset_even((x, _y, z): (i32, i32, i32)) -> (i32, i32) {
    (x, z + ((x + (x & 1)) / 2))
}

fn pointy_cube_to_offset_odd((x, _y, z): (i32, i32, i32)) -> (i32, i32) {
    (x + ((z - (z & 1)) / 2), z)

}

fn pointy_cube_to_offset_even((x, _y, z): (i32, i32, i32)) -> (i32, i32) {
    ((x + (z + (z & 1)) / 2), z)
}

// Axial

fn flat_axial_to_offset_odd((q, r): (i32, i32)) -> (i32, i32) {
    flat_cube_to_offset_odd(axial_to_cube((q, r)))
}

fn flat_axial_to_offset_even((q, r): (i32, i32)) -> (i32, i32) {
    flat_cube_to_offset_even(axial_to_cube((q, r)))
}

fn pointy_axial_to_offset_odd((q, r): (i32, i32)) -> (i32, i32) {
    pointy_cube_to_offset_odd(axial_to_cube((q, r)))
}

fn pointy_axial_to_offset_even((q, r): (i32, i32)) -> (i32, i32) {
    pointy_cube_to_offset_even(axial_to_cube((q, r)))
}

/*
    To Doubled
*/

// Offset

fn flat_offset_odd_to_doubled((x, y): (i32, i32)) -> (i32, i32) {
    (x, match x % 2 { 0 => y * 2, _ => (y * 2) + 1})
}

fn flat_offset_even_to_doubled((x, y): (i32, i32)) -> (i32, i32) {
    (x, match x % 2 { 0 => y * 2, _ => (y * 2) - 1})
}

fn pointy_offset_odd_to_doubled((x, y): (i32, i32)) -> (i32, i32) {
    (match y % 2 { 0 => x * 2, _ => (x * 2) + 1}, y)
}

fn pointy_offset_even_to_doubled((x, y): (i32, i32)) -> (i32, i32) {
    (match y % 2 { 0 => x * 2, _ => (x * 2) - 1}, y)
}

// Cube

fn flat_cube_to_doubled((x, _y, z): (i32, i32, i32)) -> (i32, i32) {
    (x, (2 * z) + x)
}

fn pointy_cube_to_doubled((x, _y, z): (i32, i32, i32)) -> (i32, i32) {
    ((2 * x) + z, z)
}

// Axial

fn flat_axial_to_doubled((q, r): (i32, i32)) -> (i32, i32) {
    flat_cube_to_doubled(axial_to_cube((q, r)))
}

fn pointy_axial_to_doubled((q, r): (i32, i32)) -> (i32, i32) {
    pointy_cube_to_doubled(axial_to_cube((q, r)))
}


/*
    to cube
*/

// Offset

fn flat_offset_odd_to_cube((x, y): (i32, i32)) -> (i32, i32, i32) {
    let z = y - ((x - (x & 1)) / 2);
    (x, -x - z, z)
}

fn flat_offset_even_to_cube((x, y): (i32, i32)) -> (i32, i32, i32) {
    let z = y - ((x + (x & 1)) / 2);
    (x, -x - z, z)
}

fn pointy_offset_odd_to_cube((x, y): (i32, i32)) -> (i32, i32, i32) {
    let x = x - ((y - (y & 1)) / 2);
    (x, -x - y, y)
}

fn pointy_offset_even_to_cube((x, y): (i32, i32)) -> (i32, i32, i32) {
    let x = x - ((y + (y & 1)) / 2);
    (x, -x - y, y)
}

// Doubled

fn flat_doubled_to_cube((x, y): (i32, i32)) -> (i32, i32, i32) {
    let z = (y - x) / 2;
    (x, -x - z, z)
}

fn pointy_doubled_to_cube((x, y): (i32, i32)) -> (i32, i32, i32) {
    let x = (x - y) / 2;
    (x, -x - y, y)
}

/*
    to axial
*/

// Offset

fn flat_offset_odd_to_axial((x, y): (i32, i32)) -> (i32, i32) {
    cube_to_axial(flat_offset_odd_to_cube((x, y)))
}

fn flat_offset_even_to_axial((x, y): (i32, i32)) -> (i32, i32) {
    cube_to_axial(flat_offset_even_to_cube((x, y)))
}

fn pointy_offset_odd_to_axial((x, y): (i32, i32)) -> (i32, i32) {
    cube_to_axial(pointy_offset_odd_to_cube((x, y)))
}

fn pointy_offset_even_to_axial((x, y): (i32, i32)) -> (i32, i32) {
    cube_to_axial(pointy_offset_even_to_cube((x, y)))
}

// Doubled

fn flat_doubled_to_axial((x, y): (i32, i32)) -> (i32, i32) {
    cube_to_axial(flat_doubled_to_cube((x, y)))
}

fn pointy_doubled_to_axial((x, y): (i32, i32)) -> (i32, i32) {
    cube_to_axial(pointy_doubled_to_cube((x, y)))
}


/*
    TESTING!!!!
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    /*
        To Offset
    */

    #[test]
    fn test_offset_to_offset() {
        assert_eq!(flat_offset_even_to_offset_odd((0, 0)), (0, 0));
        assert_eq!(flat_offset_even_to_offset_odd((5, 3)), (5, 2));
        assert_eq!(flat_offset_even_to_offset_odd((6, 4)), (6, 4));

        assert_eq!(flat_offset_odd_to_offset_even((0, 0)), (0, 0));
        assert_eq!(flat_offset_odd_to_offset_even((5, 2)), (5, 3));
        assert_eq!(flat_offset_odd_to_offset_even((6, 4)), (6, 4));

        assert_eq!(pointy_offset_even_to_offset_odd((0, 0)), (0, 0));
        assert_eq!(pointy_offset_even_to_offset_odd((3, 5)), (2, 5));
        assert_eq!(pointy_offset_even_to_offset_odd((5, 4)), (5, 4));

        assert_eq!(pointy_offset_odd_to_offset_even((0, 0)), (0, 0));
        assert_eq!(pointy_offset_odd_to_offset_even((2, 5)), (3, 5));
        assert_eq!(pointy_offset_odd_to_offset_even((5, 4)), (5, 4));
    }

    #[test]
    fn test_doubled_to_offset() {
        assert_eq!(flat_doubled_to_offset_odd((0, 0)), (0, 0));
        assert_eq!(flat_doubled_to_offset_odd((5, 5)), (5, 2));
        assert_eq!(flat_doubled_to_offset_odd((6, 6)), (6, 3));

        assert_eq!(flat_doubled_to_offset_even((0, 0)), (0, 0));
        assert_eq!(flat_doubled_to_offset_even((5, 5)), (5, 3));
        assert_eq!(flat_doubled_to_offset_even((6, 6)), (6, 3));

        assert_eq!(pointy_doubled_to_offset_odd((0, 0)), (0, 0));
        assert_eq!(pointy_doubled_to_offset_odd((4, 4)), (2, 4));
        assert_eq!(pointy_doubled_to_offset_odd((5, 5)), (2, 5));

        assert_eq!(pointy_doubled_to_offset_even((0, 0)), (0, 0));
        assert_eq!(pointy_doubled_to_offset_even((4, 4)), (2, 4));
        assert_eq!(pointy_doubled_to_offset_even((5, 5)), (3, 5));
    }

    #[test]
    fn test_cube_to_offset() {

        assert_eq!(53 & 1, 1);
        assert_eq!(52 & 1, 0);
        assert_eq!(flat_cube_to_offset_odd((0, 0, 0)), (0, 0));
        assert_eq!(flat_cube_to_offset_odd((3, -3, 0)), (3, 1));
        assert_eq!(flat_cube_to_offset_odd((2, -3, 1)), (2, 2));

        assert_eq!(flat_cube_to_offset_even((0, 0, 0)), (0, 0));
        assert_eq!(flat_cube_to_offset_even((3, -3, 0)), (3, 2));
        assert_eq!(flat_cube_to_offset_even((2, -3, 1)), (2, 2));

        assert_eq!(pointy_cube_to_offset_odd((0, 0, 0)), (0, 0));
        assert_eq!(pointy_cube_to_offset_odd((0, -3, 3)), (1, 3));
        assert_eq!(pointy_cube_to_offset_odd((1, -3, 2)), (2, 2));

        assert_eq!(pointy_cube_to_offset_even((0, 0, 0)), (0, 0));
        assert_eq!(pointy_cube_to_offset_even((0, -3, 3)), (2, 3));
        assert_eq!(pointy_cube_to_offset_even((1, -3, 2)), (2, 2));
    }

    /*
        To Doubled
    */

    #[test]
    fn test_offset_to_doubled() {
        assert_eq!(flat_offset_odd_to_doubled((0, 0)), (0, 0));
        assert_eq!(flat_offset_odd_to_doubled((5, 2)), (5, 5));
        assert_eq!(flat_offset_odd_to_doubled((6, 3)), (6, 6));

        assert_eq!(flat_offset_even_to_doubled((0, 0)), (0, 0));
        assert_eq!(flat_offset_even_to_doubled((5, 3)), (5, 5));
        assert_eq!(flat_offset_even_to_doubled((6, 3)), (6, 6));

        assert_eq!(pointy_offset_odd_to_doubled((0, 0)), (0, 0));
        assert_eq!(pointy_offset_odd_to_doubled((2, 4)), (4, 4));
        assert_eq!(pointy_offset_odd_to_doubled((2, 5)), (5, 5));

        assert_eq!(pointy_offset_even_to_doubled((0, 0)), (0, 0));
        assert_eq!(pointy_offset_even_to_doubled((2, 4)), (4, 4));
        assert_eq!(pointy_offset_even_to_doubled((3, 5)), (5, 5));
    }

    #[test]
    fn test_cube_to_doubled() {
        assert_eq!(flat_cube_to_doubled((0, 0, 0)), (0, 0));
        assert_eq!(flat_cube_to_doubled((2, -3, 1)), (2, 4));
        assert_eq!(flat_cube_to_doubled((3, -2, -1)), (3, 1));

        assert_eq!(pointy_cube_to_doubled((0, 0, 0)), (0, 0));
        assert_eq!(pointy_cube_to_doubled((-1, -2, 3)), (1, 3));
        assert_eq!(pointy_cube_to_doubled((1, -3, 2)), (4, 2));
    }

    /*
        To Cube
    */

    #[test]
    fn test_offset_to_cube() {
        assert_eq!(flat_offset_odd_to_cube((0, 0)), (0, 0, 0));
        assert_eq!(flat_offset_odd_to_cube((3, 1)), (3, -3, 0));
        assert_eq!(flat_offset_odd_to_cube((2, 2)), (2, -3, 1));

        assert_eq!(flat_offset_even_to_cube((0, 0)), (0, 0, 0));
        assert_eq!(flat_offset_even_to_cube((3, 2)), (3, -3, 0));
        assert_eq!(flat_offset_even_to_cube((2, 2)), (2, -3, 1));

        assert_eq!(pointy_offset_odd_to_cube((0, 0)), (0, 0, 0));
        assert_eq!(pointy_offset_odd_to_cube((1, 3)), (0, -3, 3));
        assert_eq!(pointy_offset_odd_to_cube((2, 2)), (1, -3, 2));

        assert_eq!(pointy_offset_even_to_cube((0, 0)), (0, 0, 0));
        assert_eq!(pointy_offset_even_to_cube((2, 3)), (0, -3, 3));
        assert_eq!(pointy_offset_even_to_cube((2, 2)), (1, -3, 2));
    }

    #[test]
    fn test_doubled_to_cube() {
        assert_eq!(flat_doubled_to_cube((0, 0)), (0, 0, 0));
        assert_eq!(flat_doubled_to_cube((2, 4)), (2, -3, 1));
        assert_eq!(flat_doubled_to_cube((3, 1)), (3, -2, -1));

        assert_eq!(pointy_doubled_to_cube((0, 0)), (0, 0, 0));
        assert_eq!(pointy_doubled_to_cube((1, 3)), (-1, -2, 3));
        assert_eq!(pointy_doubled_to_cube((4, 2)), (1, -3, 2));
    }

    #[test]
    fn test_axial_to_cube() {
        assert_eq!(axial_to_cube((0, 0)), (0, 0, 0));
        assert_eq!(axial_to_cube((-30, 14)), (-30, 16, 14));
        assert_eq!(axial_to_cube((4, 6)), (4, -10, 6));
    }

    /*
        To Axial
    */

    #[test]
    fn test_cube_to_axial() {
        assert_eq!(cube_to_axial((0, 0, 0)), (0, 0));
        assert_eq!(cube_to_axial((-30, 16, 14)), (-30, 14));
        assert_eq!(cube_to_axial((4, -10, 6)), (4, 6));
    }
}