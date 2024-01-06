#[derive(PartialEq, Clone, Debug)]
pub struct GameTime {
    pub wtime: u128,
    pub winc: u128,
    pub btime: u128,
    pub binc: u128,
}

impl GameTime {
    pub fn new(wtime: u128, winc: u128, btime: u128, binc: u128) -> Self {
        Self {
            wtime,
            winc,
            btime,
            binc,
        }
    }
}
