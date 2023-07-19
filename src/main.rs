use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::{PresentMode, WindowTheme},
    winit::WinitSettings,
};
use rand::distributions::{Distribution, Uniform};

use itertools::Itertools;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Table Demo".into(),
                    resolution: (768., 512.).into(),
                    present_mode: PresentMode::AutoNoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(WinitSettings::game())
        .add_systems(Startup, setup)
        .add_systems(Update, randomize_values)
        .run();
}

#[derive(Debug, Copy, Clone)]
struct Table {
    num_cols: u32,
    num_rows: u32,
}

impl From<Table> for Mesh {
    fn from(_table: Table) -> Self {
        Mesh::from(shape::Quad::new(Vec2::new(50., 15.)))
    }
}

#[derive(Component)]
struct Cell;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    let table = Table {
        num_cols: 60,
        num_rows: 100,
    };
    let p = 2.;
    let w = -2. * p as f32 + width / table.num_cols as f32;
    let h = -2. * p as f32 + height / table.num_rows as f32;
    let positions = (0..=table.num_cols)
        .cartesian_product(0..=table.num_rows)
        .map(|(x, y)| [x as f32 * (w + p), y as f32 * (h + p)])
        .collect::<Vec<_>>();
    let between = Uniform::try_from(0.0..1.0).unwrap();
    let mut rng = rand::thread_rng();
    for point in positions {
        let v = between.sample(&mut rng);

        let mut mesh = Mesh::from(shape::Quad::new(Vec2::new(w, h)));
        let c = (if v > 0.5 { Color::GREEN } else { Color::RED }).as_rgba_f32();
        let vertex_colors: Vec<[f32; 4]> = vec![c, c, c, c];
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
        let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: materials.add(ColorMaterial::default()),
                transform: Transform::from_translation(Vec3::new(
                    point[0] - width / 3.,
                    point[1] - height / 3.,
                    0.,
                )),
                ..default()
            })
            .insert(Cell);
    }
}

fn randomize_values(query: Query<&Mesh2dHandle, With<Cell>>, mut meshes: ResMut<Assets<Mesh>>) {
    let between = Uniform::try_from(0.0..1.0).unwrap();
    let mut rng = rand::thread_rng();
    for handle in &query {
        let v = between.sample(&mut rng);
        let mesh = meshes.get_mut(&handle.0).expect("getting mesh");
        let c = (if v > 0.5 { Color::GREEN } else { Color::RED }).as_rgba_f32();
        let vertex_colors: Vec<[f32; 4]> = vec![c, c, c, c];
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
    }
}
