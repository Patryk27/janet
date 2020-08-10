pub use self::{janet::*, test_context::*};

pub const CONFIG: &str = indoc::indoc!(
    r#"
    [bot]
    name = "janet"

    [database]
    path = "{{ database_path }}"

    [http]
    addr = "127.0.0.1:10000"

    [log]

    [gitlab]
    url = "{{ gitlab_url }}"
    personal_access_token = "token"
    webhook_secret = "secret"
"#
);

pub const TMP_DIR: &str = "/tmp/janet";

mod janet;
mod test_context;
