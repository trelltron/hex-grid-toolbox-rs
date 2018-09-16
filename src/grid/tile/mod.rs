use super::coords::{self, CoordType, CoordKey};

pub struct HexTile<C: CoordType, D>
{
    coord: C,
    data: D
}

impl<C: CoordType, D> HexTile<C, D> 
{
    pub fn new(coord: C, data: D) -> Self {
        Self { coord, data }
    }

    pub fn get_key(&self) -> CoordKey {
        self.coord.get_key()
    }

    pub fn borrow_data(&self) -> &D {
        &self.data
    }

    pub fn mut_borrow_data(&mut self) -> &mut D {
        &mut self.data
    }

    pub fn mutate_data(&mut self, f: &Fn(&mut D)) {
        f(&mut self.data)
    }

    pub fn set_data(&mut self, new: D) {
        self.data = new;
    }

    pub fn borrow_coord(&self) -> &C {
        &self.coord
    }

    pub fn to_pixel(&self, radius: f32) -> coords::PixelCoord {
        self.coord.to_pixel(radius)
    }

    pub fn to_pixels(&self, radius: f32) -> [(f32, f32); 6] {
        self.coord.to_pixel(radius).corners()
    }
}