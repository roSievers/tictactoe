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
    pub current_player : board::Player,
    pub active_region : Option<coord::Local>,
    pub board_offset: Vector2,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let board_state = board::Global::new();
        let current_player = board::Player::Cross;
        let s = MainState {
            board_state, 
            current_player, 
            active_region : None, 
            board_offset : Vector2::new(100.0, 30.0)
        };
        Ok(s)
    }

    pub fn can_place_in_region(&mut self, region: coord::Local) -> bool {
        self.active_region == None || self.active_region == Some(region)
    }

    fn on_try_place_token(&mut self, position: coord::Global) {
        let correct_region = self.can_place_in_region(position.get_region());
        let position_is_empty = self.board_state[position] == board::Token::Clear;

        if correct_region && position_is_empty {
            self.on_place_token(position);
        }
    }

    fn on_place_token(&mut self, position: coord::Global) {
        self.board_state[position] = self.current_player.into();
        self.current_player = self.current_player.other();
        self.active_region = Some(position.get_local());
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
        let rel_mouse_position = Point2::new(_x as f32, _y as f32) - self.board_offset;
        let measure = measure::Measure::default();
        let click = measure.resolve_mouse_position(rel_mouse_position);

        match click {
            MousePosition::Local(coord) => {
                self.on_try_place_token(coord);
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