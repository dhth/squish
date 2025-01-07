pub(crate) const UNEXPECTED_ERROR_MESSAGE: &str =
    "this isn't supposed to happen, let @dhth know about this via https://github.com/dhth/squish/issues";

pub(crate) fn get_unexp_err_msg(msg: &str) -> String {
    format!("{}; {}", msg, UNEXPECTED_ERROR_MESSAGE)
}
