#![feature(type_ascription)]
#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate ggez;
extern crate rand;
extern crate quickcheck;
use ggez::event::MouseButton;
use ggez::*;
use ggez::graphics::{Vector2, Point2};

mod draw;
mod coord;
mod board;
mod measure;

pub struct MainState {
    pub board_state : board::Global,
    pub board_offset: Vector2,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let board_state = board::Global::new();
        let s = MainState { board_state, board_offset : Vector2::new(100.0, 30.0) };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        draw::board(ctx, &self)?;

        graphics::present(ctx);

        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        use measure::MousePosition;
        use board::Token;
        let rel_mouse_position = Point2::new(_x as f32, _y as f32) - self.board_offset;
        let measure = measure::Measure::default();
        let click = measure.resolve_mouse_position(rel_mouse_position);
        println!("MouseUp({}, {}) -> {:?}", _x, _y, click);

        match click {
            MousePosition::Local(coord) => {
                self.board_state[coord] = Token::Cross;
            },
            _ => ()
        }
        
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}