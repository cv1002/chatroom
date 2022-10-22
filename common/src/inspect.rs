/* Trait declaration */
pub trait ResultInspect<F: FnOnce(&T), T: Sized> {
    /// Call function when ok.
    fn inspect_ok(self, f: F) -> Self;
}
pub trait ResultInspectRef<F: FnOnce(&T), T: Sized> {
    /// Call function when ok, but doesn't move.
    fn inspect_ref(&self, f: F) -> &Self;
}
pub trait ResultInspectErr<F: FnOnce(&E), E: Sized> {
    /// Call function when error.
    fn inspect_error(self, f: F) -> Self;
}
pub trait ResultInspectErrRef<F: FnOnce(&E), E: Sized> {
    /// Call function when error, but doesn't move.
    fn inspect_err_ref(&self, f: F) -> &Self;
}
pub trait OptionInspect<F: FnOnce(&T), T: Sized> {
    /// Call function when some.
    fn inspect_some(self, f: F) -> Self;
}
pub trait OptionInspectRef<F: FnOnce(&T), T: Sized> {
    /// Call function when some, but doesn't move.
    fn inspect_ref(&self, f: F) -> &Self;
}
pub trait OptionInspectNone<F: FnOnce()> {
    /// Call function when none.
    fn inspect_none(self, f: F) -> Self;
}
pub trait OptionInspectNoneRef<F: FnOnce()> {
    /// Call function when none, but doesn't move.
    fn inspect_none_ref(&self, f: F) -> &Self;
}

/* Trait implement */
impl<F: FnOnce(&T), T: Sized, E> ResultInspect<F, T> for Result<T, E> {
    fn inspect_ok(self, f: F) -> Self {
        if let Ok(o) = self.as_ref() {
            f(o);
        }

        self
    }
}

impl<F: FnOnce(&T), T: Sized, E> ResultInspectRef<F, T> for Result<T, E> {
    fn inspect_ref(&self, f: F) -> &Self {
        if let Ok(o) = self {
            f(o);
        }

        self
    }
}


impl<F: FnOnce(&E), T, E: Sized> ResultInspectErr<F, E> for Result<T, E> {
    fn inspect_error(self, f: F) -> Self {
        if let Err(e) = self.as_ref() {
            f(e);
        }

        self
    }
}

impl<F: FnOnce(&E), T, E: Sized> ResultInspectErrRef<F, E> for Result<T, E> {
    fn inspect_err_ref(&self, f: F) -> &Self {
        if let Err(e) = self {
            f(e);
        }

        self
    }
}

impl<F: FnOnce(&T), T: Sized> OptionInspect<F, T> for Option<T> {
    fn inspect_some(self, f: F) -> Self {
        if let Some(o) = self.as_ref() {
            f(o);
        }

        self
    }
}

impl<F: FnOnce(&T), T: Sized> OptionInspectRef<F, T> for Option<T> {
    fn inspect_ref(&self, f: F) -> &Self {
        if let Some(o) = self {
            f(o);
        }

        self
    }
}

impl<F: FnOnce(), T> OptionInspectNone<F> for Option<T> {
    fn inspect_none(self, f: F) -> Self {
        if let None = self.as_ref() {
            f();
        }

        self
    }
}

impl<F: FnOnce(), T> OptionInspectNoneRef<F> for Option<T> {
    fn inspect_none_ref(&self, f: F) -> &Self {
        if let None = self {
            f();
        }

        self
    }
}
