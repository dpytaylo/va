use crate::utils::math::vector::vector2::Vec2;
use crate::utils::number::Number;

use super::rect::Rect;

pub trait PointGeometry<T> 
    where T: Number,
{
    fn is_inside_box(self, rect: Rect<T>) -> bool;
}

impl<T> PointGeometry<T> for Vec2<T>
    where T: Number,
{
    fn is_inside_box(self, rect: Rect<T>) -> bool {
        rect.p1.x <= self.x && rect.p1.y <= self.y
            && self.x <= rect.p2.x && self.y <= rect.p2.y
    }
}