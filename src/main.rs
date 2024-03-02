use bevy::app::{App, PreStartup};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct LeftCamera;

#[derive(Component)]
struct RightCamera;

#[derive(Component)]
struct MouseText;

fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        LeftCamera,
    ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10., 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                // don't clear on the second camera because the first camera already cleared the
                // window
                clear_color: ClearColorConfig::None,
                // Renders the right camera after the left camera, which has a default priority
                // of 0
                order: 1,
                ..default()
            },
            ..default()
        },
        RightCamera,
    ));

    commands.spawn((
        MouseText,
        TextBundle {
            text: Text::from_section("hello world", TextStyle {
                ..default()
            }),
            style: Style {
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..default()
            },
            ..default()
        }
    ));
}

fn update(
    mut commands: Commands,
    camera_query: Query<(Entity, &Camera)>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut mouse_text_query: Query<Entity, With<MouseText>>,
) {
    let Ok(primary_window) = primary_window_query.get_single() else {
        return;
    };
    let Some(position) = primary_window.cursor_position() else {
        return;
    };
    for (camera_entity, camera) in camera_query.iter() {
        let Some(viewport_size) = camera.logical_viewport_rect() else {
            continue;
        };
        if !viewport_size.contains(position) {
            continue;
        }
        let viewport_pos = position - viewport_size.min;
        for entity in mouse_text_query.iter_mut() {
            commands
                .entity(entity)
                .insert(TargetCamera(camera_entity))
                .insert(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(viewport_pos.y),
                    left: Val::Px(viewport_pos.x),
                    ..default()
                })
            ;
        }
    }
}

fn camera_viewport(
    windows: Query<&Window>,
    mut resize_events: EventReader<bevy::window::WindowResized>,
    mut left_camera: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
    mut right_camera: Query<&mut Camera, With<RightCamera>>,
) {
// We need to dynamically resize the camera's viewports whenever the window size changes so then
    // each camera always takes up half the screen. A resize_event is sent when the window is first
    // created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let mut left_camera = left_camera.single_mut();
        left_camera.viewport = Some(bevy::render::camera::Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(
                window.resolution.physical_width() / 2,
                window.resolution.physical_height(),
            ),
            ..default()
        });

        let mut right_camera = right_camera.single_mut();
        right_camera.viewport = Some(bevy::render::camera::Viewport {
            physical_position: UVec2::new(window.resolution.physical_width() / 2, 0),
            physical_size: UVec2::new(
                window.resolution.physical_width() / 2,
                window.resolution.physical_height(),
            ),
            ..default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(PreStartup, setup)
        .add_systems(PreUpdate, camera_viewport)
        .add_systems(Update, update)
        .run()
    ;
}
