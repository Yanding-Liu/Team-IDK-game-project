use bevy::{
	prelude::*,
};

#[derive(Component)]
pub struct CombatStats {
    pub health: isize,
    pub max_health: isize,
	pub tp: isize,
	pub max_tp: isize,
	pub token: isize,
	pub max_token: isize,
	pub guard: bool,
	pub double: bool,
	pub block: bool,
	pub tp_cost_mult: isize,
}

#[derive(Component)]
pub struct CombatLog {
	pub player_damage: isize,
	pub enemy_damage: isize,
}