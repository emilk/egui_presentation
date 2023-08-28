# egui presentation

A presentation about [egui](https://github.com/emilk/egui), implemented in egui.

Work-in-progress

## TODO
* [ ] Live-loading of `slides.md`
* [ ] Fix the images for the Wasm build
* [x] Make the presentation available on <https://emilk.github.io/egui_presentation/>.

### Running it

`cargo run --release`

### Running the web version
* `rustup target add wasm32-unknown-unknown`
* `cargo install --locked trunk`
* `trunk serve`
* open `http://127.0.0.1:8080/index.html#dev`

### Web Deploy
Should deploy automatically by the CI action.
