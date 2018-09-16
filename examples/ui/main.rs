extern crate ggez;

use ggez::event::{self, MouseButton, MouseState, Keycode, Mod};
use ggez::graphics::{self, Point2, Mesh, DrawParam, Color};
use ggez::{Context, GameResult, ContextBuilder, mouse};

extern crate hex_test;
use hex_test::{grid, HexShape, Orientation};
use grid::initializers::HexagonalCenteredGridIterator;

const DEFAULT_HEX_RADIUS: f32 = 25.0;

pub struct SomeData {
    pub hovered: bool,
    pub selected: bool
}

impl SomeData {
    fn new() -> Self {
        Self { hovered: false, selected: false }
    }
}

type MyCoordSystem = grid::coords::CubeCoords;
type MyGridWrapper = grid::HexGridHashMapWrapper<MyCoordSystem, SomeData>;
type MyHexGrid = grid::HexGrid<MyCoordSystem, SomeData, MyGridWrapper>;

pub struct GraphicsData {
    origin_mesh: Mesh,
    zoom: f32,
    offset: (f32, f32)
}

fn tuple_to_point((x, y): (f32, f32)) -> Point2 {
    Point2::new(x, y)
}

fn point_to_tuple(p: Point2) -> (f32, f32) {
    (p.coords.x, p.coords.y)
}

fn ui_scaling((x, y): (f32, f32), gfx_data: &GraphicsData) -> (f32, f32){
    let ((off_x, off_y), scale) = (gfx_data.offset, gfx_data.zoom);
    ((x * scale) + off_x, (y * scale) + off_y)
}

fn reverse_ui_scaling((x, y): (f32, f32), gfx_data: &GraphicsData) -> (f32, f32) {
    let ((off_x, off_y), scale) = (gfx_data.offset, gfx_data.zoom);
    ((x - off_x) / scale, (y - off_y) / scale)
}

fn point_data_to_mesh(ctx: &mut Context, data: [(f32, f32); 6]) -> Mesh {
    let vec: Vec<Point2> =  data.to_vec().into_iter().map(|(x, y)| Point2::new(x, y)).collect();

    Mesh::new_polygon(ctx, graphics::DrawMode::Fill, &vec).unwrap()
}

fn draw_everything(ctx: &mut Context, hex_map: &MyHexGrid, gfx_data: &GraphicsData) {

    for ref hex in hex_map.wrapper.into_iter() {
        let data = hex.borrow_data();

        // let dest = tuple_to_point(gfx_data.offset);
        let dest = tuple_to_point(ui_scaling(hex.to_pixel(DEFAULT_HEX_RADIUS).get(), gfx_data));

        let hex_shape = &gfx_data.origin_mesh;
        // let hex_shape = point_data_to_mesh(ctx, hex.to_pixels(DEFAULT_HEX_RADIUS));

        let scale = tuple_to_point((gfx_data.zoom * 0.95, gfx_data.zoom * 0.95));

        let color = match (data.selected, data.hovered) {
            (true, true) => Some(Color::from_rgb(255, 0, 0)),
            (true, false) => Some(Color::from_rgb(200, 100, 0)),
            (false, true) => Some(Color::from_rgb(0, 0, 255)),
            (false, false) => Some(Color::from_rgb(0, 255, 0))
        };

        graphics::draw_ex(ctx, hex_shape, DrawParam { 
            dest,
            scale,
            color,
            .. Default::default()
        }).unwrap();
    }
}

struct State {
    map: MyHexGrid,
    gfx_data: GraphicsData
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        draw_everything(ctx, &self.map, &self.gfx_data);
        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
        if let Ok(mouse_point) = mouse::get_position(ctx) {
            let rel_point = reverse_ui_scaling(point_to_tuple(mouse_point), &self.gfx_data);
            let result = self.map.mut_borrow_data_for_pixel(rel_point);
            match result {
                Some(data) => data.selected = !data.selected,
                None => ()
            };
        }
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
        self.map.mutate_all_data(&|d: &mut SomeData| d.hovered = false);
        let rel_point = reverse_ui_scaling((x as f32, y as f32), &self.gfx_data);
        let result = self.map.mut_borrow_data_for_pixel(rel_point);
        match result {
            Some(data) => data.hovered = true,
            None => ()
        };
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            _ => ()
        }
    }
}


fn main() {
    let cb = ContextBuilder::new("astroblasto", "ggez");
    let ctx = &mut cb.build().unwrap();

    let shape = HexShape::new(DEFAULT_HEX_RADIUS, Orientation::Flat);
    
    let mut map = MyHexGrid::new(shape, None);
    let initializer = HexagonalCenteredGridIterator::new(5, shape.orient());
    map.initialize(initializer, &SomeData::new);
    let origin_mesh = point_data_to_mesh(ctx, shape.corners((0.0, 0.0)));

    let mut state = State {
        map,
        gfx_data: GraphicsData { zoom: 1.0, offset: (300.0, 300.0), origin_mesh}
    };
    event::run(ctx, &mut state).unwrap();
}
