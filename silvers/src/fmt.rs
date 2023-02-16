use std::fmt::Display;

use itertools::{Either, Itertools};

pub(crate) fn spaced<T: Display>(xs: impl IntoIterator<Item = T>) -> impl Display {
    xs.into_iter().format(" ")
}
pub(crate) fn comma<T: Display>(xs: impl IntoIterator<Item = T>) -> impl Display {
    xs.into_iter().format(", ")
}
pub(crate) fn lined<T: Display>(xs: impl IntoIterator<Item = T>) -> impl Display {
    xs.into_iter().format("\n")
}
pub(crate) fn prefixed<T: Display>(
    s: &'static str,
    xs: impl IntoIterator<Item = T>,
) -> impl Iterator<Item = String> {
    xs.into_iter().map(move |x| format!("{s}{x}"))
}
pub(crate) fn indent<T: Display>(x: T) -> impl Display {
    x.to_string().lines().map(|l| format!("  {l}")).join("\n")
}
pub(crate) fn indented<T: Display>(xs: impl IntoIterator<Item = T>) -> impl Display {
    xs.into_iter().map(|l| format!("  {l}")).join("\n")
}
pub(crate) fn opt<T: Display>(x: &Option<T>) -> impl Display + '_ {
    if let Some(x) = x {
        Either::Left(x)
    } else {
        Either::Right("")
    }
}
