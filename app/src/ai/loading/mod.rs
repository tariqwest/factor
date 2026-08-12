//! Loading animation components for AI features.

mod shimmering_warp_loading_text;
mod warper_loader;
pub use shimmering_warp_loading_text::shimmering_warp_loading_text;
pub use warper_loader::{factor_loader_icon, WARPER_LOADER_FRAME_COUNT};

/// Alias kept for any remaining Warper-era call sites.
pub use warper_loader::factor_loader_icon as warper_loader_icon;
