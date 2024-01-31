/// Player state structures
pub mod controller;

/// Player & editor cameras
pub mod camera;

/// Develper runtime console
pub mod console;

/// UI debugger
pub mod debugger;


/// Boxy prelude
pub mod prelude {
    pub use crate::camera::*;
    pub use crate::controller::*;
    pub use crate::console::*;
    pub use crate::debugger::*;
}
