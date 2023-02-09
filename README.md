# bevy_ui_style_builder
[![crates.io](https://img.shields.io/crates/v/bevy_ui_style_builder)](https://crates.io/crates/bevy_ui_style_builder)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_ui_style_builder)
[![crates.io](https://img.shields.io/crates/d/bevy_ui_style_builder)](https://crates.io/crates/bevy_ui_style_builder)

Experimental Bevy UI helper extension methods.

Supports Bevy 0.9

# Usage

Add the dependency to your project:

```
cargo add bevy_ui_style_builder
```

Then the following example draws a red rectangle in the middle of the window:

```rust
use bevy::prelude::*;
use bevy_ui_style_builder::prelude::*;

fn spawn_example(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        node()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .justify_content_center()
        .align_items_center()
    ).with_children(|builder| {
        builder.spawn(
            node()
            .width(Val::Px(150.0))
            .height(Val::Px(100.0))
            .color(Color::RED)
        );
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_example)
        .run();
}
```

There is a larger example based on Bevy's UI example in the examples folder.
You can run it with:

```
cargo run --example ui
```