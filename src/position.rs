pub type Position = (u64, u64);

pub trait Normalizable {
    fn normal(&self) -> usize {
        return 0;
    }
}

impl Normalizable for Position {
    fn normal(&self) -> usize {
        let (x, y) = self;
        (((y - 1) * 8) + (x - 1)) as usize
    }
}
