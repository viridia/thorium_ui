use std::f32::consts::PI;

use bevy::{
    color::palettes::{self, css},
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    ui,
};
use thorium_ui::{CreateForEach, ListItems, ThoriumUiCorePlugin};

fn main() {
    App::new()
        .init_resource::<List>()
        .init_resource::<Random32>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            ThoriumUiCorePlugin,
        ))
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (close_on_esc, rotate, update_list))
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
                flex_direction: ui::FlexDirection::Column,
                border: ui::UiRect::all(ui::Val::Px(3.)),
                ..default()
            },
            BorderColor(css::ALICE_BLUE.into()),
        ))
        .with_children(|builder| {
            builder.for_each(
                |mut items: InMut<ListItems<String>>, list: Res<List>| {
                    items.clone_from_iter(list.items.iter().cloned());
                },
                move |suit, builder| {
                    let suit = suit.clone();
                    let suit2 = suit.clone();
                    // let suit3 = suit.clone();
                    builder
                        .spawn(Node {
                            border: ui::UiRect::all(ui::Val::Px(3.)),
                            ..default()
                        })
                        .observe(move |_trigger: Trigger<Pointer<Down>>| {
                            println!("Clicked on {}", suit);
                        })
                        .with_children(|builder| {
                            builder.spawn(Text::new(suit2));
                        });
                },
                |builder| {
                    builder.spawn(Text::new("No items"));
                },
            );
        });
}

const SUITS: &[&str] = &["hearts", "spades", "clubs", "diamonds"];

#[derive(Resource, Default)]
pub struct List {
    pub items: Vec<String>,
}

fn update_list(
    mut list: ResMut<List>,
    key: Res<ButtonInput<KeyCode>>,
    mut random: ResMut<Random32>,
) {
    if key.just_pressed(KeyCode::Space) {
        println!("-- Space pressed --");
        let i = (random.next() as usize) % SUITS.len();
        list.items.push(SUITS[i].to_string());
        while list.items.len() > 10 {
            list.items.remove(0);
        }
    } else if key.just_pressed(KeyCode::Minus) {
        println!("-- Minus pressed --");
        list.items.pop();
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

#[derive(Resource)]
struct Random32 {
    state: u32,
}

impl Random32 {
    // Generate a pseudo-random number
    fn next(&mut self) -> u32 {
        // Constants for 32-bit LCG (example values, you might want to choose different ones)
        let a: u32 = 1664525; // Multiplier
        let c: u32 = 1013904223; // Increment
        let m: u32 = 2u32.pow(31); // Modulus, often set to 2^31 for a 32-bit generator

        // Simple LCG formula: X_{n+1} = (aX_n + c) mod m
        self.state = (a.wrapping_mul(self.state).wrapping_add(c)) % m;
        self.state
    }
}

impl Default for Random32 {
    fn default() -> Self {
        Self { state: 17 }
    }
}
