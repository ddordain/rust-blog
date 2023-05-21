use crate::errors::AppError;
use actix_web::HttpResponse;

pub(super) mod users;
pub(super) mod posts;

// Converts a `Result<T, E>` into `Result<HttpResponse, AppError>`.
//
// If `Result` is `Ok`, creates a successful HTTP response with a JSON body derived from the `Ok` value.
// If `Result` is `Err`, converts the error of type `E` into `AppError`.
//
// # Parameters
// * `res` - A `Result` which could be of two types: `Ok(T)` or `Err(E)`
//
// # Returns
// * An `HttpResponse` wrapped in `Ok` if input `Result` is `Ok`, or an `AppError` wrapped in `Err` if input `Result` is `Err`.
//
// # Type Parameters
// * `T` - Must implement `serde::Serialize` to be able to be converted into JSON.
// * `E` - Must be able to be converted into `AppError`.
fn convert<T, E>(res: Result<T, E>) -> Result<HttpResponse, AppError>
where
    T: serde::Serialize,
    AppError: From<E>,
{
    res.map(|d| HttpResponse::Ok().json(d)).map_err(Into::into)
}
