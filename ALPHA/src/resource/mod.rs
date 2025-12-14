#[cfg(feature = "embed")]
use rust_embed::Embed;

#[cfg(feature = "embed")]
#[derive(Embed)]
#[folder = "static/"]
#[prefix = "base/"]
pub struct Resource;
