use async_graphql::{Context, Request};
use uuid::Uuid;
use std::any::Any;

mod school_id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternalRequestId(Uuid);

pub struct ExecutorBuilder(Request);
impl ExecutorBuilder {
    pub fn new(req: Request) -> Self {
        Self(req)
    }

    pub fn set_req_id(self) -> Self {
        self.data(InternalRequestId(Uuid::new_v4()))
    }
    pub fn set_school_id(self, school_id: Option<Uuid>) -> Self {
        self.data(SchoolId::new(school_id))
    }
    pub fn data<T: Any + Send + Sync>(self, data: T) -> Self {
        Self(self.0.data(data))
    }
    pub fn some_then_data<T: Any + Send + Sync, O>(
        self,
        value: Option<O>,
        mapper: impl FnOnce(O) -> T,
    ) -> Self {
        if let Some(value) = value {
            self.data(mapper(value))
        } else {
            self
        }
    }

    pub fn inner(self) -> Request {
        self.0
    }
}

pub fn req_id(ctx: &Context<'_>) -> Uuid {
    if let Ok(id) = ctx.data::<InternalRequestId>() {
        id.0
    } else {
        logging::warn!("Internal request ID not found");
        Uuid::nil()
    }
}

pub use school_id::{ get_school_id, SchoolId };
