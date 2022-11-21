/// Player state structures
pub mod character_state;

/// Player & editor cameras
pub mod camera;

/// Develper runtime console
pub mod console;

/// UI debugger
pub mod debugger;

/// Character controller
pub mod character;

/// Boxxed prelude
pub mod prelude {
    pub use crate::camera::*;
    pub use crate::character::*;
    pub use crate::character_state::*;
    pub use crate::console::*;
    pub use crate::debugger::*;
}
