use bevy::{ecs::{event::{Event, EventReader, EventWriter}, system::{ParamSet, Query}, query::With}, transform::components::Transform};

use crate::{Player, Wall, Direction, player, SCREEN_GAME_Y};

#[derive(Event)]
pub struct TickEvent;

pub fn tick_event_listener(
    mut events: ParamSet<(EventReader<TickEvent>, EventWriter<TickEvent>)>
) {
    if events.p0().read().last().is_none() { return; }
    println!("tick !");
}

#[derive(Event)]
pub struct EndTickEvent;

pub fn end_tick_event_listener(
    mut events: ParamSet<(EventReader<EndTickEvent>, EventWriter<TickEvent>)>,
    mut player: Query<&mut Player>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    wall_query: Query<&Wall>,
) {
    if events.p0().read().last().is_none() { return; }
    println!("end tick !");

    // Gravity
    {
        let mut player = player.single_mut();
        if player.game_x.is_none() || player.game_y.is_none() || player.is_animating { return; }
        if wall_query.is_empty() { return; }

        let player_game_x = player.game_x.unwrap();
        let player_game_y = player.game_y.unwrap();

        
        let mut is_wall_under_player = false;
        for wall in wall_query.iter() {
            if player_game_x == wall.game_x && player_game_y-1 == wall.game_y {
                is_wall_under_player = true;
            }
        }

        if is_wall_under_player == false {
            player.move_with_direction(Direction::Bottom);
            events.p1().send(TickEvent);
        }
    }

    // Quand le joueur tombe tout en bas le mettre en haut
    {
        let mut player = player.single_mut();
        let mut player_transform = player_transform.single_mut();
        if player.game_x.is_none() || player.game_y.is_none() || player.is_animating { return; }
        if wall_query.is_empty() { return; }

        let player_game_x = player.game_x.unwrap();
        let player_game_y = player.game_y.unwrap();

        if player_game_y < -1 {
            player_transform.translation = player.move_without_animation(player_game_x, SCREEN_GAME_Y-1).extend(0.);
            events.p1().send(TickEvent);
        }
    }
}