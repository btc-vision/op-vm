#[derive(Copy, Clone)]
pub struct AbortData {
    pub message: u32,
    pub file_name: u32,
    pub line: u32,
    pub column: u32,
}
