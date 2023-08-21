// use std::os::unix::process::parent_id;

// use crate::{ctx::Ctx, model::ModelController, Error, Result};
// use axum::{
//     async_trait,
//     extract::{FromRequestParts, State},
//     http::{request::Parts, Request},
//     middleware::Next,
//     response::Response,
//     RequestPartsExt,
// };
// use lazy_regex::regex_captures;
// use tower_cookies::{Cookie, Cookies};

// pub async fn middleware_require_auth<B>(
//     ctx: Result<Ctx>,
//     req: Request<B>,
//     next: Next<B>,
// ) -> Result<Response> {
//     println!("->> {:12} - middleware_require_auth", "MIDDLEWARE");

//     ctx?;

//     Ok(next.run(req).await)
// }

// pub async fn middleware_ctx_resolver<B>(
//     _mc: State<ModelController>,
//     cookies: Cookies,
//     mut req: Request<B>,
//     next: Next<B>,
// ) -> Result<Response> {
//     println!("->> {:12} - middleware_ctx_resolver", "MIDDLEWARE");

//     let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

//     // Compute Result<Ctx>.
//     let result_ctx = match auth_token
//         .ok_or(Error::AuthFailNoAuthTokenCookie)
//         .and_then(parse_auth_token)
//     {
//         Ok((user_id, _exp, _sign)) => {
//             // TODO: Token components validations.
//             Ok(Ctx::new(user_id))
//         }
//         Err(e) => Err(e),
//     };

//     // Remove the cookie if something went wrong other than NoAuthTokenCookie.
//     if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
//         cookies.remove(Cookie::named(AUTH_TOKEN))
//     }

//     // Store the ctx_result in the request extension.
//     req.extensions_mut().insert(result_ctx);

//     Ok(next.run(req).await)
// }

// // region: -- Ctx Extractor
// #[async_trait]
// impl<S: Send + Sync> FromRequestParts<S> for Ctx {
//     type Rejection = Error;

//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
//         println!("->> {:12} - Ctx Extractor", "EXTRACTOR");

//         match parts.extensions.get::<Result<Ctx>>() {
//             Some(Ok(ctx)) => Ok(ctx.clone()),
//             _ => Err(Error::AuthFailCtxNotInExt),
//         }
//     }
// }
// // endregion: -- Ctx Extractor

// /// Parse a token of format `user-[user-id].[exp].[signature]`
// /// Returns (user-id, exp, signature)
// fn parse_auth_token(token: String) -> Result<(u64, String, String)> {
//     let (_whole, user_id, exp, signature) =
//         regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::AuthFailInvalidToken)?;

//     let user_id = match { user_id.parse::<u64>() } {
//         Ok(user_id) => user_id,
//         Err(_) => return Err(Error::AuthFailInvalidToken),
//     };

//     Ok((user_id, exp.to_string(), signature.to_string()))
// }
