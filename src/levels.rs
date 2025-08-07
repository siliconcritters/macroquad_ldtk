//! Module to handle interactions with the levels

use macroquad::prelude::*;

use crate::types::{LdtkEntityInstance, LdtkLayerType, LdtkLevel, LdtkResources};

impl LdtkResources {
    /// Draws the specified level. The texture array passed in should be the same as when the project was initially loaded.
    /// The `source` rect is in grid coordinates, while the `position` vector is in pixel coordinates.
    ///
    /// `textures` *must* be the same array as was passed in when the project was loaded.
    pub fn draw_level(
        &self,
        level_coord: (i64, i64),
        textures: &[(Texture2D, &str)],
        position: Vec2,
        source: Option<Rect>,
    ) {
        let lvl = &self
            .levels
            .get(&level_coord)
            .expect(format!("No level at coordinate {:?}", level_coord).as_str()); // I feel a panic is good enough here.
        let tilesets = &self.tilesets;

        for layer in &lvl.layers {
            let layerdef = &self.layer_defs.get(&layer.layerdef_id).unwrap();

            if layerdef.layer_type == LdtkLayerType::Entities {
                continue; // Skip non displayable layers
            }

            if layer.tileset_id.is_none() {
                continue; // This layer has nothing to render
            }
            let tileset_id = layer.tileset_id.as_ref().unwrap();

            let tileset = tilesets.get(tileset_id).unwrap();
            let tex = &textures[tileset.texture_index as usize].0;

            for t in &layer.tiles {
                if let Some(s) = source {
                    let grid_x = t.px_coords[0] as f32 / tileset.tile_grid_size as f32;
                    let grid_y = t.px_coords[1] as f32 / tileset.tile_grid_size as f32;

                    // Don't render outside of the specified source rectangle
                    if grid_x < s.x || grid_x >= s.x + s.w || grid_y < s.y || grid_y >= s.y + s.h {
                        continue;
                    }
                }
                draw_texture_ex(
                    tex,
                    t.px_coords[0] as f32 + position.x,
                    t.px_coords[1] as f32 + position.y,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect {
                            x: t.src_coords[0] as f32,
                            y: t.src_coords[1] as f32,
                            w: tileset.tile_grid_size as f32,
                            h: tileset.tile_grid_size as f32,
                        }),
                        ..Default::default()
                    },
                );
            }
        }
    }

    /// Gets all entities in a specified level. Useful for spawning entities on load.
    pub fn get_entities(&self, level_coord: (i64, i64)) -> Vec<&LdtkEntityInstance> {
        let mut entities = Vec::new();

        let level = &self
            .levels
            .get(&level_coord)
            .expect(format!("No level at coordinate {:?}", level_coord).as_str());
        for l in &level.layers {
            for e in &l.entities {
                entities.push(e);
            }
        }

        entities
    }
}

impl LdtkLevel {
    /// Generates rectangles that can easily be iterated over to check collision.
    pub fn generate_collision_rects(&self, layer_idx: usize, target_value: i64) -> Vec<Rect> {
        let layer = &self.layers[layer_idx];
        let mut rects = Vec::new();

        for (i, val) in layer.int_grid_values.iter().enumerate() {
            if val.to_owned() != target_value {
                continue;
            }
            let r = Rect::new(
                ((i as i64 % layer.grid_width) * layer.grid_size) as f32,
                ((i as i64 / layer.grid_width) * layer.grid_size) as f32,
                layer.grid_size as f32,
                layer.grid_size as f32,
            );

            rects.push(r);
        }

        rects
    }
}

mod test {
    use macroquad::math::Rect;

    use crate::types::{LdtkLayerInstance, LdtkLevel};

    #[test]
    fn rect_generation() {
        let layer = LdtkLayerInstance {
            grid_width: 4,
            grid_height: 3,
            grid_size: 16,
            layerdef_id: "No".to_owned(),
            tileset_id: None,
            tiles: Vec::new(),
            entities: Vec::new(),
            int_grid_values: vec![2, 0, 3, 0, 1, 2, 1, 0, 1, 1, 0, 1],
        };
        let level = LdtkLevel {
            width: 4,
            height: 3,
            layers: vec![layer],
        };

        let expected = vec![
            Rect::new(0.0 * 16.0, 1.0 * 16.0, 16.0, 16.0),
            Rect::new(2.0 * 16.0, 1.0 * 16.0, 16.0, 16.0),
            Rect::new(0.0 * 16.0, 2.0 * 16.0, 16.0, 16.0),
            Rect::new(1.0 * 16.0, 2.0 * 16.0, 16.0, 16.0),
            Rect::new(3.0 * 16.0, 2.0 * 16.0, 16.0, 16.0),
        ];

        assert_eq!(level.generate_collision_rects(0, 1), expected);
    }
}
