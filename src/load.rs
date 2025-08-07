//! Functions to load data from an LDtk project.

use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use convert::{convert_layer_def, convert_level};
use macroquad::texture::Texture2D;

use crate::{
    error::Error,
    parser::*,
    types::{LdtkLayerDef, LdtkLevel, LdtkResources, LdtkTileset},
};

/// Loads an LDtk project from a JSON file, with unimportant data stripped out.
/// Returns a struct containing the LDtk project resources.
///
/// Accepts an array of tuples that contain your texture and their path (relative to the project file).
pub fn load_project(path: &str, textures: &[(Texture2D, &str)]) -> Result<LdtkResources, Error> {
    let json = load_project_raw(path)?;

    let mut path_base = PathBuf::from(path);
    path_base.pop(); // Remove the filename so just the folder containing the project remains

    // Load tilesets
    let mut tilesets: HashMap<String, LdtkTileset> = HashMap::new();
    for json_t in &json.defs.tilesets {
        let tex_i = textures
            .iter()
            .position(|(_, name)| *name == json_t.rel_path.as_ref().unwrap().as_str())
            .unwrap();

        let tileset = LdtkTileset {
            grid_height: json_t.c_hei,
            grid_width: json_t.c_wid,
            padding: json_t.padding,
            spacing: json_t.spacing,
            tile_grid_size: json_t.tile_grid_size,
            identifier: json_t.identifier.clone(),
            uid: json_t.uid,
            texture_index: tex_i as u32,
        };

        tilesets.insert(textures[tex_i].1.to_owned(), tileset);
    }

    // Load layer definitions
    let mut layer_defs: HashMap<String, LdtkLayerDef> = HashMap::new();
    for json_l in &json.defs.layers {
        let layerdef = convert_layer_def(json_l)?;

        layer_defs.insert(json_l.identifier.clone(), layerdef);
    }

    // Load levels
    let mut levels: HashMap<(i64, i64), LdtkLevel> = HashMap::new();
    if json.world_layout.is_none() {
        return Err(Error::NullWorldType);
    }

    match json.world_layout.unwrap() {
        WorldLayout::Free | WorldLayout::GridVania => {
            for level in &json.levels {
                levels.insert((level.world_x, level.world_y), convert_level(level));
            }
        }
        WorldLayout::LinearHorizontal => {
            for (i, level) in json.levels.iter().enumerate() {
                levels.insert((i as i64, 0), convert_level(level));
            }
        }
        WorldLayout::LinearVertical => {
            for (i, level) in json.levels.iter().enumerate() {
                levels.insert((0, i as i64), convert_level(level));
            }
        }
    }

    // Compile loaded assets into the final structure
    let resources = LdtkResources {
        levels,
        tilesets,
        layer_defs,
    };

    Ok(resources)
}

/// Loads the project and gives the raw `serde` output.
pub fn load_project_raw(path: &str) -> Result<LdtkJson, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json: LdtkJson = serde_json::from_reader(reader)?;

    Ok(json)
}

/// Internal type conversions mod
mod convert {
    use crate::error::Error;
    use crate::parser::{EntityInstance, LayerDefinition, Level, TileInstance};
    use crate::types::{
        LdtkEntityInstance, LdtkLayerDef, LdtkLayerInstance, LdtkLayerType, LdtkLevel,
        LdtkTileInstance,
    };

    /// Converts a TileInstance into an LdtkTileInstance.
    pub fn convert_tile_instance(input: &TileInstance) -> LdtkTileInstance {
        LdtkTileInstance {
            alpha: input.a,
            px_coords: [input.px[0], input.px[1]],
            src_coords: [input.src[0], input.src[1]],
            tile_id: input.t,
        }
    }

    /// Converts a String into the correct LdtkLayerType.
    pub fn convert_layer_type(input: &String) -> Result<LdtkLayerType, Error> {
        match input.as_str() {
            "IntGrid" => Ok(LdtkLayerType::IntGrid),
            "Tiles" => Ok(LdtkLayerType::Tiles),
            "AutoLayer" => Ok(LdtkLayerType::AutoLayer),
            "Entities" => Ok(LdtkLayerType::Entities),
            _ => Err(Error::LayerTypeNotFound {
                layer_type: input.clone(),
            }),
        }
    }

    /// Converts an EntityInstance into an LdtkEntityInstance.
    pub fn convert_entity_instance(input: &EntityInstance) -> LdtkEntityInstance {
        let world_coords = if let Some(wx) = input.world_x {
            if let Some(wy) = input.world_y {
                Some([wx, wy])
            } else {
                None
            }
        } else {
            None
        };

        LdtkEntityInstance {
            grid_coords: [input.grid[0], input.grid[1]],
            pivot: [input.pivot[0], input.pivot[1]],
            tags: input.tags.clone(),
            px_coords: [input.px[0], input.px[1]],
            world_coords,
            identifier: input.identifier.clone(),
            iid: input.iid.clone(),
            height: input.height,
            width: input.width,
        }
    }

    /// Converts LayerDefinition to an LdtkLayerDef.
    pub fn convert_layer_def(input: &LayerDefinition) -> Result<LdtkLayerDef, Error> {
        let layer_type = convert_layer_type(&input.layer_definition_type)?;

        let layerdef = LdtkLayerDef {
            layer_type,
            identifier: input.identifier.clone(),
            opacity: input.display_opacity,
            grid_size: input.grid_size,
            uid: input.uid,
        };

        Ok(layerdef)
    }

    /// Converts a Level into an LdtkLevel.
    pub fn convert_level(input: &Level) -> LdtkLevel {
        let mut layer_insts: Vec<LdtkLayerInstance> = Vec::new();

        for l in input.layer_instances.as_ref().unwrap() {
            let source_tiles = if l.grid_tiles.len() > 0 {
                &l.grid_tiles
            } else {
                &l.auto_layer_tiles
            };
            let tiles: Vec<LdtkTileInstance> = source_tiles
                .iter()
                .map(|me| convert_tile_instance(me))
                .collect();

            let entities: Vec<LdtkEntityInstance> = l
                .entity_instances
                .iter()
                .map(|me| convert_entity_instance(me))
                .collect();

            let l_converted = LdtkLayerInstance {
                grid_height: l.c_hei,
                grid_width: l.c_wid,
                grid_size: l.grid_size,
                layerdef_id: l.identifier.clone(),
                tileset_id: l.tileset_rel_path.clone(),
                entities,
                tiles,
                int_grid_values: l.int_grid_csv.clone(),
            };
            layer_insts.push(l_converted);
        }

        LdtkLevel {
            layers: layer_insts,
            width: input.px_wid,
            height: input.px_hei,
        }
    }
}
