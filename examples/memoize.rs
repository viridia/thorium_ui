use std::f32::consts::PI;

use bevy::{
    color::palettes::{self, css},
    ecs::world::DeferredWorld,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    ui,
};
use thorium_ui::{CreateCond2, CreateMemo, ReadMemo, ThoriumUiCorePlugin};

fn main() {
    App::new()
        .init_resource::<Counter>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            ThoriumUiCorePlugin,
        ))
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (close_on_esc, rotate, update_counter))
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

fn setup_view_root(mut commands: Commands) {
    commands
        .spawn((
            Node {
                left: ui::Val::Px(0.),
                top: ui::Val::Px(0.),
                right: ui::Val::Px(0.),
                // bottom: ui::Val::Px(0.),
                position_type: ui::PositionType::Absolute,
                display: ui::Display::Flex,
                flex_direction: ui::FlexDirection::Row,
                border: ui::UiRect::all(ui::Val::Px(3.)),
                ..default()
            },
            BorderColor(css::ALICE_BLUE.into()),
        ))
        .with_children(|builder| {
            let even = builder.create_memo(|counter: Res<Counter>| counter.count & 1 == 0, false);
            builder.cond2(
                move |world: DeferredWorld| world.read_memo(even),
                || Spawn(Text::new("even")),
                || Spawn(Text::new("odd")),
            );
        });
}

#[derive(Resource, Default)]
pub struct Counter {
    pub count: u32,
}

fn update_counter(mut counter: ResMut<Counter>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::Space) {
        counter.count += 1;
    }
}

// Setup 3d shapes
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(shape),
            MeshMaterial3d(debug_material.clone()),
            Transform::from_xyz(
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                2.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            Shape,
        ));
    }

    commands.spawn((
        PointLight {
            // intensity: 9000.0,
            intensity: 10000000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::from(palettes::css::SILVER))),
    ));

    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
