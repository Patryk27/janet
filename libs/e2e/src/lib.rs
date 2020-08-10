pub use self::{framework::*, prelude::*};
use std::future::Future;
use std::sync::Arc;

mod framework;
mod prelude;

pub async fn test<Fun, Fut>(test: Fun)
where
    Fun: FnOnce(Arc<TestContext>) -> Fut,
    Fut: Future<Output = ()>,
{
    let ctxt = Arc::new(TestContext::create().await);

    (test)(Arc::clone(&ctxt)).await;

    let TestContext { gitlab, janet } = Arc::try_unwrap(ctxt).unwrap();

    // Kill Janet
    let (stdout, stderr) = janet.kill().await;

    println!("===== Janet's stdout =====");
    println!();
    println!("{}", stdout);
    println!();
    println!("===== Janet's stderr =====");
    println!();
    println!("{}", stderr);

    // Kill GitLab server
    drop(gitlab);
}
