pub trait IsCopyTypeEqual<T = Self>: Copy {
    fn is_equal(self, other: T, epsilon: f64) -> bool;
}

impl IsCopyTypeEqual for f64 {
    fn is_equal(self, other: Self, epsilon: f64) -> bool {
        if self == other {
            // shortcut, handles infinities
            return true;
        }

        let diff = (self - other).abs();
        if diff > epsilon {
            return false;
        }

        true
    }
}