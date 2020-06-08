use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender, camera::Camera},
};
use crate::entity::tile::tile::Tile;

#[derive(SystemDesc)]
pub struct StaticTileSystem;

impl StaticTileSystem {
    pub fn new() -> Self {
        StaticTileSystem
    }

    fn draw(&mut self, tile: &Tile, sprite_render: &mut SpriteRender) {
        sprite_render.sprite_number = tile.attrs.to_sprite_index();
    }
}

impl<'s> System<'s> for StaticTileSystem {
    type SystemData = (WriteStorage<'s, Tile>);

    fn run(&mut self, data: Self::SystemData) {

    }
}