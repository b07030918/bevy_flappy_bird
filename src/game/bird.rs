use super::{BIRD_ANIMATION_SPEED, FALL_SPEED, FALL_VELOCITY_LIMIT, JUMP_AMOUNT, MOVE_SPEED};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Bird {
    //速率
    velocity: f32,
}

//向上的速度
pub(super) fn jump(mut bird: Query<&mut Bird>) {
    for mut bird in &mut bird {
        //向上
        bird.velocity = JUMP_AMOUNT;
    }
}

//自由下落的速度，最大FALL_VELOCITY_LIMIT（-0.5）
pub(super) fn fall(mut bird: Query<&mut Bird, With<Bird>>, time: Res<Time>) {
    for mut bird in &mut bird {
        bird.velocity -= FALL_SPEED * time.delta_seconds();
        bird.velocity = bird.velocity.max(FALL_VELOCITY_LIMIT);
    }
}

pub(super) fn move_bird(mut bird: Query<(&mut Transform, &Bird), With<Bird>>, time: Res<Time>) {
    for (mut transform, bird) in &mut bird {
        transform.translation.y += bird.velocity * MOVE_SPEED * time.delta_seconds();
    }
}

pub(super) fn animate_bird(mut bird: Query<&mut TextureAtlasSprite, With<Bird>>, time: Res<Time>) {
    for mut bird in &mut bird {
        bird.index = (time.elapsed_seconds() * BIRD_ANIMATION_SPEED) as usize % 4;
    }
}
