pub trait Transformation<F: FnOnce(T) -> R, T: Sized, R: Sized> {
    fn transformation(self, f: F) -> R;
}
impl<F: FnOnce(T) -> R, T: Sized, R: Sized> Transformation<F, T, R> for T {
    fn transformation(self, f: F) -> R {
        f(self)
    }
}
