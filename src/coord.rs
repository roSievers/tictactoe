#[derive(Clone, Copy, Debug)]
pub struct Global(u8);

#[derive(Clone, Copy, Debug)]
pub struct Local(u8);

impl Global {
    fn new(region: Local, local: Local) -> Self {
        Global(9 * region.0 + local.0)
    }

    fn get_region(self) -> Local {
        Local(self.0 / 9)
    }

    fn get_local(self) -> Local {
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

    pub fn iter() -> impl Iterator<Item = Self>  {
        (0..9).map( |i : u8| Local(i))
    }
}