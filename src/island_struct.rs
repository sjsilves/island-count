use crate::point_struct::Point;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Island {
    pub id: usize,
    pub lands: Vec<Point>
}