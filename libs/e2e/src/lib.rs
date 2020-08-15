pub use self::{framework::*, prelude::*};
use std::any::Any;
use std::future::Future;

mod framework;
mod prelude;

/// Performs a single integration test.
///
/// Accepts an async closure which is launched with a `TestContext`,
/// allowing the test to operate on live Janet's process.
pub async fn test<Fun, Fut>(test: Fun)
where
    Fun: FnOnce(TestContext) -> Fut + Any,
    Fut: Future<Output = ()>,
{
    let ctxt = TestContext::create().await.unwrap();

    (test)(ctxt).await;
}
