#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum HardFork {
    #[default]
    Roswell = 0,
    Rachel = 1,
}
