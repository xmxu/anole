
pub(crate) trait Chain<T, R> {
    fn next(t: T) -> R;
}