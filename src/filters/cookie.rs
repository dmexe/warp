//! Cookie Filters

use headers::Cookie;

use ::never::Never;
use ::filter::{Filter, filter_fn_one};
use ::reject::Rejection;
use ::generic2::One;
use super::header;

/// Creates a `Filter` that requires a cookie by name.
///
/// If found, extracts the value of the cookie, otherwise rejects.
pub fn cookie(name: &'static str) -> impl Filter<Extract=One<String>, Error=Rejection> + Copy {
    header::header2()
        .and_then(move |cookie: Cookie| {
            cookie
                .get(name)
                .map(String::from)
                .ok_or_else(|| ::reject::bad_request())
        })
}

/// Creates a `Filter` that looks for an optional cookie by name.
///
/// If found, extracts the value of the cookie, otherwise continues
/// the request, extracting `None`.
pub fn optional(name: &'static str) -> impl Filter<Extract=One<Option<String>>, Error=Never> + Copy {
    header::optional()
        .map(move |opt: Option<Cookie>| {
            opt.and_then(|cookie| {
                cookie
                    .get(name)
                    .map(String::from)
            })
        })
}

#[doc(hidden)]
#[deprecated(note = "optional filters will be generalized")]
pub fn optional_value<U, F>(name: &'static str, func: F)
    -> impl Filter<Extract=One<Option<U>>, Error=Never> + Copy
where
    F: Fn(&str) -> U + Copy,
    U: Send,
{
    use headers::HeaderMapExt;
    filter_fn_one(move |route| {
        Ok(route.headers()
            .typed_get()
            .and_then(|cookie: Cookie| {
                cookie
                    .get(name)
                    .map(func)
            }))
    })
}

