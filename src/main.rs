#![feature(type_ascription)] 
extern crate ggez;
extern crate rand;
use ggez::*;

mod draw;
mod coord;
mod board;

struct MainState {
    state : board::Global
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let state = board::Global::random();
        let s = MainState { state };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        draw::board(ctx, &self.state)?;

        graphics::present(ctx);

        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}