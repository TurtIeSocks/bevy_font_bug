# Bevy Font Rendering Issue

Minimal reproduction of a bug related to the `cosmic_text` crate that is causing inconsistent font rendering in Bevy. 

1. Clone repo
2. Run `cargo run` in the root directory

After 1 second, two texts will be rendered, one that is loading the `NotoSansJA.ttf` font and displays correctly, the second is attempting to render Japanese characters with the default Bevy font and fails to do so, as expected.

After 2 seconds, the same two texts will be rendered, except this time the second text will be incorrectly rendered with the `NotoSansJA.ttf` font.
