use bevy::prelude::*;
use the_tanuki_trip_lib::plugins::SideEffectsPlugin;
fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SideEffectsPlugin)
        .run();
}
