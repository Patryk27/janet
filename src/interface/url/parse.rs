use crate::interface::{Parse, Url};
use nom::bytes::complete::take_till1;
use nom::error::ErrorKind;
use nom::Err;
use nom::IResult;

impl Parse for Url {
    fn parse(i: &str) -> IResult<&str, Self> {
        let i2 = <&str>::clone(&i);
        let (i, url) = url(i)?;

        if url::Url::parse(url).is_ok() {
            Ok((i, Self::new(url)))
        } else {
            Err(Err::Error((i2, ErrorKind::Verify)))
        }
    }
}

fn url(i: &str) -> IResult<&str, &str> {
    take_till1(|i| i == ' ' || i == ')' || i == ']')(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert(input: &str) {
        let expected = Ok(("", Url::new(input)));
        let actual = Url::parse(input);

        assert_eq!(expected, actual, "Input: {}", input);
    }

    mod with_scheme {
        use super::*;

        #[test]
        fn of_http() {
            assert("http://gitlab.com");
        }

        #[test]
        fn of_https() {
            assert("https://gitlab.com");
        }
    }

    mod with_authority {
        use super::*;

        mod with_host {
            use super::*;

            #[test]
            fn of_name() {
                assert("http://site.com");
                assert("http://subdomain.site.com");
                assert("http://subsubdomain.subdomain.site.com");
            }

            #[test]
            fn of_ip() {
                assert("http://192.168.1.1");
            }
        }

        #[test]
        fn with_port() {
            assert("http://site.com:123");
            assert("http://192.168.1.1:123");
        }
    }

    mod with_path {
        use super::*;

        #[test]
        fn present() {
            assert("https://127.0.0.1/patryk/janet/-/merge_requests/123");
            assert("https://somewhere.gitlab.com/patryk/janet/-/merge_requests/123");
        }

        mod and_query {
            use super::*;

            #[test]
            fn present() {
                assert("https://127.0.0.1/patryk/janet/-/merge_requests/123?foo=bar&zar=dar");
                assert("https://somewhere.gitlab.com/patryk/janet/-/merge_requests/123?foo=bar&zar=dar");
            }
        }
    }
}
