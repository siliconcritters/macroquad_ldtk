//! Types used to represent LDtk resources

use std::collections::HashMap;

/// Struct that holds all necessary resources from an LDtk project.
/// Does not hold all data from the project, only what is needed for its own methods.
pub struct LdtkResources {
    /// Levels are in a HashMap of a tuple (x,y), representing the level location. In horizontal levels y is always 0 and in vertical levels x is always 0.
    pub levels: HashMap<(i64, i64), LdtkLevel>,

    /// Map of all tilesets
    pub tilesets: HashMap<String, LdtkTileset>,

    pub layer_defs: HashMap<String, LdtkLayerDef>,
}

/// Contains all data for a specific level
pub struct LdtkLevel {
    pub width: i64,
    pub height: i64,
    pub layers: Vec<LdtkLayerInstance>,
}

/// Extra layer data, such as opacity
pub struct LdtkLayerDef {
    pub layer_type: LdtkLayerType,
    pub identifier: String,
    pub opacity: f64,
    pub grid_size: i64,

    pub uid: i64,
}

/// Instances of a layer that hold actual terrain data
pub struct LdtkLayerInstance {
    pub grid_height: i64,
    pub grid_width: i64,
    pub grid_size: i64,

    /// Identifier used to index through the hashmap of layer definitions
    pub layerdef_id: String,

    /// Path of tileset, used to index into a hashmap
    pub tileset_id: Option<String>,

    /// `Vec` of all tiles, sorted in render order, not in position.
    pub tiles: Vec<LdtkTileInstance>,

    /// `Vec` of all entities.
    pub entities: Vec<LdtkEntityInstance>,

    /// IntGrid values for the layer, in CSV format.
    pub int_grid_values: Vec<i64>,
}

/// Entity instance as placed in LDtk.
/// Should be treated as a spawnpoint, not the actual entity.
pub struct LdtkEntityInstance {
    /// Grid-based coordinates.
    pub grid_coords: [i64; 2],

    /// Pivot coordinates of the entity.
    pub pivot: [f64; 2],

    /// List of tags from the entity definition.
    pub tags: Vec<String>,

    /// Current level coordinates in pixels.
    pub px_coords: [i64; 2],

    /// World coordinates in pixels. Only usable in Gridvania and Free world layouts.
    pub world_coords: Option<[i64; 2]>,

    /// Entity definition identifier.
    pub identifier: String,
    /// Unique instance identifier.
    pub iid: String,

    pub height: i64,
    pub width: i64,
}

/// Holds the data for a tileset.
pub struct LdtkTileset {
    /// Index of the texture in the passed-in array
    pub texture_index: u32,

    pub grid_height: i64,
    pub grid_width: i64,

    pub padding: i64,
    pub spacing: i64,
    pub tile_grid_size: i64,

    pub identifier: String,
    pub uid: i64,
}

/// Holds the data of an individual tile.
pub struct LdtkTileInstance {
    pub alpha: f64,

    /// Coordinates in the level, in pixels.
    pub px_coords: [i64; 2],
    /// Coordinates in the source image.
    pub src_coords: [i64; 2],

    pub tile_id: i64,
}

/// Layer types selectable in the LDtk editor
#[derive(Eq, PartialEq, Debug)]
pub enum LdtkLayerType {
    /// Grid of integer values
    IntGrid,
    /// Layer of entities. Does not display through `LdtkResources::draw_level`.
    Entities,
    /// Layer of manually placed tiles.
    Tiles,
    /// Layer of automatically placed tiles.
    AutoLayer,
}
