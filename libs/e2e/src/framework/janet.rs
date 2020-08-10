use anyhow::*;
use reqwest::Client;
use serde::Serialize;
use std::path::Path;
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::{process, time};

pub struct Janet {
    url: String,
    child: process::Child,
    client: reqwest::Client,
}

impl Janet {
    pub async fn start(config: impl AsRef<Path>) -> Self {
        // TODO this path should automatically detect debug vs release
        let process = Path::new("../../target/debug/janet");

        let config = config.as_ref();

        // TODO this port should be random
        let url = "http://127.0.0.1:10000".into();

        let child = process::Command::new(process)
            .arg("-c")
            .arg(config)
            .arg("--sync")
            .stdout(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| format!("Couldn't launch Janet: {}", process.display()))
            .unwrap();

        let this = Self {
            url,
            child,
            client: Client::new(),
        };

        this.wait_until_reachable().await;
        this
    }

    pub async fn spoof_gitlab_webhook(&self, body: &impl Serialize) {
        let body = serde_json::to_string(body).unwrap();

        self.client
            .post(&format!("{}/webhooks/gitlab", self.url))
            .body(body)
            .send()
            .await
            .context("Couldn't send request")
            .unwrap()
            .error_for_status()
            .context("Got an unexpected response status code")
            .unwrap();
    }

    pub async fn kill(mut self) -> (String, String) {
        self.child.kill().context("Couldn't kill Janet").unwrap();

        let stdout = if let Some(mut stream) = self.child.stdout.take() {
            let mut out = String::new();

            stream
                .read_to_string(&mut out)
                .await
                .context("Couldn't read Janet's stdout")
                .unwrap();

            out
        } else {
            String::default()
        };

        let stderr = if let Some(mut stream) = self.child.stderr.take() {
            let mut out = String::new();

            stream
                .read_to_string(&mut out)
                .await
                .context("Couldn't read Janet's stderr")
                .unwrap();

            out
        } else {
            String::default()
        };

        (stdout, stderr)
    }

    async fn wait_until_reachable(&self) {
        let mut i = 0usize;

        loop {
            if i > 500 {
                panic!("Janet failed to start in time");
            }

            time::delay_for(time::Duration::from_millis(10)).await;

            if self
                .client
                .get(&format!("{}/health", self.url))
                .send()
                .await
                .is_ok()
            {
                break;
            }

            i += 1;
        }
    }
}

impl Drop for Janet {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
