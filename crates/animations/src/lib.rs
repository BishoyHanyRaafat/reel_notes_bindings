//! Pure animation math for sampling segment motion.
//!
//! This crate owns the time-based calculations that turn animation definitions
//! into a renderable layer state. Rendering code should stay focused on drawing
//! and compositing the returned values.

mod easing;
mod segment;
mod state;

pub use segment::evaluate_segment_animation;
pub use state::LayerAnimation;
