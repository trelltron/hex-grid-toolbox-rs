use std::collections::HashMap;
use std::marker::PhantomData;

pub mod coords;
pub mod tile;
pub mod initializers;

use self::tile::{HexTile};
use self::coords::utils::HexShape;

// Need to import CoordType to use the trait's function on the apllicable objects
use self::coords::{CoordType, CoordKey};

pub trait HexGridDataWrapper<C: CoordType, D> {
    fn new() -> Self;
    fn initialize<I>(&mut self, init: I, new_data: &Fn() -> D) 
        where I: Iterator<Item=C>;

    fn borrow_tile_by_key(&self, key: CoordKey) -> Option<&HexTile<C, D>>;
    fn mut_borrow_tile_by_key(&mut self, key: CoordKey) -> Option<&mut HexTile<C, D>>;

    fn mutate_all_data(&mut self, f: &Fn(&mut D));
}

// pub struct HexGrid2DVecWrapper<C: CoordType, D> {
//     tiles: Vec<Vec<HexTile<C, D>>>
// }

// impl<C: CoordType, D> HexGridDataWrapper<C, D> for HexGrid2DVecWrapper<C, D> {
//     fn borrow_tile_by_key(&self, key: CoordKey) -> Option<&HexTile<C, D>> {
//         let (x, y) = key.to_array_idxs();
//         if x >= self.tiles.len() || y >= self.tiles[x].len() {
//             return None
//         }
//         Some(&self.tiles[x][y])
//     }

//     fn mut_borrow_tile_by_key(&mut self, key: CoordKey) -> Option<&mut HexTile<C, D>> {
//         let (x, y) = key.to_array_idxs();
//         if x >= self.tiles.len() || y >= self.tiles[x].len() {
//             return None
//         }
//         Some(&mut self.tiles[x][y])
//     }

//     fn mutate_all_data(&mut self, f: &Fn(&mut D)) {
//         for col in self.tiles.iter_mut() {
//             for t in col.iter_mut() {
//                 t.mutate_data(f);
//             }
//         }
//     }
// }


pub struct HexGridHashMapWrapper<C: CoordType, D> {
    tiles: HashMap<CoordKey, HexTile<C, D>>
}

impl<C: CoordType, D> HexGridDataWrapper<C, D> for HexGridHashMapWrapper<C, D> {
    fn new() -> Self {
        Self {
            tiles: HashMap::new()
        }
    }

    fn initialize<I>(&mut self, init: I, new_data: &Fn() -> D) 
            where I: Iterator<Item=C> {
        for coord in init {
            self.tiles.insert(coord.get_key(), HexTile::new(coord, new_data()));
        }
    }

    fn borrow_tile_by_key(&self, key: CoordKey) -> Option<&HexTile<C, D>> {
        self.tiles.get(&key)
    }

    fn mut_borrow_tile_by_key(&mut self, key: CoordKey) -> Option<&mut HexTile<C, D>> {
        self.tiles.get_mut(&key)
    }

    fn mutate_all_data(&mut self, f: &Fn(&mut D)) {
        for (_, tile) in self.tiles.iter_mut() {
            tile.mutate_data(f);
        }
    }
}

impl<'a, C: CoordType, D> IntoIterator for &'a HexGridHashMapWrapper<C, D> {
    type Item = &'a HexTile<C, D>;
    type IntoIter = ::std::collections::hash_map::Values<'a, CoordKey, HexTile<C, D>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.values()
    }
}

/*
    Define outer grid construct
*/

pub struct HexGrid<C: CoordType, D, W: HexGridDataWrapper<C, D>> {
    pub wrapper: W,
    hexshape: HexShape,
    _c: PhantomData<C>,
    _d: PhantomData<D>
}

impl<C: CoordType, D, W: HexGridDataWrapper<C, D>> HexGrid<C, D, W> {
    pub fn new(hexshape: HexShape, init: Option<&Fn() -> W>) -> Self {
        match init {
            Some(f) => Self {
                wrapper: f(),
                hexshape,
                _c: PhantomData,
                _d: PhantomData
            },
            None => Self {
                wrapper: W::new(),
                hexshape,
                _c: PhantomData,
                _d: PhantomData
            }
        }
    }

    pub fn initialize<I>(&mut self, init: I, new_data: &Fn() -> D) 
            where I: Iterator<Item=C> {
        self.wrapper.initialize(init, new_data)
    }

    pub fn borrow_tile_by_key(&self, key: CoordKey) -> Option<&HexTile<C, D>> {
        self.wrapper.borrow_tile_by_key(key)
    }

    pub fn mut_borrow_tile_by_key(&mut self, key: CoordKey) -> Option<&mut HexTile<C, D>> {
        self.wrapper.mut_borrow_tile_by_key(key)
    }

    pub fn borrow_data_by_key(&self, key: CoordKey) -> Option<&D> {
        match self.borrow_tile_by_key(key) {
            Some(tile) => Some(tile.borrow_data()),
            None => None
        }
    }

    pub fn mut_borrow_data_by_key(&mut self, key: CoordKey) -> Option<&mut D> {
        match self.mut_borrow_tile_by_key(key) {
            Some(tile) => Some(tile.mut_borrow_data()),
            None => None
        }
    }

    pub fn mutate_all_data(&mut self, f: &Fn(&mut D)) {
        self.wrapper.mutate_all_data(f)
    }

    pub fn mut_borrow_data_for_pixel(&mut self, (x, y): (f32, f32)) -> Option<&mut D> {
        let key = C::from_pixel(&coords::PixelCoord::new((x, y), self.hexshape)).get_key();
        self.mut_borrow_data_by_key(key)
    }
}

/*
    Allow HexGrid to cast into an iterator of all its items
*/

// Helpers

// fn get_coord_from_1d_index<C: CoordType, D>(grid: &HexGrid<C, D>, i: usize) -> (usize, usize) {
//     let (mut x, mut y): (usize, usize) = (0, i);
//     while x < grid.tiles.len() && y >= grid.tiles[x].len() {
//         y -= grid.tiles[x].len();
//         x += 1;
//     }
//     (x, y)
// }

// Immutable Iteration

// impl<'a, C: CoordType, D> IntoIterator for &'a HexGrid<C, D> {
//     type Item = &'a HexTile<C, D>;
//     type IntoIter = HexGridTileIterator<'a, C, D>;

//     fn into_iter(self) -> Self::IntoIter {
//         HexGridTileIterator { grid: self, index: 0 }
//     }
// }

// pub struct HexGridTileIterator<'a, C: CoordType + 'a, D: 'a> {
//     grid: &'a HexGrid<C, D>,
//     index: usize
// }

// impl<'a, C: CoordType + 'a, D: 'a> Iterator for HexGridTileIterator<'a, C, D> {
//     type Item = &'a HexTile<C, D>;

//     fn next(&mut self) -> Option<&'a HexTile<C, D>> {
//         let result = get_coord_from_1d_index(self.grid, self.index);
//         self.index += 1;
//         self.grid.borrow_tile(result)
//     }
// }

// Mutable iteration (Might be impossible without hacks)

// impl<'a> IntoIterator for &'a mut HexGrid {
//     type Item = &'a mut HexTile<coords::OffsetOddCoordsFlat, SomeData>;
//     type IntoIter = HexGridTileIteratorMut<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         HexGridTileIteratorMut { grid: self, index: 0 }
//     }
// }

// pub struct HexGridTileIteratorMut<'a> {
//     grid: &'a mut HexGrid,
//     index: usize
// }

// impl<'a> HexGridTileIteratorMut<'a> {
//     fn get(&mut self, r: (usize, usize)) -> Option<&mut HexTile<coords::OffsetOddCoordsFlat, SomeData>> {
//         self.grid.mut_borrow_tile(r)
//     }
// }

// impl<'a> Iterator for HexGridTileIteratorMut<'a> {
//     type Item = &'a mut HexTile<coords::OffsetOddCoordsFlat, SomeData>;

//     fn next(&mut self) -> Option<&'a mut HexTile<coords::OffsetOddCoordsFlat, SomeData>> {
//         let result = get_coord_from_1d_index(self.grid, self.index);
//         self.index += 1;
//         // self.grid.mut_borrow_tile(result)
//         self.get(result)
//     }
// }