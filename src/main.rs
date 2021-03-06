#![feature(type_ascription)]
#![feature(plugin)]
#![plugin(quickcheck_macros)]
#![warn(clippy)]
#![allow(cast_lossless)]

extern crate ggez;
extern crate quickcheck;
extern crate rand;
use ggez::event::MouseButton;
use ggez::graphics::Point2;
use ggez::*;

mod board;
mod coord;
mod draw;
mod measure;

use draw::GraphicsCache;
use measure::MousePosition;

pub struct MainState {
    pub board_state: board::Global,
    pub current_player: board::Player,
    pub active_region: Option<coord::Local>,
    pub active_hover: MousePosition,
    pub mouse_down_position: MousePosition,
    pub gfx: GraphicsCache,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let board_state = board::Global::new();
        let current_player = board::Player::Cross;
        let gfx = GraphicsCache::new(ctx)?;
        let s = MainState {
            board_state,
            current_player,
            active_region: None,
            active_hover: MousePosition::Outside,
            mouse_down_position: MousePosition::Outside,
            gfx,
        };
        Ok(s)
    }

    pub fn can_place_in_region(&mut self, region: coord::Local) -> bool {
        let is_active = self.active_region == None || self.active_region == Some(region);
        let still_has_space = self.board_state[region].total == board::Ownership::Undecided;

        is_active && still_has_space
    }

    fn on_try_place_token(&mut self, position: coord::Global) {
        let correct_region = self.can_place_in_region(position.get_region());
        let position_is_empty = self.board_state[position] == board::Token::Clear;

        if correct_region && position_is_empty {
            self.on_place_token(position);
        }
    }

    fn on_place_token(&mut self, position: coord::Global) {
        self.board_state
            .place_token(position, self.current_player.into());
        self.current_player = self.current_player.other();
        if self.board_state[position.get_local()].total == board::Ownership::Undecided {
            self.active_region = Some(position.get_local());
        } else {
            self.active_region = None;
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        draw::board(ctx, self)?;

        graphics::present(ctx);

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: i32,
        _y: i32,
    ) {
        use ggez::event::MouseButton;
        if _button == MouseButton::Left {
            // Get click position in game terms and store it for the release
            let rel_mouse_position = Point2::new(_x as f32, _y as f32) - self.gfx.grid_offset;
            self.mouse_down_position = self.gfx.measures.resolve_mouse_position(rel_mouse_position);
        } else if _button == MouseButton::Right {
            self.mouse_down_position = MousePosition::Outside;
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: i32,
        _y: i32,
    ) {
        let rel_mouse_position = Point2::new(_x as f32, _y as f32) - self.gfx.grid_offset;
        let click = self.gfx.measures.resolve_mouse_position(rel_mouse_position);

        if click != self.mouse_down_position {
            // Click position tracking indicates that the user wants to cancel the operation.
            self.mouse_down_position = MousePosition::Outside;
            return;
        }

        if let MousePosition::Local(coord) = click {
            self.on_try_place_token(coord);
        }

        self.mouse_down_position = MousePosition::Outside;
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        match GraphicsCache::new(ctx) {
            Ok(gfx) => self.gfx = gfx,
            Err(e) => panic!("Error while resizing: {:?}", e),
        }

        graphics::set_screen_coordinates(
            ctx,
            ggez::graphics::Rect::new(0.0, 0.0, width as f32, height as f32),
        ).unwrap();
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.resizable = true;
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();

    event::run(ctx, state).unwrap();
}
