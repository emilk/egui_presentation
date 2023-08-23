# egui
[www.egui.rs](www.egui.rs)

An easy-to-use immediate mode GUI in Rust that runs on both web and native.

The most popular pure Rust GUI library on [crates.io](https://crates.io/search?q=gui&sort=downloads) (second only to GTK).

Limited accessibility support via [accesskit](https://crates.io/crates/accesskit).

-------------------------------------------------------------------------------

# Integrations
## Official
* `eframe`
* `egui_glow`
* `egui-wgpu`
* `egui-winit`

## 3rd party
![integrations](images/integrations.png)

-------------------------------------------------------------------------------

# History
Wrote the first piece of code in 2018, but started seriously during 2020 pandemic.

Heavily inspired by [Dear ImGui](https://github.com/ocornut/imgui), my favoruite GUI library for C++.

Not my first GUI, but my first immediate mode one and my first in Rust.

-------------------------------------------------------------------------------

# Crates
![crates](images/crates.png)

-------------------------------------------------------------------------------

# Immediate mode
20 year old GUI paradigm from game dev

``` rs
if ui.button("-").clicked() {
    counter -= 1;
}

ui.label(counter.to_string());

if ui.button("+").clicked() {
    counter += 1;
}
```

!!!counter_example

-------------------------------------------------------------------------------

# Context
```rs
let mut ctx = egui::Context::default();

// Game loop:
loop {
    // Gather keyboard/mouse events:
    let raw_input = window.collect_input();

    // Run egui:
    let output = ctx.run(raw_input, |ctx| {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.label("Hello world!");
            if ui.button("Click me").clicked() {
                // take some action here
            }
        });
    });

    // Set cursor icon, set clipboard, open url, …
    window.handle_platform_output(output.platform_output);

    let triangles = ctx.tessellate(output.shapes);
    window.paint(output.textures_delta, triangles);
}
```

-------------------------------------------------------------------------------

# Input
``` rs
pub struct RawInput {
    pub time: Option<f64>,
    pub events: Vec<Event>,
    pub modifiers: Modifies,
    // …
}

pub enum Event {
    Copy,
    Cut,
    Paste(String),
    Text(String),
    Key { … },
    PointerMoved(Pos2),
    PointerButton { … },
    …
}
```

-------------------------------------------------------------------------------

# Ui
A region of the screen with a layout where you can put widgets.

``` rs
ui.label("Some text");
ui.horizontal(|ui| {
    ui.label("More");
    ui.label("text");
});
```

!!!ui_example

-------------------------------------------------------------------------------

# Layout

|           |                                                  |
|-----------|--------------------------------------------------|
| Min Rect  |   Bounding rect of all widgets so far            |
| Max Rect  |   Try to keep within these bounds (wrap width)   |
| Direction |   Down, Up, Left-to-Right, Right-to-Left         |
| Cursor    |   Where to place the next widget                 |

-------------------------------------------------------------------------------

# Id

auto vs persist

-------------------------------------------------------------------------------

# Response
Returned from by widget. Has a `Rect`, a `Context` and interaction flags.

``` rs
if ui
    .button("Save")
    .on_hover_text("Click to save document")
    .clicked()
{
    save();
}

if ui.add(egui::Slider::new(&mut volume, 0.0..=100.0)).changed() {
    set_volume(volume);
}
```

-------------------------------------------------------------------------------

# Custom widget

``` rs
fn toggle_widget(ui: &mut Ui, on: &mut bool) -> Response {
    let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}
```

``` rs
toggle_widget(ui, &mut some_bool);
```

!!!toggle_widget

-------------------------------------------------------------------------------

# FullOutput

``` rs
pub struct FullOutput {
    /// Non-rendering related output.
    pub platform_output: PlatformOutput,

    /// If `Duration::is_zero()`, egui is requesting immediate repaint (i.e. on the next frame).
    pub repaint_after: std::time::Duration,

    /// Texture changes since last frame (including the font texture).
    pub textures_delta: epaint::textures::TexturesDelta,

    /// What to paint.
    pub shapes: Vec<epaint::ClippedShape>,
}

```

-------------------------------------------------------------------------------

# FullOutput

``` rs
pub struct PlatformOutput {
    /// Set the cursor to this icon.
    pub cursor_icon: CursorIcon,

    /// If set, open this url.
    pub open_url: Option<OpenUrl>,

    /// If set, put this text in the system clipboard. Ignore if empty.
    pub copied_text: String,

    // …
}

```

-------------------------------------------------------------------------------

# Painting
``` rs
pub enum Shape {
    Circle(CircleShape),
    Text(TextShape),
    …

    /// Backend-specific painting.
    Callback(PaintCallback),
}

pub struct CircleShape {
    pub center: Pos2,
    pub radius: f32,
    pub fill: Color32,
    pub stroke: Stroke,
}
```

`Shape` ➡ tesslator ➡ `Mesh`

Uses feathering for anti-aliasing

`ab_glyph` ➡ font image

-------------------------------------------------------------------------------

# eframe

-------------------------------------------------------------------------------

# Shortcomings
* [Immediate mode limitations](https://github.com/emilk/egui#why-immediate-mode)
* Styling
* Composition

-------------------------------------------------------------------------------

# Summary

-------------------------------------------------------------------------------

# Q&A
Thank you!

Questions?
