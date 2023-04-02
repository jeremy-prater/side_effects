use bevy::prelude::*;
use side_effects_lib::plugins::SideEffectsPlugin;
fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SideEffectsPlugin)
        // .add_startup_system(setup)
        .run();
}