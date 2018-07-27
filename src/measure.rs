use ggez::graphics::{Vector2, Point2, Rect};
use coord;

#[derive(Debug, Clone)]
pub struct Measure {
    pub inner: HashtagMeasure,
    pub outer: HashtagMeasure,
}

#[derive(Debug, Clone)]
pub struct HashtagMeasure {
    pub block_size: f32,
    pub line_width: f32,
    pub total_size: f32,
    pub inner_padding: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MousePosition {
    Local(coord::Global),
    Region(coord::Local),
    BigHashtag,
    Outside
}

impl HashtagMeasure {
    pub fn new(block_size: f32, line_width: f32, inner_padding: f32) -> Self {
        let total_size = 3.0 * block_size + 2.0 * line_width;
        HashtagMeasure { block_size, line_width, total_size, inner_padding }
    }

    pub fn get_cell_rect(&self, coord: coord::Local) -> Rect {
        let step_size = self.block_size + self.line_width;
        Rect::new(
            coord.get_x() as f32 * step_size,
            coord.get_y() as f32 * step_size,
            self.block_size,
            self.block_size
        )
    }

    pub fn get_offset(&self, coord: coord::Local) -> Vector2 {
        Vector2::new(
            self.get_cell_rect(coord).x,
            self.get_cell_rect(coord).y
        )
    }

    pub fn get_offset_with_padding(&self, coord: coord::Local) -> Vector2 {
        self.get_offset(coord) + Vector2::new(self.inner_padding, self.inner_padding)
    }

    pub fn get_block_size_without_padding(&self) -> f32 {
        self.block_size - 2.0 * self.inner_padding
    }
}

impl Measure {
    fn from_inner_measures(small_block: f32, small_line: f32, small_padding: f32, big_line: f32, big_padding: f32) -> Self {
        let inner = HashtagMeasure::new(small_block, small_line, small_padding);
        let outer = HashtagMeasure::new(inner.total_size + 2.0 * big_padding, big_line, big_padding);
        Measure { inner, outer }
    }

    pub fn resolve_mouse_position(&self, pos: Point2) -> MousePosition {
        // Is the Point even inside the playing area?
        if !Rect::new(0.0, 0.0, self.outer.total_size, self.outer.total_size).contains(pos) {
            return MousePosition::Outside
        }

        // Search for the mouse position
        for region in coord::Local::iter() {
            if self.outer.get_cell_rect(region).contains(pos) {
                // Region found, find local
                for local in coord::Local::iter() {
                    let mut rect = self.inner.get_cell_rect(local);
                    rect.translate(self.outer.get_offset_with_padding(region));
                    if rect.contains(pos) {
                        return MousePosition::Local(coord::Global::new(region, local));
                    }
                }
                // No local found, return region.
                return MousePosition::Region(region);
            }
        }

        // Inside Rectangle, but not inside any Region => Mouse is on Hashtag.
        MousePosition::BigHashtag
    }
}

impl Default for Measure {
    fn default() -> Self {
        Measure::from_inner_measures(50.0, 5.0, 5.0, 10.0, 10.0)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};
    use coord;
    use measure::*;

    impl Arbitrary for Measure {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Measure::from_inner_measures(
                g.gen() : f32 * 100.0 + 1.0, 
                g.gen() : f32 * 100.0,
                g.gen() : f32 * 100.0,
                g.gen() : f32 * 100.0,
                g.gen() : f32 * 100.0
            )
        }
    }

    impl Arbitrary for HashtagMeasure {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            HashtagMeasure::new(
                g.gen() : f32 * 100.0 + 1.0,
                g.gen() : f32 * 100.0,
                g.gen() : f32 * 100.0
            )
        }
    }

    #[quickcheck]
    fn check_padding(measure: Measure) -> bool {
        // Floating point values only have approximate equality
        let error = measure.outer.get_block_size_without_padding() - measure.inner.total_size;
        error.abs() < 0.1
    }
}