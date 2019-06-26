use crate::point_struct::Point;

#[derive(PartialEq, PartialOrd, Debug, Eq, Hash, Clone)]
pub struct Island {
    pub id: usize,
    pub lands: Vec<Point>
}