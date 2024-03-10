use bevy::prelude::*;
use bevy::window::WindowResolution;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use main_lib::startup_system::camera_init::*;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window{
            resolution: WindowResolution::new(700.0, 800.0),
            title: "MineSweeper".to_string(),
            ..default()
        }),
        ..default()
    }));
    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::default());
    app.add_systems(Startup, camera_setup);
    app.run();
}
