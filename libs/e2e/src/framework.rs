pub use self::{janet::*, test_context::*};

pub const CONFIG: &str = indoc::indoc!(
    r#"
    [bot]
    name = "janet"

    [database]
    path = "{{ database.path }}"

    [gitlab]
    url = "{{ gitlab.url }}"
    personal_access_token = "token"
    webhook_secret = "secret"

    [http]
    addr = "{{ http.addr }}"

    [log]
"#
);

mod janet;
mod test_context;
