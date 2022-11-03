pub use get_langs::*;
pub use lookup::*;

mod get_langs;
mod lookup;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid API key.")]
    KeyInvalid,
    #[error("This API key has been blocked.")]
    KeyBlocked,
    #[error("Exceeded the daily limit on the number of requests.")]
    DailyReqLimit,
    #[error("The text size exceeds the maximum.")]
    TextTooLong,
    #[error("The specified translation direction is not supported.")]
    LangNotSupported,
}
