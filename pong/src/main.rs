use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const PADDLE_SIZE: Vec3 = Vec3::new(20., 120., 0.);
const BALL_SIZE: Vec3 = Vec3::new(30., 30., 0.);

const WALL_THICKNESS: f32 = 10.;

const WALL_COLOR: Color = Color::WHITE;

const GAP_BETWEEN_PADDLE_AND_WALL: f32 = 20.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.),
                    scale: location.size().extend(1.),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let player1 = LEFT_WALL + GAP_BETWEEN_PADDLE_AND_WALL;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(player1, 0., 0.),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));

    let player2 = RIGHT_WALL - GAP_BETWEEN_PADDLE_AND_WALL;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(player2, 0., 0.),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(WALL_COLOR)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity((1., 1.).into()),
    ));

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}
