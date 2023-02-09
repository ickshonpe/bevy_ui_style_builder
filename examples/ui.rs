//! This example illustrates the various features of Bevy UI.

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings,
};

use bevy_ui_style_builder::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(mouse_scroll)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // root node
    commands
        .spawn(
            node()
            .width(Val::Percent(100.))
            .height(Val::Percent(100.))
            .justify_space_between()
        )
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(
                    node()
                    .width(Val::Px(200.))
                    .height(Val::Percent(100.))
                    .border(Breadth::Px(2.))
                    .background_color(Color::rgb(0.65, 0.65, 0.65))
                )
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(
                            node()
                            .width(Val::Px(196.))
                            .height(Val::Percent(100.))
                            .background_color(Color::rgb(0.15, 0.15, 0.15))
                        )
                        .with_children(|parent| {
                            // text
                            parent.spawn(
                                TextBundle::from_section(
                                    "Text Example",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(style().margin(Val::Px(5.)))
                            );
                        });
                });
            // right vertical fill
            parent
                .spawn(
                    node()
                    .column()
                    .justify_center()
                    .align_center()
                    .size(Size::new(Val::Px(200.0), Val::Percent(100.0)))
                    .background_color(Color::rgb(0.15, 0.15, 0.15))
                )
                .with_children(|parent| {
                    // Title
                    parent.spawn(
                        TextBundle::from_section(
                            "Scrolling list",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(
                            style().size(Size::new(Val::Undefined, Val::Px(25.)))
                        )
                    );
                    // List with hidden overflow
                    parent
                        .spawn(node()
                                .column()
                                .size(Size::new(Val::Percent(100.0), Val::Percent(50.0)))
                                .hide_overflow()
                            .background_color(Color::rgb(0.10, 0.10, 0.10))
                        )
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    node().column().grow(1.0).max_size(Size::UNDEFINED),
                                    ScrollingList::default(),
                                ))
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent.spawn(
                                            TextBundle::from_section(
                                                format!("Item {i}"),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.,
                                                    color: Color::WHITE,
                                                }
                                            )
                                            .shrink(0.)
                                            .height(Val::Px(20.))
                                            .margin(UiRect::horizontal(Val::Auto))
                                        );
                                    }
                                });
                        });
                });
            parent
                .spawn(
                node()
                    .size(Size::new(Val::Px(200.0), Val::Px(200.0)))
                    .absolute()
                    .left(Val::Px(210.0))
                    .bottom(Val::Px(10.0))
                    .border(NumRect::all(Breadth::Px(20.0)))
                    .background_color(Color::rgb(0.4, 0.4, 1.0))
                )
                .with_children(|parent| {
                    parent.spawn(
                        node()
                        .size(Size::new(Val::Percent(100.0), Val::Percent(100.0)))
                        .background_color(Color::rgb(0.8, 0.8, 1.0))
                    );
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn(
                    node()
                    .width(Val::Percent(100.0))
                    .height(Val::Percent(100.0))
                    .absolute()
                    .align_center()
                    .justify_center()
                )
                .with_children(|parent| {
                    parent
                        .spawn(node()
                            .size(Size::new(Val::Px(100.0), Val::Px(100.0)))
                            .background_color(Color::rgb(1.0, 0.0, 0.0))
                        )
                        .with_children(|parent| {
                            parent.spawn(
                                node()
                                .size(Size::new(Val::Px(100.0), Val::Px(100.0)))
                                .absolute()
                                .left(Val::Px(20.0))
                                .bottom(Val::Px(20.0))
                                .background_color(Color::rgb(1.0, 0.3, 0.3))
                            );
                            parent.spawn(
                                node()
                                .size(Size::new(Val::Px(100.0), Val::Px(100.0)))
                                .absolute()
                                .left(Val::Px(40.0))
                                .bottom(Val::Px(40.0))
                                .background_color(Color::rgb(1.0, 0.5, 0.5))
                            );
                            parent.spawn(
                                node()
                                .size(Size::new(Val::Px(100.0), Val::Px(100.0)))
                                .absolute()
                                .left(Val::Px(60.0))
                                .bottom(Val::Px(60.0))
                                .background_color(Color::rgb(1.0, 0.7, 0.7))
                            );
                            // alpha test
                            parent.spawn(
                                node()
                                .size(Size::new(Val::Px(100.0), Val::Px(100.0)))
                                .absolute()
                                .left(Val::Px(80.0))
                                .bottom(Val::Px(80.0))
                                .background_color(Color::rgba(1.0, 0.9, 0.9, 0.4))
                            );
                });
            // bevy logo (flex center)
            parent
                .spawn(
                    node()
                    .size(Size::new(Val::Percent(100.0), Val::Percent(100.0)))
                    .absolute()
                    .justify_center()
                    .align_start()
                )
                .with_children(|parent| {
                    // bevy logo (image)
                    parent.spawn(
                        ImageBundle {
                            image: asset_server.load("branding/bevy_logo_dark_big.png").into(),
                            ..default()
                        }
                        .size(Size::new(Val::Px(500.0), Val::Auto))
                    );
                });
        });
    });
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size().y)
                .sum();
            let panel_height = uinode.size().y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
