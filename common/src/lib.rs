use serde_derive::{Deserialize, Serialize};

pub mod inspect;
pub mod lazy;
pub mod transform;

/// Simulation of try block...
/// # Examples
///
/// ```rust
/// use fleaxj::util::try_do;
///
/// let ret = try_do(|| {
///     None?;
///     Some(1)
/// });
/// assert_eq!(ret, None);
/// ```
pub fn try_do<R>(f: impl FnOnce() -> R) -> R {
    f()
}


#[derive(Deserialize, Serialize, Clone)]
pub struct Message {
    pub sender: String,
    pub send_time: String,
    pub message: String,
}
