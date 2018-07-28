use ggez::graphics::{Color, DrawMode, Drawable, Mesh, Point2, Vector2};
use ggez::*;

use board;
use coord;
use measure::*;
use MainState;

/// A type to cache meshes and other information required for rendering.
pub struct GraphicsCache {
    pub measures: Measure,
    small_circle: Mesh,
    big_circle: Mesh,
    pub grid_offset: Vector2,
}

/// Several mostly constant values that only depend on the screen size.
/// They are calculated whenever the screen size changes.
impl GraphicsCache {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let (width, _height) = graphics::get_size(ctx);
        // We only support a horizontal mode at the moment
        let height = u32::min(_height, width * 3 / 4);
        let measures = Measure::from_size(height as f32 * 0.94, 0.02, 0.04);

        // Generate Meshes
        let small_circle = Mesh::new_circle(
            ctx,
            DrawMode::Line(measures.inner.line_width),
            Point2::origin(),
            0.5 * measures.inner.get_block_size_without_padding(),
            0.2,
        )?;
        let big_circle = Mesh::new_circle(
            ctx,
            DrawMode::Line(measures.outer.line_width),
            Point2::origin(),
            0.5 * measures.outer.get_block_size_without_padding(),
            0.2,
        )?;
        let grid_offset = Vector2::new(
            width as f32 - height as f32 + height as f32 * 0.03,
            height as f32 * 0.03,
        );

        Ok(GraphicsCache {
            measures,
            small_circle,
            big_circle,
            grid_offset,
        })
    }
}

pub fn board(ctx: &mut Context, state: &mut MainState) -> GameResult<()> {
    let white: Color = Color::from_rgb(255, 255, 255);
    let gray: Color = Color::from_rgb(50, 50, 50);

    graphics::set_background_color(ctx, white);

    // Draw the current player in the top left corner
    let info_offset = Vector2::new(10.0, 10.0);
    match state.current_player {
        board::Player::Cross => cross(
            ctx,
            info_offset,
            state.gfx.measures.inner.get_block_size_without_padding(),
            state.gfx.measures.inner.line_width,
        )?,
        board::Player::Circle => circle(
            ctx,
            &state.gfx.small_circle,
            info_offset,
            state.gfx.measures.inner.get_block_size_without_padding(),
        )?,
    };

    graphics::set_color(ctx, gray)?;
    hashtag(ctx, state.gfx.grid_offset, &state.gfx.measures.outer)?;

    for region in coord::Local::iter() {
        let region_offset =
            state.gfx.grid_offset + state.gfx.measures.outer.get_offset_with_padding(region);

        graphics::set_color(
            ctx,
            determine_color(can_place_in_region(state.active_region, region), false),
        )?;

        let board_region: &board::Local = &state.board_state[region];
        match board_region.total {
            board::Ownership::Undecided => {
                hashtag(ctx, region_offset, &state.gfx.measures.inner)?;
                for local in coord::Local::iter() {
                    let token_offset =
                        region_offset + state.gfx.measures.inner.get_offset_with_padding(local);

                    let mut token = state.board_state[region][local];
                    let mut is_ghost = false;

                    // If there is no token and the mouse is held down, we show a ghost
                    if token == board::Token::Clear
                        && state.mouse_down_position
                            == MousePosition::Local(coord::Global::new(region, local))
                    {
                        token = state.current_player.into();
                        is_ghost = true;
                    }

                    graphics::set_color(
                        ctx,
                        determine_color(can_place_in_region(state.active_region, region), is_ghost),
                    )?;

                    draw_token(
                        ctx,
                        token,
                        &state.gfx.small_circle,
                        token_offset,
                        state.gfx.measures.inner.get_block_size_without_padding(),
                        state.gfx.measures.inner.line_width,
                    )?;
                }
            }
            board::Ownership::Cross => {
                cross(
                    ctx,
                    region_offset,
                    state.gfx.measures.outer.get_block_size_without_padding(),
                    state.gfx.measures.outer.line_width,
                )?;
            }
            board::Ownership::Circle => {
                circle(
                    ctx,
                    &state.gfx.big_circle,
                    region_offset,
                    state.gfx.measures.outer.get_block_size_without_padding(),
                )?;
            }
            _ => {
                // TODO: Insert some drawing, maybe a squiggle?
            }
        }
    }

    Ok(())
}

fn determine_color(is_active_region: bool, is_ghost: bool) -> Color {
    match (is_active_region, is_ghost) {
        (true, false) => Color::from_rgb(50, 50, 50),
        (true, true) => Color::from_rgb(170, 170, 170),
        (false, false) => Color::from_rgb(170, 170, 170),
        (false, true) => panic!("Ghosts can only exist inside an active region."),
    }
}

fn can_place_in_region(active_region: Option<coord::Local>, region: coord::Local) -> bool {
    if let Some(r) = active_region {
        r == region
    } else {
        // No active_region => all fields are highlighted
        true
    }
}

fn hashtag(ctx: &mut Context, offset: Vector2, hashtag_measure: &HashtagMeasure) -> GameResult<()> {
    let c0 = 0.0;
    let c1 = hashtag_measure.block_size + 0.5 * hashtag_measure.line_width;
    let c2 = 2.0 * hashtag_measure.block_size + 1.5 * hashtag_measure.line_width;
    let c3 = 3.0 * hashtag_measure.block_size + 2.0 * hashtag_measure.line_width;

    line(
        ctx,
        Point2::new(c1, c0) + offset,
        Point2::new(c1, c3) + offset,
        hashtag_measure.line_width,
    )?;
    line(
        ctx,
        Point2::new(c2, c0) + offset,
        Point2::new(c2, c3) + offset,
        hashtag_measure.line_width,
    )?;
    line(
        ctx,
        Point2::new(c0, c1) + offset,
        Point2::new(c3, c1) + offset,
        hashtag_measure.line_width,
    )?;
    line(
        ctx,
        Point2::new(c0, c2) + offset,
        Point2::new(c3, c2) + offset,
        hashtag_measure.line_width,
    )?;

    Ok(())
}

fn draw_token(
    ctx: &mut Context,
    token: board::Token,
    circle_mesh: &Mesh,
    offset: Vector2,
    block_size: f32,
    line_width: f32,
) -> GameResult<()> {
    match token {
        board::Token::Circle => circle(ctx, circle_mesh, offset, block_size),
        board::Token::Cross => cross(ctx, offset, block_size, line_width),
        board::Token::Clear => Ok(()),
    }
}

fn cross(ctx: &mut Context, offset: Vector2, block_size: f32, line_width: f32) -> GameResult<()> {
    let c0 = 0.0;
    let c1 = block_size;

    line(
        ctx,
        Point2::new(c0, c0) + offset,
        Point2::new(c1, c1) + offset,
        line_width,
    )?;
    line(
        ctx,
        Point2::new(c1, c0) + offset,
        Point2::new(c0, c1) + offset,
        line_width,
    )?;

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
