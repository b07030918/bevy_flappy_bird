use super::UI_Z;
use crate::{cleanup, has_user_input, GameState};
use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            //进入菜单界面，产生MenuEntity
            .add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
            //离开菜单 销毁 MenuEntity
            .add_system(cleanup::<MenuEntity>.in_schedule(OnExit(GameState::Menu)))
            .add_system(
                start_playing
                    .in_set(OnUpdate(GameState::Menu))
                    .run_if(has_user_input),
            );
    }
}

#[derive(Component)]
struct MenuEntity;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/start.png"),
            transform: Transform::from_xyz(0.0, 80.0, UI_Z),
            ..Default::default()
        },
        MenuEntity,
    ));
}

fn start_playing(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}
