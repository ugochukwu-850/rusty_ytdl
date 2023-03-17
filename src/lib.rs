#![recursion_limit = "256"]
// #![allow(dead_code)]
// #![allow(unused_imports)]

mod info;
mod info_extras;
mod structs;
mod utils;

pub mod constants;

#[cfg(feature = "blocking")]
pub mod blocking;

pub use info::Video;
pub use structs::{
    DownloadOptions, RequestOptions, VideoError, VideoOptions, VideoQuality, VideoSearchOptions,
};
pub use utils::{choose_format, get_video_id};
// export to access proxy feature
pub use reqwest;
