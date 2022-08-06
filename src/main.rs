mod constants;
mod components;

use bevy::{prelude::*, sprite::collide_aabb::collide, math::{Vec3Swizzles, vec2}, time::FixedTimestep};
use components::{Board, Snake, LastEntity, SnakeHead, Block, BlockCount, SnakeDirection, GameStep};
use constants::{GAME_AREA_WIDTH, GAME_AREA_HEIGHT, TIME_STEP, GAME_AREA_STEP};
use rand::{thread_rng, Rng};

use crate::components::{MotionTimer, PreviousEntity};


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_system)
    .add_startup_system(initial_snake_spawn_system)
    .add_system_set(
        SystemSet::new().with_run_criteria(FixedTimestep::step(2.))
        .with_system(random_background_color_system)
    )
    .insert_resource(BlockCount(0))
    .insert_resource(GameStep::default())
    .add_system(block_spawn_system)
    // .add_system_set(
    //     SystemSet::new().with_run_criteria(FixedTimestep::step(2.))
    //     .with_system(snake_movement_system)
    // )
    // .add_system(snakehead_movement_system)
    .add_system(keyboard_system)
    .add_system(collide_system)
    .add_system_set(
        SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        .with_system(snakehead_movement_system)
        .with_system(snake_movement_system.after(snakehead_movement_system))
    )
    // .add_system(snake_movement_system)
    .insert_resource(MotionTimer { timer: Timer::from_seconds(0.05, true) })
    .run();
}


fn setup_system(
    mut commands: Commands
) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(GAME_AREA_WIDTH, GAME_AREA_HEIGHT)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Board);

    // commands.insert_resource(LastEntity { block: entity })
}

fn random_background_color_system(
    mut query: Query<&mut Sprite, With<Board>>,
) {
    let board = query.get_single_mut();
        match board {
            Ok(mut sprite) => {
                let r = rand::random::<f32>();
                let g = rand::random::<f32>();
                let b = rand::random::<f32>();

                sprite.color = Color::rgb(r, g, b);

            },
            _ => {}

        }
}

fn initial_snake_spawn_system(
    mut commands: Commands
) {
    
    let entity = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 },
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    // .insert(Snake)
    .insert(SnakeHead )
    .id();

    commands.insert_resource(LastEntity { block: entity });
    commands.insert_resource(SnakeDirection::Up);
}

fn block_spawn_system(
    mut commands: Commands,
    mut block_count: ResMut<BlockCount>,
) {

    if block_count.0 == 0 {
        let mut rng = thread_rng();
        let random_x = rng.gen_range(-GAME_AREA_WIDTH / 2.1 ..GAME_AREA_WIDTH / 2.1);
        let random_y = rng.gen_range(-GAME_AREA_HEIGHT / 2.1 ..GAME_AREA_HEIGHT / 2.1);
        // let y = rand::random::<f32>();

        // println!("{:}", GAME_AREA_WIDTH);
        // println!("{:}", GAME_AREA_HEIGHT);
        
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::TOMATO,
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(random_x, random_y, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Block);

        block_count.0 = 1;
    }
}

fn snakehead_movement_system(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<SnakeHead>>,
    time: Res<Time>,
    direction: ResMut<SnakeDirection>,
    mut motion_timer: ResMut<MotionTimer>,
    game_step: Res<GameStep>
) {

    for mut elem in query.iter_mut() {
        match *direction {
                SnakeDirection::Left => {
                    elem.translation.x -= game_step.0;
                    if elem.translation.x < -GAME_AREA_WIDTH / 2. {
                        elem.translation.x = GAME_AREA_WIDTH / 2.;
                    }
                },
                SnakeDirection::Right => {
                    elem.translation.x += game_step.0;
                    if elem.translation.x > GAME_AREA_WIDTH / 2. {
                        elem.translation.x = -GAME_AREA_WIDTH / 2.
                    }
                },
                SnakeDirection::Up => {
                    elem.translation.y += game_step.0;
                    if elem.translation.y > GAME_AREA_HEIGHT / 2. - 10. {
                        elem.translation.y = -GAME_AREA_HEIGHT / 2. + 10.
                    }
                },
                SnakeDirection::Down => {
                    elem.translation.y -= game_step.0;
                    if elem.translation.y < -GAME_AREA_HEIGHT / 2. - 10. {
                        elem.translation.y = GAME_AREA_HEIGHT / 2. + 10.
                    }
                },
            }
    }
}

fn snake_movement_system(
    mut snake_query: Query<&mut Transform, With<Snake>>,
    prev_query: Query<(&Transform, &SnakeHead), Without<Snake>>,

) {
    let snake_head = prev_query.get_single().unwrap();
    let mut prev_position = Vec2::new(snake_head.0.translation.x, snake_head.0.translation.y);

    for (index, mut snake_tf) in snake_query.iter_mut().enumerate() {
        let next_position = prev_position.clone();
        prev_position.x = snake_tf.translation.x;
        prev_position.y = snake_tf.translation.y;
        snake_tf.translation.y = next_position.y;
        snake_tf.translation.x = next_position.x;
    }
}

fn keyboard_system(
    mut query: Query<&Transform, With<SnakeHead>>,
    key: Res<Input<KeyCode>>,
    mut direction: ResMut<SnakeDirection>
) {

    for _ in query.iter_mut() {
        if key.just_pressed(KeyCode::Left) {
            *direction = SnakeDirection::Left;
        }
        if key.just_pressed(KeyCode::Right) {
            *direction = SnakeDirection::Right;
        }
        if key.just_pressed(KeyCode::Up) {
            *direction = SnakeDirection::Up;
        }
        if key.just_pressed(KeyCode::Down) {
            *direction = SnakeDirection::Down;
        }
    }
}

fn collide_system(
    mut commands: Commands,
    mut snake_query: Query<&Transform, With<SnakeHead>>,
    mut block_query: Query<(&Transform, Entity), With<Block>>,
    mut last_entity: ResMut<LastEntity>,
    mut block_count: ResMut<BlockCount>,
    mut last_entity_query: Query<&Transform>,
    mut game_step: ResMut<GameStep>
) {
    let snake_head = snake_query.get_single();
    let block = block_query.get_single();

    match snake_head {
        Ok(snake) => {
            match block {
                Ok(block_tf) => {
                    let collision = collide(snake.translation,
                        Vec2::new(20., 20.),
                        block_tf.0.translation,
                        Vec2::new(20., 20.)
                    );

                    match collision {
                        Some(_) => {
                            let entity = last_entity_query.get_mut(last_entity.block).unwrap();
                            let new_snake = commands.spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BISQUE,
                                    custom_size: Some(Vec2::new(20., 20.)),
                                    ..Default::default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(entity.translation.x, entity.translation.y + 20., 1.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(Snake)
                            // .insert(PreviousEntity(last_entity.block))
                            .id();

                            block_count.0 = 0;
                            last_entity.block = new_snake; 
                            
                            commands.entity(block_tf.1).despawn();

                            game_step.0 -= 1.;
                        },
                        None => {},
                    }
                },
                Err(_) => {},
            }
        },
        Err(_) => {},
    }

}