# egui presentation

A presentation about [egui](https://github.com/emilk/egui), implemented in egui.

You can view the presentation at <https://emilk.github.io/egui_presentation/>.

## TODO
* [ ] Live-reloading of `slides.md`

### Running it

`cargo run --release`

### Running the web version
* `rustup target add wasm32-unknown-unknown`
* `cargo install --locked trunk`
* `trunk serve`
* open `http://127.0.0.1:8080/index.html#dev`

### Web Deploy
Should deploy automatically by the CI action.
