use ggez::*;
use ggez::graphics::{Point2, DrawMode, Mesh, Drawable, Vector2, Color};

use board;
use coord;
use measure::*;
use ::MainState;

pub fn board(ctx: &mut Context, state: &MainState) -> GameResult<()> {

    let white : Color = Color::from_rgb(255, 255, 255);
    let gray : Color = Color::from_rgb(50, 50, 50);

    graphics::set_background_color(ctx, white);

    let measures = Measure::default();

    let m = Mesh::new_circle(
        ctx,
        DrawMode::Line(measures.inner.line_width),
        Point2::origin(),
        0.5 * measures.inner.get_block_size_without_padding(),
        0.2)?;

    let m2 = Mesh::new_circle(
        ctx,
        DrawMode::Line(measures.outer.line_width),
        Point2::origin(),
        0.5 * measures.outer.get_block_size_without_padding(),
        0.2)?;

    // Draw the current player in the top left corner
    let info_offset = Vector2::new(10.0, 10.0);
    match state.current_player {
        board::Player::Cross => cross(ctx, info_offset, measures.inner.get_block_size_without_padding(), measures.inner.line_width)?,
        board::Player::Circle => circle(ctx, &m, info_offset, measures.inner.get_block_size_without_padding())?
    };

    graphics::set_color(ctx, gray)?;
    hashtag(ctx, state.board_offset, &measures.outer)?;

    for region in coord::Local::iter() {
        let region_offset = state.board_offset + measures.outer.get_offset_with_padding(region);

        graphics::set_color(ctx, determine_color(state.active_region, region))?;

        let board_region : &board::Local = &state.board_state[region];
        match board_region.total {
            board::Ownership::Undecided => {
                hashtag(ctx, region_offset, &measures.inner)?;
                for local in coord::Local::iter() {
                    let token_offset = region_offset + measures.inner.get_offset_with_padding(local);

                    let token = state.board_state[region][local];

                    match token {
                        board::Token::Clear => (),
                        board::Token::Cross => cross(ctx, token_offset, measures.inner.get_block_size_without_padding(), measures.inner.line_width)?,
                        board::Token::Circle => circle(ctx, &m, token_offset, measures.inner.get_block_size_without_padding())?
                    };
                }
            },
            board::Ownership::Cross => {
                cross(ctx, region_offset, measures.outer.get_block_size_without_padding(), measures.outer.line_width)?;
            },
            board::Ownership::Circle => {
                circle(ctx, &m2, region_offset, measures.outer.get_block_size_without_padding())?;
            },
            _ => {
                // TODO: Insert some drawing, maybe a squiggle?
            }
        }

    }

    Ok(())
}

fn determine_color(active_region: Option<coord::Local>, region: coord::Local) -> Color {
    let light_gray : Color = Color::from_rgb(150, 150, 150);
    let gray : Color = Color::from_rgb(50, 50, 50);

    if active_region == None || active_region == Some(region) {
        gray
    } else {
        light_gray
    }
}

fn hashtag(ctx: &mut Context, offset : Vector2, hashtag_measure: &HashtagMeasure) -> GameResult<()> {
    let c0 = 0.0;
    let c1 = hashtag_measure.block_size + 0.5 * hashtag_measure.line_width;
    let c2 = 2.0 * hashtag_measure.block_size + 1.5 * hashtag_measure.line_width;
    let c3 = 3.0 * hashtag_measure.block_size + 2.0 * hashtag_measure.line_width;

    line(ctx, Point2::new(c1, c0) + offset, Point2::new(c1, c3) + offset, hashtag_measure.line_width)?;
    line(ctx, Point2::new(c2, c0) + offset, Point2::new(c2, c3) + offset, hashtag_measure.line_width)?;
    line(ctx, Point2::new(c0, c1) + offset, Point2::new(c3, c1) + offset, hashtag_measure.line_width)?;
    line(ctx, Point2::new(c0, c2) + offset, Point2::new(c3, c2) + offset, hashtag_measure.line_width)?;

    Ok(())
}

fn cross(ctx: &mut Context, offset: Vector2, block_size: f32, line_width: f32) -> GameResult<()> {
    let c0 = 0.0;
    let c1 = block_size;

    line(ctx, Point2::new(c0, c0) + offset, Point2::new(c1, c1) + offset, line_width)?;
    line(ctx, Point2::new(c1, c0) + offset, Point2::new(c0, c1) + offset, line_width)?;

    Ok(())
}

fn circle(ctx: &mut Context, mesh: &Mesh, offset: Vector2, block_size: f32) -> GameResult<()> {
    let center = Point2::new(0.5 * block_size, 0.5 * block_size) + offset;

    mesh.draw(ctx, center, 0.0)?;

    Ok(())
}

fn line(ctx: &mut Context, start: Point2, stop: Point2, width: f32) -> GameResult<()> {
    graphics::line(ctx, &vec![start, stop], width)
}