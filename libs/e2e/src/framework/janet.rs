use anyhow::*;
use reqwest::Client;
use serde::Serialize;
use std::path::Path;
use std::process::Stdio;
use std::thread;
use tokio::io::AsyncReadExt;
use tokio::{process, time};

pub struct Janet {
    url: String,
    child: process::Child,
    client: reqwest::Client,
}

impl Janet {
    pub async fn start(addr: impl AsRef<str>, config: impl AsRef<Path>) -> Result<Self> {
        #[cfg(debug_assertions)]
        let process = Path::new("target/debug/janet");

        #[cfg(not(debug_assertions))]
        let process = Path::new("target/release/janet");

        let url = format!("http://{}", addr.as_ref());
        let config = config.as_ref();

        let child = process::Command::new(process)
            .arg("--config")
            .arg(config)
            .arg("--sync")
            .stdout(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| format!("Couldn't launch Janet: {}", process.display()))?;

        let this = Self {
            url,
            child,
            client: Client::new(),
        };

        this.wait_until_reachable().await?;

        Ok(this)
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

    pub async fn kill(&mut self) -> Result<(String, String)> {
        self.child.kill()?;

        let stdout = if let Some(mut stream) = self.child.stdout.take() {
            let mut out = String::new();

            stream
                .read_to_string(&mut out)
                .await
                .context("Couldn't read stdout")?;

            out
        } else {
            String::default()
        };

        let stderr = if let Some(mut stream) = self.child.stderr.take() {
            let mut out = String::new();

            stream
                .read_to_string(&mut out)
                .await
                .context("Couldn't read stderr")?;

            out
        } else {
            String::default()
        };

        Ok((stdout, stderr))
    }

    async fn wait_until_reachable(&self) -> Result<()> {
        let mut i = 0usize;

        loop {
            if i > 500 {
                bail!("Janet failed to start in time");
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

        Ok(())
    }
}

impl Drop for Janet {
    fn drop(&mut self) {
        futures::executor::block_on(async {
            let (stdout, stderr) = self.kill().await.context("Couldn't kill Janet").unwrap();

            if !stdout.is_empty() {
                println!("===== Janet's stdout =====");
                println!("{}", stdout);
                println!();
            }

            if !stderr.is_empty() {
                println!("===== Janet's stderr =====");
                println!("{}", stderr);
                println!();
            }

            if !thread::panicking() && stdout.contains("ERROR") {
                panic!("Janet's logs contain an unexpected error");
            }
        });
    }
}
