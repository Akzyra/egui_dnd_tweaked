use std::hash::{Hash, Hasher};

use eframe::egui::{Color32, Context};
use eframe::emath::lerp;
use eframe::{egui, App, Frame};
use egui::{Rounding, Ui, Vec2};
use egui_extras::{Size, StripBuilder};

use egui_dnd::utils::shift_vec;
use egui_dnd::DragDropUi;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Clone)]
struct Color {
    color: Color32,
    name: String,
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

struct DnDApp {
    dnd: DragDropUi,
    dnd2: DragDropUi,
    items: Vec<Color>,
    items2: Vec<Color>,
    preview: Option<Vec<Color>>,
}

// ff36ab abff36 36abff

// 9742ff 42ff97 ff9742

impl Default for DnDApp {
    fn default() -> Self {
        DnDApp {
            dnd: DragDropUi::default(),
            dnd2: DragDropUi::default(),
            items: vec![
                Color {
                    name: "Panic Purple".to_string(),
                    color: egui::hex_color!("642CA9"),
                },
                Color {
                    name: "Generic Green".to_string(),
                    color: egui::hex_color!("2A9D8F"),
                },
                Color {
                    name: "Ownership Orange".to_string(),
                    color: egui::hex_color!("E9C46A"),
                },
            ],
            items2: vec![
                Color {
                    name: "Red".to_string(),
                    color: egui::hex_color!("c91b08"),
                },
                Color {
                    name: "Lime".to_string(),
                    color: egui::hex_color!("27D507"),
                },
                Color {
                    name: "Blue".to_string(),
                    color: egui::hex_color!("352a85"),
                },
            ],
            preview: None,
        }
    }
}

impl DnDApp {
    fn dnd_ui_offset_auto(&mut self, ui: &mut Ui) {
        let response = self
            .dnd
            .ui::<Color>(ui, self.items.iter_mut(), |item, ui, handle| {
                ui.horizontal(|ui| {
                    let (_id, rect) = ui.allocate_space(Vec2::new(32.0, 32.0));
                    ui.painter()
                        .rect_filled(rect, Rounding::same(1.0), item.color);
                    ui.heading(&item.name);

                    // offset None -> calculate offset from parent
                    handle.ui_offset(ui, item, None, |ui| {
                        ui.heading("DRAG");
                    });
                });
            });
        if let Some(response) = response.completed {
            shift_vec(response.from, response.to, &mut self.items);
        }
        if let Some(response) = response.current_drag {
            self.preview = Some(self.items.clone());
            shift_vec(response.from, response.to, self.preview.as_mut().unwrap());
        }
    }

    fn dnd_ui_offset(&mut self, ui: &mut Ui) {
        let frame_width = ui.available_width();
        let response = self
            .dnd2
            .ui::<Color>(ui, self.items2.iter_mut(), |item, ui, handle| {
                // === NOT AN egui_dnd ISSUE, just a hack to make StripBuilder look nice ===
                // fix StripBuilder taking all vertical space for first drag item
                ui.set_height(32.0);
                // fix StripBuilder taking window width when dragging
                ui.set_width(frame_width);

                let strip_pos = ui.next_widget_position();
                StripBuilder::new(ui)
                    .size(Size::initial(32.0))
                    .size(Size::remainder())
                    .size(Size::exact(64.0))
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            let (_id, rect) = ui.allocate_space(Vec2::new(32.0, 32.0));
                            ui.painter()
                                .rect_filled(rect, Rounding::same(1.0), item.color);
                        });

                        strip.cell(|ui| {
                            ui.heading(&item.name);
                        });

                        strip.cell(|ui| {
                            // manually calculate offset
                            // auto does not work, would be relative to strip cell
                            let offset = ui.next_widget_position() - strip_pos;
                            handle.ui_offset(ui, item, Some(offset), |ui| {
                                ui.heading("DRAG");
                            });
                        });
                    });
            });
        if let Some(response) = response.completed {
            shift_vec(response.from, response.to, &mut self.items2);
        }
    }
}

impl App for DnDApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            vertex_gradient(
                ui,
                Default::default(),
                &Gradient(
                    self.preview
                        .as_ref()
                        .unwrap_or(self.items.as_ref())
                        .iter()
                        .map(|c| c.color)
                        .collect(),
                ),
            );

            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::exact(360.0))
                .size(Size::remainder())
                .horizontal(|mut strip| {
                    strip.empty();

                    strip.strip(|builder| {
                        builder
                            .size(Size::remainder())
                            .size(Size::exact(500.0))
                            .size(Size::remainder())
                            .vertical(|mut strip| {
                                strip.empty();

                                strip.cell(|ui| {
                                    ui.painter().rect_filled(
                                        ui.available_rect_before_wrap(),
                                        Rounding::same(4.0),
                                        ui.style().visuals.panel_fill,
                                    );

                                    egui::Frame::none().outer_margin(20.0).show(ui, |ui| {
                                        ui.heading("Auto Offset");
                                        ui.label("Fix cursor position automatically if handle is not at 0,0.");
                                        ui.small("Only works if handle is not in nested frames/scopes/layouts.");
                                        self.dnd_ui_offset_auto(ui);

                                        ui.heading("Manual Offset");
                                        ui.label("Strip cells are nested layout, so offset needs to be calculated manually.");
                                        ui.add_space(5.0);

                                        self.dnd_ui_offset(ui);

                                        ui.add_space(15.0);
                                        ui.separator();
                                        ui.add_space(15.0);

                                        ui.label("This is a demo for egui_dnd, a drag and drop sorting library for egui.");

                                        ui.hyperlink_to("View on GitHub", "https://github.com/lucasmerlin/egui_dnd");
                                        ui.hyperlink_to("View on Crates.io", "https://crates.io/crates/egui_dnd")
                                    });
                                });
                                strip.empty();
                            });
                    });

                    strip.empty();
                });
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "DnD Example App 2",
        options,
        Box::new(|_a| Box::new(DnDApp::default())),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "canvas",
            web_options,
            Box::new(|_a| Box::new(DnDApp::default())),
        )
        .await
        .expect("failed to start eframe");
    });
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Gradient(pub Vec<Color32>);

// taken from the egui demo
fn vertex_gradient(ui: &mut Ui, bg_fill: Color32, gradient: &Gradient) {
    use egui::epaint::*;

    let rect = ui.max_rect();

    if bg_fill != Default::default() {
        let mut mesh = Mesh::default();
        mesh.add_colored_rect(rect, bg_fill);
        ui.painter().add(Shape::mesh(mesh));
    }
    {
        let n = gradient.0.len();
        assert!(n >= 2);
        let mut mesh = Mesh::default();
        for (i, &color) in gradient.0.iter().enumerate() {
            let t = i as f32 / (n as f32 - 1.0);
            let y = lerp(rect.y_range(), t);
            mesh.colored_vertex(pos2(rect.left(), y), color);
            mesh.colored_vertex(pos2(rect.right(), y), color);
            if i < n - 1 {
                let i = i as u32;
                mesh.add_triangle(2 * i, 2 * i + 1, 2 * i + 2);
                mesh.add_triangle(2 * i + 1, 2 * i + 2, 2 * i + 3);
            }
        }
        ui.painter().add(Shape::mesh(mesh));
    };
}
