use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
};

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const PADDLE_SPEED: f32 = 500.;
const PADDLE_SIZE: Vec3 = Vec3::new(20., 120., 0.);
const BALL_SIZE: Vec3 = Vec3::new(30., 30., 0.);

const WALL_THICKNESS: f32 = 10.;

const WALL_COLOR: Color = Color::WHITE;

const GAP_BETWEEN_PADDLE_AND_WALL: f32 = 20.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(ScoreBoard(0, 0))
        .add_event::<CollisionEvent>()
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                apply_velocity.before(check_for_collisions),
                move_paddles.before(check_for_collisions),
                check_for_collisions,
                move_paddles
                    .before(check_for_collisions)
                    .after(apply_velocity),
            ),
        )
        .add_systems(Update, (update_scoreboard, bevy::window::close_on_esc))
        .run();
}

#[derive(Resource)]
struct ScoreBoard(u8, u8);

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
struct Paddle(u8);

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    location: WallLocation,
}

#[derive(Component, PartialEq, Eq)]
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
            location,
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
        Paddle(0),
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
        Paddle(1),
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
        Velocity((400., 400.).into()),
    ));

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    commands.spawn((
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: 40.,
            color: Color::WHITE,
            ..default()
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.),
            left: Val::Px(10.),
            ..default()
        }),
        Paddle(0),
    ));

    commands.spawn((
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: 40.,
            color: Color::WHITE,
            ..default()
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.),
            left: Val::Px(40.),
            ..default()
        }),
        Paddle(1),
    ));
}

fn move_paddles(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Paddle)>,
    time_step: Res<FixedTime>,
) {
    query.iter_mut().for_each(|(mut transform, paddle)| {
        let mut direction = 0.;

        if (paddle.0 == 0 && keyboard_input.pressed(KeyCode::W))
            || (paddle.0 == 1 && keyboard_input.pressed(KeyCode::I))
        {
            direction += 1.;
        }

        if (paddle.0 == 0 && keyboard_input.pressed(KeyCode::S))
            || (paddle.0 == 1 && keyboard_input.pressed(KeyCode::K))
        {
            direction -= 1.;
        }

        let new_paddle_position =
            transform.translation.y + direction * PADDLE_SPEED * time_step.period.as_secs_f32();

        let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0;
        let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0;

        transform.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
    });
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    query.iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    });
}

fn update_scoreboard(scoreboard: Res<ScoreBoard>, mut query: Query<(&mut Text, &Paddle)>) {
    query.iter_mut().for_each(|(mut text, paddle)| {
        text.sections[0].value = match paddle.0 {
            0 => scoreboard.0.to_string(),
            1 => scoreboard.1.to_string(),
            _ => "error".to_string(),
        };
    });
}

fn check_for_collisions(
    mut scoreboard: ResMut<ScoreBoard>,
    mut ball_query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    mut collider_query: Query<(&Transform, Option<&WallLocation>), (With<Collider>, Without<Ball>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, mut ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    collider_query
        .iter_mut()
        .for_each(|(transform, wall_location)| {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                transform.scale.truncate(),
            );

            if let Some(collision) = collision {
                collision_events.send_default();

                if let Some(wall_location) = wall_location {
                    if *wall_location == WallLocation::Left {
                        scoreboard.1 += 1;
                        ball_transform.translation.x = 0.
                    }
                    if *wall_location == WallLocation::Right {
                        scoreboard.0 += 1;
                        ball_transform.translation.x = 0.
                    }
                }

                let mut reflect_x = false;
                let mut reflect_y = false;

                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.,
                    Collision::Right => reflect_x = ball_velocity.x < 0.,
                    Collision::Top => reflect_y = ball_velocity.y < 0.,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.,
                    Collision::Inside => {}
                }

                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        });
}
