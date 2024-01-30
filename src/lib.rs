/// Player state structures
pub mod character_state;

/// Player & editor cameras
pub mod camera;

/// Develper runtime console
pub mod console;

/// UI debugger
pub mod debugger;

/// Character controller
pub mod controller;

/// Boxy prelude
pub mod prelude {
    pub use crate::camera::*;
    pub use crate::controller::*;
    pub use crate::character_state::*;
    pub use crate::console::*;
    pub use crate::debugger::*;
}
