# texture-packer
A custom texture packer I'm building for my game project written in Rust.

## Planned features
- [x] Importing [Hiero](https://libgdx.com/wiki/tools/hiero) `.fnt` bitmap fonts
- [ ] Importing [Tiled](https://www.mapeditor.org/) `.tsj` tilesets
- [x] Ability to add custom meta data to sprites
- [ ] Ability to apply image effects to sprites
- [x] ~~Creating texture atlases using [`rectangle-pack`](https://crates.io/crates/rectangle-pack) (it seems to be not very optimal for this use case)~~
- [x] Creating texture atlases using custom packing algo
- [x] Exporting everything in a JSON format
- [x] Exporting everything in an [RMP](https://github.com/3Hren/msgpack-rust) format

