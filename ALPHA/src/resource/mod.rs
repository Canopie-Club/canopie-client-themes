use include_dir::{Dir, include_dir};
#[cfg(feature = "embed")]
use rust_embed::Embed;

#[cfg(feature = "embed")]
#[derive(Embed)]
#[folder = "static/"]
#[prefix = "base/"]
pub struct Resource;

static PROJECT_DIR: Dir<'_> = include_dir!("static/");
