use rogalik::prelude::*;

pub fn load_assets(context: &mut Context) {
    load_textures(context);
}

fn load_textures(context: &mut Context) {
    context.graphics.load_material(
        "sprites",
        MaterialParams {
            atlas: Some(AtlasParams {
                cols: 49,
                rows: 22,
                padding: Some((1., 1.)),
            }),
            diffuse_path: "sprites/sprites.png",
            ..Default::default()
        },
    );
    context.graphics.load_material(
        "icons_small",
        MaterialParams {
            atlas: Some(AtlasParams {
                cols: 8,
                rows: 8,
                padding: None,
            }),
            diffuse_path: "ui/icons_small.png",
            ..Default::default()
        },
    );
    context
        .graphics
        .load_font("default", "ui/font.png", 16, 16, Some((11., 8.)));

    context
        .graphics
        .load_font("digits", "ui/digits.png", 16, 16, Some((4., 2.)));
}
