use std::cmp::{max, min};

fn sum_tuple_2((a, b): (i32, i32), (x, y): (i32, i32)) -> (i32, i32) {
    (a + x, b + y)
}

fn sum_tuple_3((a, b, c): (i32, i32, i32), (x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
    (a + x, b + y, c + z)
}

/*
    Neighbors
*/

const CUBE_DIRECTIONS: [(i32, i32, i32); 6] = [
    (1, -1, 0), (1, 0, -1), (0, 1, -1),
    (-1, 1, 0), (-1, 0, 1), (0, -1, 1)
];

const AXIAL_DIRECTIONS: [(i32, i32); 6] = [
    (1, 0), (1, -1), (0, -1),
    (-1, 0), (-1, 1), (0, 1)
];

// Offset

// fn get_neighbours_offset_odd(){}
// fn get_neighbours_offset_even(){}

// Doubled

// fn get_neighbours_doubled(){}

// Cube

fn get_neighbour_cube(input: (i32, i32, i32), direction: usize) -> (i32, i32, i32) {
    sum_tuple_3(input, CUBE_DIRECTIONS[direction % 6])
}

pub fn get_neighbours_cube(input: (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        get_neighbour_cube(input, 0),
        get_neighbour_cube(input, 1),
        get_neighbour_cube(input, 2),
        get_neighbour_cube(input, 3),
        get_neighbour_cube(input, 4),
        get_neighbour_cube(input, 5)
    ]
}

// Axial

fn get_neighbour_axial(input: (i32, i32), direction: usize) -> (i32, i32) {
    sum_tuple_2(input, AXIAL_DIRECTIONS[direction % 6])
}

pub fn get_neighbours_axial(input: (i32, i32)) -> [(i32, i32); 6] {
    [
        get_neighbour_axial(input, 0),
        get_neighbour_axial(input, 1),
        get_neighbour_axial(input, 2),
        get_neighbour_axial(input, 3),
        get_neighbour_axial(input, 4),
        get_neighbour_axial(input, 5)
    ]
}

/*
    Distances
*/


/*
    Ranges (All hexes within x distance from the argument)
*/

// Cube

pub fn get_range_cube(input: (i32, i32, i32), range: i32) -> Vec<(i32, i32, i32)> {
    let mut results = Vec::new();
    let range = range.abs();

    for x in -range..(range + 1) {
        for y in max(-range, -range-x)..(min(range, range-x) + 1) {
            results.push(sum_tuple_3(input, (x, y, -x-y)));
        }
    }
    results
}
