#[derive(PartialEq, PartialOrd, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
} 
impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}