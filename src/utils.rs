use bevy::math::Vec3;

pub fn world_pos_to_board_id(world_pos: Vec3) -> u32 {
    ((world_pos.z + 3.5) * 8.0 + world_pos.x + 3.5) as u32
}

pub fn board_id_to_world_pos(board_id: u32) -> Vec3 {
    Vec3::new(
        (board_id % 8) as f32 - 3.5,
        0.1,
        (board_id / 8) as f32 - 3.5,
    )
}
