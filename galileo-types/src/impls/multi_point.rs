pub struct MultiPoint<P>(Vec<P>);

impl<P> crate::multi_point::MultiPoint for MultiPoint<P> {
    type Point = P;

    fn iter_points(&self) -> impl Iterator<Item = &'_ Self::Point> {
        self.0.iter()
    }
}

impl<P> From<Vec<P>> for MultiPoint<P> {
    fn from(value: Vec<P>) -> Self {
        Self(value)
    }
}
