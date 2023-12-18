#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RangeOption<T> {
    Inside { min: Option<T>, max: Option<T>, inclusive: bool },
    Outside { min: Option<T>, max: Option<T>, inclusive: bool },
    Unlimited,
}
