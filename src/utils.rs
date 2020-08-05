use anyhow::*;
use std::future::Future;
use tokio::task;

/// Spawns future in the background and flattens resulting `JoinError`.
/// Makes using `try_join!` way nicer.
pub async fn spawn_future<T, F>(fut: F) -> Result<T>
where
    T: 'static,
    F: Future<Output = Result<T>> + Send + 'static,
    F::Output: Send + 'static,
{
    task::spawn(fut).await?
}

#[cfg(test)]
pub fn to_json<T: serde::Serialize>(model: &T) -> String {
    serde_json::to_string(model).unwrap()
}
