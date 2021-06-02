use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use actix_web::{
    dev::{
        Service,
        ServiceRequest,
        ServiceResponse,
        Transform,
    },
    error::Error as AWError,
    http::HeaderName,
    http::HeaderValue,
    HttpMessage,
};
use futures::future::{ok, Ready};
use log::*;

use crate::Foo;

#[derive(Clone)]
pub struct FoobarMiddleware;

impl FoobarMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> Transform<S> for FoobarMiddleware
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=AWError> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type InitError = ();
    type Transform = FoobarService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(FoobarService {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct FoobarService<S>
    where
        S: 'static,
{
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for FoobarService<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=AWError> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.borrow_mut().poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let foobar = req.extensions().get::<Foo>().map(|foo| HeaderValue::from_str(&foo.bar).unwrap());
        debug!("foobar: {:?}", &foobar);
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            if let Some(value) = foobar {
                let name = HeaderName::from_static("x-foobar");
                res.headers_mut().insert(name, value);
            }
            Ok(res)
        })
    }
}
