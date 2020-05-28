use amethyst::{
    prelude::*,
    ecs::{Entity, Component, DenseVecStorage},
    renderer::{Texture, SpriteSheet, SpriteRender, ImageFormat, SpriteSheetFormat},
    assets::{AssetStorage, Handle, Loader}
};


pub fn load_spritesheet_handle(world: &mut World, name: String) -> Handle<SpriteSheet> {
    let base_str = "texture/";
    let pic_path = base_str.to_string() + name.as_ref() + ".png".to_string().as_ref();
    let ron_path = base_str.to_string() + name.as_ref() + ".ron";

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            pic_path,
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path, // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}