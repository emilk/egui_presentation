use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

enum SlideMarker {
    CounterExample,

    UiExample,

    ToggleWidget,

    UnknownMarker(String),
}

struct Slide {
    title: String,

    markdown: String,

    /// Special slide id
    markers: Vec<SlideMarker>,
}

impl Slide {
    pub fn new(text: &str) -> Self {
        use itertools::Itertools as _;

        let mut markers = vec![];

        let markdown = text
            .trim()
            .lines()
            .filter(|line| {
                if let Some(marker_str) = line.strip_prefix("!!!") {
                    let marker = match marker_str.trim() {
                        "counter_example" => SlideMarker::CounterExample,
                        "ui_example" => SlideMarker::UiExample,
                        "toggle_widget" => SlideMarker::ToggleWidget,
                        _ => {
                            log::warn!("Unknown slide marker: {marker_str:?}");
                            SlideMarker::UnknownMarker(marker_str.to_owned())
                        }
                    };
                    markers.push(marker);
                    false
                } else {
                    true
                }
            })
            .join("\n");

        let title = if let Some(trailing) = markdown.strip_prefix("# ") {
            let newline = trailing.find('\n').unwrap_or(trailing.len());
            trailing[..newline].trim()
        } else {
            log::warn!("Unknown title for slide: {:?}", markdown);
            "???"
        };

        Self {
            title: title.to_owned(),
            markdown: markdown.to_owned(),
            markers,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Presentation {
    #[serde(skip)]
    cm_cache: CommonMarkCache,

    #[serde(skip)]
    slides: Vec<Slide>,

    slide_nr: usize,

    // For examples:
    counter: i32,
    some_bool: bool,
}

impl eframe::App for Presentation {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if !frame.is_web() {
            egui::gui_zoom::zoom_with_keyboard_shortcuts(ctx, frame.info().native_pixels_per_point);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }

    fn persist_native_window(&self) -> bool {
        false
    }
}

impl Default for Presentation {
    fn default() -> Self {
        let slides = include_str!("../slides.md");
        let slides = slides.split("\n-------------------------------------------------------------------------------\n").map(Slide::new).collect::<Vec<_>>();

        Self {
            cm_cache: Default::default(),
            slides,
            slide_nr: 0,
            counter: 0,
            some_bool: false,
        }
    }
}

impl Presentation {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx
            .set_pixels_per_point(cc.integration_info.native_pixels_per_point.unwrap_or(1.0) * 2.0);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            cm_cache,
            slides,
            slide_nr,

            counter,
            some_bool,
        } = self;

        ui.input(|i| {
            if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::Space) {
                *slide_nr = (*slide_nr + 1) % slides.len();
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                *slide_nr = (*slide_nr + slides.len() - 1) % slides.len();
            }
        });

        let mut preview_slide_nr = *slide_nr;

        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 20.0),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                let text = format!("{}/{}", *slide_nr + 1, slides.len());
                let text = egui::RichText::new(text).weak();
                ui.menu_button(text, |ui| {
                    for (i, slide) in slides.iter().enumerate() {
                        let response = ui.selectable_value(slide_nr, i, &slide.title);
                        if response.hovered() {
                            preview_slide_nr = i;
                        }
                        if response.clicked() {
                            ui.close_menu();
                        }
                    }
                });
            },
        );

        let slide = &slides[preview_slide_nr];
        let Slide {
            title: _,
            markdown,
            markers,
        } = slide;

        CommonMarkViewer::new("viewer")
            .max_image_width(Some(ui.available_width().floor() as _))
            .show(ui, cm_cache, markdown);

        for marker in markers {
            match marker {
                SlideMarker::CounterExample => {
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("-").clicked() {
                            *counter -= 1;
                        }

                        ui.label(counter.to_string());

                        if ui.button("+").clicked() {
                            *counter += 1;
                        }
                    });
                }

                SlideMarker::UiExample => {
                    ui.separator();
                    ui.label("Some text");
                    ui.horizontal(|ui| {
                        ui.label("More");
                        ui.label("text");
                    });
                }

                SlideMarker::ToggleWidget => {
                    ui.separator();
                    toggle_widget(ui, some_bool);
                }

                SlideMarker::UnknownMarker(marker) => {
                    ui.label(
                        egui::RichText::new(format!("⚠ Unknown slider marker: {marker:?} ⚠"))
                            .color(ui.visuals().warn_fg_color),
                    );
                }
            }
        }
    }
}

fn toggle_widget(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}
