use ggez::graphics::Vector2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Global(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Local(u8);

impl Global {
    pub fn new(region: Local, local: Local) -> Self {
        Global(9 * region.0 + local.0)
    }

    #[allow(dead_code)]
    pub fn get_region(self) -> Local {
        Local(self.0 / 9)
    }

    #[allow(dead_code)]
    pub fn get_local(self) -> Local {
        Local(self.0 % 9)
    }
}

impl Local {
    pub fn new(x: u8, y: u8) -> Self {
        assert!(x < 3);
        assert!(y < 3);
        Local(3 * y + x)
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }

    pub fn get_x(self) -> u8 {
        self.0 % 3
    }

    pub fn get_y(self) -> u8 {
        self.0 / 3
    }

    pub fn as_vec(self) -> Vector2 {
        Vector2::new(self.get_x() as f32, self.get_y() as f32)
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        (0..9).map(Local)
    }
}

#[cfg(test)]
mod tests {
    use coord::*;
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Local {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Local(g.gen(): u8 % 9)
        }
    }

    impl Arbitrary for Global {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Global(g.gen(): u8 % 81)
        }
    }

    #[quickcheck]
    fn reassemble_local_coord(local_coord: Local) -> bool {
        local_coord == Local::new(local_coord.get_x(), local_coord.get_y())
    }

    #[quickcheck]
    fn reassemble_global_coord(global_coord: Global) -> bool {
        global_coord == Global::new(global_coord.get_region(), global_coord.get_local())
    }

    #[quickcheck]
    fn disassemble_local_pair(region_coord: Local, local_coord: Local) -> bool {
        let global_coord = Global::new(region_coord, local_coord);
        region_coord == global_coord.get_region() && local_coord == global_coord.get_local()
    }
}
