use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::{Error, HttpResponse};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::{Future, future::{ok, Ready}};
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};

use crate::auth::Claims;
use crate::config::CONFIG;
use crate::errors::{ make_error_object};

const HEADER_SESSION_ID: &str = "Session-Id";

pub struct Auth;

impl<S, B> Transform<S> for Auth
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let _auth = req.headers().get(HEADER_SESSION_ID);
        if _auth.is_none() {
            return Box::pin(async move {
                error!("Missing sessionId header.");
                Ok(req.into_response(
                    make_auth_error_json_response(format!("Missing header '{}'", HEADER_SESSION_ID).as_str()).into_body()
                ))
            });
        }

        let token = _auth.unwrap().to_str().unwrap();
        let _var = &CONFIG.jwt_key;
        let key = _var.as_bytes();
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(_token) => {
                debug!("Session token is ok.");
                let fut = self.service.call(req);

                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(_e) => {
                return Box::pin(async move {
                    error!("Invalid token: {}", _e);
                    Ok(req.into_response(
                        make_auth_error_json_response("Invalid token").into_body()
                    ))
                });
            }
        }
    }
}

fn make_auth_error_json_response(msg: &str) -> HttpResponse {
    HttpResponse::Unauthorized().json(make_error_object(msg))
}