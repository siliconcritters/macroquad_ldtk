# macroquad_ldtk
A Rust library to load LDtk level files into macroquad.

## Features
* Tile rendering
* Collision `Rect` generation from IntGrids
* Levels stored through their world coordinates
* Access to the unprocessed JSON data straight from `serde`

## To Be Implemented
* External level files

## How to Use
See [the basic example](https://github.com/siliconcritters/macroquad_ldtk/blob/main/examples/basic.rs) for a simple example of use.

Run the example with:
```
cargo run --example basic
```

## License
This library is dual-licensed under the Apache and MIT licenses.

[assets/kenney_platformer.png](https://github.com/siliconcritters/macroquad_ldtk/blob/main/assets/kenney_platformer.png) is from [the Pixel Platformer asset pack](https://kenney.nl/assets/pixel-platformer) by Kenney, licensed under CC0.
