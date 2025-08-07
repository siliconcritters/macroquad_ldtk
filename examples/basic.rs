use macroquad::{prelude::*, window::Conf};
use macroquad_ldtk::prelude::*;

const VIRTUAL_WIDTH: i32 = 270; // These are the dimensions of a single level.
const VIRTUAL_HEIGHT: i32 = 252; // The level is rendered onto a canvas of this size, then upscaled.
const SCALE: i32 = 3;

#[macroquad::main(window_conf)]
async fn main() {
    let main_tileset = load_texture("assets/kenney_platformer.png").await.unwrap();
    main_tileset.set_filter(FilterMode::Nearest);

    let tilesets = [(main_tileset, "kenney_platformer.png")]; // This array of tuples is passed to the loading and drawing functions.

    let res = load_project("assets/basic.ldtk", &tilesets).unwrap();

    let level = (0, 0); // The level is stored by its coordinates (except in Horizontal and Vertical layouts; see docs)

    // Load all coins into the level
    let coins: Vec<Coin> = res
        .get_entities(level)
        .iter()
        .map(|me| Coin {
            position: me.px_coords,
        })
        .collect();

    // Create render targets to upscale the level
    let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut render_target_cam = Camera2D::from_display_rect(Rect::new(
        0.0,
        0.0,
        VIRTUAL_WIDTH as f32,
        VIRTUAL_HEIGHT as f32,
    ));
    render_target_cam.render_target = Some(render_target.clone());

    loop {
        set_camera(&render_target_cam);
        clear_background(BLUE);
        res.draw_level(level, &tilesets, Vec2::new(0.0, 0.0), None);

        // Draw coins
        for c in &coins {
            draw_texture_ex(
                &tilesets[0].0,
                c.position[0] as f32,
                c.position[1] as f32,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect {
                        x: 11.0 * 18.0,
                        y: 7.0 * 18.0,
                        w: 18.0,
                        h: 18.0,
                    }),
                    ..Default::default()
                },
            );
        }

        set_default_camera();
        draw_texture_ex(
            &render_target.texture,
            (screen_width() - (VIRTUAL_WIDTH * SCALE) as f32) * 0.5,
            (screen_height() - (VIRTUAL_HEIGHT * SCALE) as f32) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    (VIRTUAL_WIDTH * SCALE) as f32,
                    (VIRTUAL_HEIGHT * SCALE) as f32,
                )),
                flip_y: true,
                ..Default::default()
            },
        );

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Basic LDtk Example".to_string(),
        window_width: VIRTUAL_WIDTH * SCALE,
        window_height: VIRTUAL_HEIGHT * SCALE,
        window_resizable: false,
        ..Default::default()
    }
}

struct Coin {
    position: [i64; 2],
}
