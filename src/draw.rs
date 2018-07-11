use ggez::*;
use ggez::graphics::{Point2, DrawMode, Rect};

use board;
use coord;

const LINE_WIDTH : f32 = 0.02;

// Doing it the OpenGL way

pub fn board(ctx: &mut Context, state: &board::Global) -> GameResult<()> {
    // Prepare viewport for the board.
    let layer = viewport::new_layer(ctx);
    viewport::make_unit_square(ctx, Rect::new(100.0, 0.0, 600.0, 600.0));
    viewport::zoom(ctx, 0.96);
    let layer_app = viewport::apply_layer(ctx, layer)?;

    graphics::apply_transformations(ctx)?;

    hashtag(ctx)?;

    for region in coord::Local::iter() {
        let inner_layer = viewport::new_layer(ctx);
        const THIRD : f32 = 1.0 / 3.0;

        viewport::make_unit_square(ctx, Rect::new(region.get_x() as f32 * THIRD, region.get_y() as f32 * THIRD, THIRD, THIRD));
        viewport::zoom(ctx, 0.8);

        let inner_layer_app = viewport::apply_layer(ctx, inner_layer)?;

        subgame(ctx, &state[region])?;

        viewport::remove_layer(ctx, inner_layer_app);
    }

    viewport::remove_layer(ctx, layer_app);

    Ok(())
}

pub fn subgame(ctx: &mut Context, state: &board::Local) -> GameResult<()> {
    hashtag(ctx)?;

    for region in coord::Local::iter() {
        let inner_layer = viewport::new_layer(ctx);
        const THIRD : f32 = 1.0 / 3.0;

        viewport::make_unit_square(ctx, Rect::new(region.get_x() as f32 * THIRD, region.get_y() as f32 * THIRD, THIRD, THIRD));
        viewport::zoom(ctx, 0.8);

        let inner_layer_app = viewport::apply_layer(ctx, inner_layer)?;

        if let board::Token::Cross = state[region] {
            cross(ctx)?;
        } else {
            circle(ctx)?;
        }

        viewport::remove_layer(ctx, inner_layer_app);
    }

    Ok(())
}

fn hashtag(ctx: &mut Context) -> GameResult<()> {
    const THIRD : f32 = 1.0 / 3.0;

    line(ctx, Point2::new(1.0 * THIRD, 0.0), Point2::new(1.0 * THIRD, 1.0), LINE_WIDTH)?;
    line(ctx, Point2::new(2.0 * THIRD, 0.0), Point2::new(2.0 * THIRD, 1.0), LINE_WIDTH)?;
    line(ctx, Point2::new(0.0, 1.0 * THIRD), Point2::new(1.0, 1.0 * THIRD), LINE_WIDTH)?;
    line(ctx, Point2::new(0.0, 2.0 * THIRD), Point2::new(1.0, 2.0 * THIRD), LINE_WIDTH)?;

    Ok(())
}

// fn token(ctx: &mut Context, token: board::Token, block_size: f32, line_width: f32, offset: (f32, f32)) -> GameResult<()> {
//     match token {
//         board::Token::Clear => Ok(()),
//         board::Token::Cross => cross(ctx, block_size, line_width, offset),
//         board::Token::Circle => circle(ctx, block_size, line_width, offset)
//     }
// }

fn cross(ctx: &mut Context) -> GameResult<()> {
    line(ctx, Point2::new(0.0, 0.0), Point2::new(1.0, 1.0), LINE_WIDTH)?;
    line(ctx, Point2::new(1.0, 0.0), Point2::new(0.0, 1.0), LINE_WIDTH)?;

    Ok(())
}

fn circle(ctx: &mut Context) -> GameResult<()> {
    let center = Point2::new(0.5, 0.5);

    graphics::circle(ctx, DrawMode::Line(LINE_WIDTH), center, 0.5, 0.01)?;

    Ok(())
}

fn line(ctx: &mut Context, start: Point2, stop: Point2, width: f32) -> GameResult<()> {
    graphics::line(ctx, &vec![start, stop], width)
}

mod viewport {
    use ggez::{Context, graphics, GameResult};
    use ggez::graphics::{Matrix4, Rect};
    use ggez::nalgebra as na;
    type Vector3 = na::Vector3<f32>;

    // This zero sized type ensures, that we can not remove more layers than we create.
    // Must use helps us keep track of the layers.
    #[must_use]
    pub struct ViewportCreatedToken();

    // Ensures, that we apply the transformation in each layer.
    #[must_use]
    pub struct ViewportAppliedToken();

    pub fn new_layer(ctx: &mut Context) -> ViewportCreatedToken {
        graphics::push_transform(ctx, None);
        ViewportCreatedToken()
    }

    pub fn apply_layer(ctx: &mut Context, _: ViewportCreatedToken) -> GameResult<ViewportAppliedToken> {
        graphics::apply_transformations(ctx)?;
        Ok(ViewportAppliedToken())
    }

    pub fn remove_layer(ctx: &mut Context, _ : ViewportAppliedToken) {
        graphics::pop_transform(ctx);
    }

    pub fn make_unit_square(ctx: &mut Context, rect: Rect) {
        let offset_matrix = Matrix4::new_translation(&Vector3::new(rect.left(), rect.top(), 0.0));
        let scale_matrix = Matrix4::new_scaling(rect.w); // FIXME

        let current_matrix = graphics::get_transform(ctx);
        graphics::set_transform(ctx, current_matrix * offset_matrix * scale_matrix);
    }

    pub fn zoom(ctx: &mut Context, factor: f32) {
        let offset = (1.0 - factor) / 2.0;
        make_unit_square(ctx, Rect::new(offset, offset, factor, factor))
    }
}