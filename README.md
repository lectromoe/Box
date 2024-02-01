## Tools for bevy out-of-the-box

Plug-and-play tools to quickly integrate into your project

### Goals
1. Simple modular architechture 
2. WASM as a first-class citizen
3. Networking

### Tools
- Character controller `BoxyControllerPlugin`
- Universal camera `BoxyCameraPlugin`
- Physics `BoxyPhysicsPlugin` (using `bevy_rapier`)
- In-game dev console (TODO)
- Debug info UI (TODO)
- Examples (TODO)

### How to use
1. Add the plugins

```rust
use bevy::prelude::*;
use boxy::prelude::*;

fn main() {
    App::new()
        .add_plugins(BoxyControllerPlugin)
        .add_plugins(BoxyPhysicsPlugin)
        .add_plugins(BoxyCameraPlugin)
        /// ...
        .run();
}
```

2. Start a dev server for examples using [trunk](https://github.com/thedodd/trunk)
```bash
trunk serve
```

3. For produciton deploy see [`github/workflows/main.yml`](.github/workflows/main.yml) 

Live example build is available at [kualta.github.io/boxy](https://kualta.github.io/boxy)