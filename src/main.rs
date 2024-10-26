#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


pub mod base_configuration;
pub mod components;

use std::cell::RefCell;
use eframe::{egui, App, NativeOptions};
use egui::{Align, Align2, Context, Layout, RichText, ScrollArea, TopBottomPanel, CentralPanel, SidePanel, Ui, ImageButton, Image, TextBuffer, Id, LayerId, Order, TextStyle, vec2, TextureHandle, epaint::{Color32}, Widget};


use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::base_configuration::ComponentLevel;
use crate::components::draggable_item::DraggableItem;
use crate::components::main_flow::main_flow::MainFlow;

enum ComponentType {
    MainFlow(components::main_flow::main_flow::MainFlow),
    // Choice(Choice),
    // Add more widget types here
}

pub struct Panels {
    left_panel_open: bool,
    right_panel_open: bool,
    bottom_panel_open: bool,
    arrow_up_texture: Option<TextureHandle>,
    main_flow_draggable_item: Option<DraggableItem>,
    choice_draggable_item: Option<DraggableItem>,
    is_dragging: bool,
    show_dropped_box: bool,
    dragging_item: Option<DraggableItem>, // Track which item is being dragged
    pub accepted_items: VecDeque<DraggableItem>, // Queue to store accepted items
    widget_list: Vec<ComponentType>,
}


impl Panels {
    /// New method to create an instance of `Panels` and load textures
    pub fn new(ctx: &Context) -> Self {
        let arrow_up_bytes = include_bytes!("../assets/icons/arrow-up_8600241.png");
        let choice_bytes = include_bytes!("../assets/images/choiceSimplified_48.png");

        let arrow_up_texture = Self::load_texture(ctx, arrow_up_bytes, "arrow_up");
        let choice_draggable_item = DraggableItem::new(ctx, "Choice",  ComponentLevel::ParentOnly.as_str());
        let main_flow_draggable_item = DraggableItem::new(ctx, "MainFlow",  ComponentLevel::ParentOnly.as_str()) ;
        Panels {
            left_panel_open: true,
            right_panel_open: true,
            bottom_panel_open: false,
            arrow_up_texture,
            main_flow_draggable_item: Option::from(main_flow_draggable_item),
            choice_draggable_item: Option::from(choice_draggable_item),
            is_dragging: false,
            show_dropped_box: false,
            dragging_item: None,
            accepted_items: VecDeque::new(),
            widget_list: vec![],
        }
    }

    fn load_texture(ctx: &Context, image_bytes: &[u8], name: &str) -> Option<TextureHandle> {
        // Decode the image using the `image` crate
        if let Ok(image) = image::load_from_memory(image_bytes) {
            let size = [image.width() as _, image.height() as _];
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image.to_rgba8());
            Some(ctx.load_texture(name, color_image, Default::default()))
        } else {
            None
        }
    }


    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Main Application - Central Panel");
        });
        ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Drop Here");
            });


        });
    }
}

struct AppWrapper {
    app: Panels,
}



impl App for AppWrapper {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        CentralPanel::default().show(ctx, |ui| {
            // Drag-and-drop area
            let frame = egui::Frame {
                fill: Color32::from_rgba_premultiplied(0, 0, 0, 0),
                ..Default::default()
            };

            let (_response, dropped_payload) =
                ui.dnd_drop_zone::<DraggableItem, ()>(frame, |ui| {
                // Handle the drop zone but do not draw MainFlow components here
                // ui.layer_id().order = Order::Background;
                ui.label("Drop items here").layer_id = LayerId::new(Order::Foreground, Id::new("drop_label"));
                ui.allocate_space(ui.available_size());

                // For testing purposes, add a label

            });

            if let Some(dropped_item) = dropped_payload {
                println!("Item dropped: {:?}", dropped_item.name); // Debug print
                let item = dropped_item;
                if item.component_level == ComponentLevel::ParentOnly.as_str() {
                    match item.name.as_str() {
                        "MainFlow" => {
                            let main_flow = MainFlow::new("MainFlow Title".to_string());
                            self.app.widget_list.push(ComponentType::MainFlow(main_flow));
                            println!("MainFlow added to widget_list"); // Debug print
                        },
                        _ => {},
                    }
                }
            }


            // Render MainFlow components on a specific layer
            // ui.columns(self.app.widget_list.len(), |ui| {
                ui.with_layer_id( LayerId::new(Order::Foreground, Id::new("mainflow_layer") ), |ui| {
                    for widget in &mut self.app.widget_list {
                        match widget {
                            ComponentType::MainFlow(main_flow) => {
                                let title = main_flow.title.clone();
                                main_flow.show_in_panel(ui, vec2(150.0/*ui.available_width()*/, 100.0), &title);
                            },
                            // Handle other components
                        }
                    }
                });
            // });


            // For testing purposes, add a label
            ui.label("Central Panel is working!");
        });




        // Left Panel (collapsible with draggable items)
        {
            let left_panel_open = self.app.left_panel_open;

            if left_panel_open {
                SidePanel::left("left_panel")
                    .resizable(true)
                    .default_width(150.0)
                    .width_range(80.0..=200.0)
                    .show(ctx, |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui.button("\u{2B05}").clicked() {
                                self.app.left_panel_open = !self.app.left_panel_open;
                            }
                        });

                        // Add draggable items and other UI elements
                        ui.horizontal_wrapped(|ui| {
                            // Clone draggable items
                            let (main_flow_draggable, choice_draggable) = (
                                self.app.main_flow_draggable_item.clone(),
                                self.app.choice_draggable_item.clone(),
                            );

                            // Allow dragging `choice_draggable`
                            if let Some(choice_draggable) = choice_draggable {
                                if let Some(texture) = choice_draggable.texture.as_ref() {
                                    let texture_id = texture.id();
                                    let sized_texture = egui::load::SizedTexture::from((
                                        texture_id,
                                        egui::vec2(48.0, 48.0),
                                    ));
                                    let the_image = egui::Image::from(sized_texture);

                                    let response = ui
                                        .dnd_drag_source(
                                            egui::Id::new("choice_texture"),
                                            choice_draggable.clone(),
                                            move |ui| {
                                                ui.add(egui::ImageButton::new(the_image));
                                            },
                                        )
                                        .response;

                                    if response.drag_started() {
                                        println!("Dragging Choice started!");
                                    }
                                    if response.drag_stopped() {
                                        println!("Dragging Choice stopped!");
                                    }
                                }
                            }

                            // Allow dragging `main_flow_draggable` similarly
                            if let Some(main_flow_draggable) = main_flow_draggable {
                                if let Some(texture) = main_flow_draggable.texture.as_ref() {
                                    let texture_id = texture.id();
                                    let sized_texture = egui::load::SizedTexture::from((
                                        texture_id,
                                        egui::vec2(48.0, 48.0),
                                    ));
                                    let the_image = egui::Image::from(sized_texture);

                                    let response = ui
                                        .dnd_drag_source(
                                            egui::Id::new("main_flow_texture"),
                                            main_flow_draggable.clone(),
                                            move |ui| {
                                                ui.add(egui::ImageButton::new(the_image));
                                            },
                                        )
                                        .response;

                                    if response.drag_started() {
                                        println!("Dragging Main Flow started!");
                                    }
                                    if response.drag_stopped() {
                                        println!("Dragging Main Flow stopped!");
                                    }
                                }
                            }
                        });

                        ScrollArea::vertical().show(ui, |ui| {
                            ui.label("Left panel content here.");
                        });
                    });
            } else {
                SidePanel::left("left_panel_toggler")
                    .resizable(false)
                    .default_width(20.0)
                    .show(ctx, |ui| {
                        if ui.button("\u{27A1}").clicked() {
                            self.app.left_panel_open = true;
                        }
                    });
            }
        }

        // Right Panel (collapsible)
        {
            let right_panel_open = self.app.right_panel_open;

            if right_panel_open {
                SidePanel::right("right_panel")
                    .resizable(true)
                    .default_width(150.0)
                    .width_range(80.0..=200.0)
                    .show(ctx, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            if ui.button("\u{27A1}").clicked() {
                                self.app.right_panel_open = !self.app.right_panel_open;
                            }
                        });
                        ui.vertical_centered(|ui| {
                            ui.heading("Right Panel");
                        });
                        ScrollArea::vertical().show(ui, |ui| {
                            lorem_ipsum(ui);
                        });
                    });
            } else {
                SidePanel::right("right_panel_toggler")
                    .resizable(false)
                    .default_width(20.0)
                    .show(ctx, |ui| {
                        if ui.button("\u{2B05}").clicked() {
                            self.app.right_panel_open = !self.app.right_panel_open;
                        }
                    });
            }
        }

        // Bottom Panel (collapsible)
        {
            let bottom_panel_open = self.app.bottom_panel_open;

            if bottom_panel_open {
                TopBottomPanel::bottom("bottom_panel")
                    .resizable(false)
                    .min_height(0.0)
                    .show(ctx, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            if ui.button("\u{2B07}").clicked() {
                                self.app.bottom_panel_open = !self.app.bottom_panel_open;
                            }
                        });
                        ui.vertical_centered(|ui| {
                            ui.heading("Bottom Panel");
                        });
                        if ui.button("Close Bottom Panel").clicked() {
                            self.app.bottom_panel_open = !self.app.bottom_panel_open;
                        }
                    });
            } else {
                TopBottomPanel::bottom("bottom_panel_toggler")
                    .resizable(false)
                    .min_height(20.0)
                    .show(ctx, |ui| {
                        if let Some(texture) = &self.app.arrow_up_texture {
                            if ui.add(ImageButton::new(Image::new(texture).fit_to_exact_size([16.0, 16.0].into()))).clicked() {
                                self.app.bottom_panel_open = true;
                            }
                        }
                    });
            }
        }
    }
}




// Helper function for placeholder text
fn lorem_ipsum(ui: &mut Ui) {
    ui.with_layout(Layout::top_down(Align::LEFT).with_cross_justify(true), |ui| {
        ui.label(RichText::new(LOREM_IPSUM_LONG).small().weak());
    });
}

pub const LOREM_IPSUM_LONG: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

// Main function to run the application
fn main() -> eframe::Result<()> {

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true) , // Start the window maximized to act like full screen
        ..Default::default()
    };

    eframe::run_native(
        "Egui Panels Example",
        options,
        Box::new(|cc| {
            // Create the `Panels` instance with the `Context`
            let app = Panels::new(&cc.egui_ctx);
            // Return the `AppWrapper`
            Ok(Box::new(AppWrapper { app }))
        }),
    )
}
