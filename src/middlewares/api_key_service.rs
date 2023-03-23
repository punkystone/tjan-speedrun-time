use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

pub struct ApiKeyService {
    pub api_key: String,
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyService
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyServiceMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyServiceMiddleware {
            service: Rc::new(service),
            api_key: self.api_key.clone(),
        }))
    }
}
#[allow(clippy::module_name_repetitions)]
pub struct ApiKeyServiceMiddleware<S> {
    api_key: String,
    service: Rc<S>,
}

impl<S, B> ApiKeyServiceMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    fn response_to_string(
        response_string: &'static str,
        request: ServiceRequest,
    ) -> LocalBoxFuture<'static, Result<ServiceResponse<EitherBody<B>>, Error>> {
        let (request, _) = request.into_parts();
        Box::pin(async move {
            Ok(ServiceResponse::new(
                request,
                HttpResponse::Unauthorized()
                    .body(response_string)
                    .map_into_right_body(),
            ))
        })
    }
}

impl<S, B> Service<ServiceRequest> for ApiKeyServiceMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        if request.path() == "/place/decrement" {
            match request.headers().get("API-KEY") {
                Some(api_key) => {
                    let Ok(api_key) = api_key.to_str() else {
                    return Self::response_to_string("Api Key Invalid", request);
                    };

                    if self.api_key != api_key {
                        return Self::response_to_string("Api Key Invalid", request);
                    }
                }
                None => {
                    return Self::response_to_string("Api Key Missing", request);
                }
            };
        }

        let res = self.service.call(request);

        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
