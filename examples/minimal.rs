use bevy::prelude::*;
use bevy_ui_style_builder::prelude::*;

fn spawn_example(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(node()
            .width(Val::Percent(100.0))
            .height(Val::Percent(100.0))
            .justify_content_center()
            .align_items_center()
        )
        .with_children(|builder| {
            builder.spawn(node()
                .width(Val::Px(150.0))
                .height(Val::Px(100.0))
                .background_color(Color::RED),
            );
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_example)
        .run();
}
