use flecs_ecs::prelude::*;

#[derive(Debug, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub fn setup_core() -> World {
    let world = World::new();
    world
        .system::<(& mut Position, &Velocity)>()
        .each(|(pos, vel)| {
            pos.x += vel.x;
            pos.y += vel.y;
        });

    world
}
