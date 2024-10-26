
use egui::{Context, Frame, Ui, Color32, vec2, Id, Align2, Order, CentralPanel, Window};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use eframe::emath::Align;
use crate::base_configuration::{BaseConfiguration, ComponentLevel, ConfigParams};

// Placeholder for MainFlow struct
pub struct MainFlow {
    pub is_expanded: bool,
    pub config: Arc<Mutex<BaseConfiguration>>,
    pub title: String, // Title of the window/frame
}

impl MainFlow {

    pub fn show_in_panel(&mut self, ui: &mut Ui, desired_size: egui::Vec2, label: &str) {
        // Create a frame or container for MainFlow
        ui.group(|ui| {
            ui.set_min_size(desired_size);

            // Draw the MainFlow title and size as part of a label
            ui.horizontal(|ui| {
                ui.label(label);

                let size = ui.min_size();
                let size_text = format!("Size: {:.1} x {:.1}", size.x, size.y);
                ui.label(size_text);
            });

            // Draw the content of the MainFlow
            self.show_contents(ui);
        });
    }

    pub fn show_contents(&mut self, ui: &mut Ui) {
        // Drag-and-drop area and children display
        let frame = egui::Frame {
            fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
            ..Default::default()
        };

        // Drag-and-drop area and children display
        let (response, dropped_payload) =
            ui.dnd_drop_zone::<BaseConfiguration, ()>(frame,|ui| {
                ui.layer_id().order = Order::Background;
                ui.allocate_space(ui.available_size());

                if let Ok(config) = self.config.lock() {
                    for child in config.children.iter() {
                        let unwrapped_child = child.lock().unwrap();

                        ui.label(format!("Child: {}", unwrapped_child.uuid));
                    }
                } else {
                    println!("Failed to acquire lock on BaseConfiguration.");
                }
            });

        if let Some(new_child) = dropped_payload {
            if let Ok(mut config) = self.config.lock() {
                let unwrapped_child = Arc::try_unwrap(new_child).unwrap();
                config.children.push(Arc::new(Mutex::from(unwrapped_child)));
            } else {
                println!("Failed to acquire lock on BaseConfiguration.");
            }
        }
    }


    pub fn new(title: String) -> Self {
        Self {
            is_expanded: true,
            title: title,
            config: Arc::new(Mutex::new(
                BaseConfiguration::default().widget_type("MainFlow".to_string()),
            )),
        }
    }

    pub fn from_base_config(config: Arc<Mutex<BaseConfiguration>>) -> Self {
        let title = {
            let config_lock = config.lock().unwrap();
            config_lock.name.clone()
        };
        Self {
            is_expanded: true,
            title,
            config,
        }
    }

    pub fn set_expansion(&mut self, expanded: bool) {
        self.is_expanded = expanded;
    }

    // Method to set the title
    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    // Function to toggle expansion/collapse
    pub fn toggle_expansion(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    // Add child widget to the MainFlow if it meets the requirements (ComponentLevel::ChildOnly)
    pub fn add_child(&mut self, child: BaseConfiguration) {
        if child.component_level != ComponentLevel::ChildOnly {
            return;
        }

        if let Ok(mut config) = self.config.lock() {
            config.children.push(Arc::new(Mutex::new(child)));
        } else {
            println!("Failed to acquire lock on BaseConfiguration.");
        }
    }

    // Render the MainFlow window with drag-and-drop functionality
    pub fn show(&mut self, ctx: &Context) {
        // Define the title and toggle button
        let frame_title = if self.is_expanded {
            format!("▼ {}", self.title)
        } else {
            format!("▶ {}", self.title)
        };

        if self.is_expanded {
            Window::new(&frame_title)
                .collapsible(true)
                .resizable(true)
                .show(ctx, |ui| {
                    let frame = eframe::egui::Frame {
                        fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
                        ..Default::default()
                    };
                    // Drag-and-drop area and children display
                    let (_response, dropped_payload) = ui.dnd_drop_zone::<Mutex<BaseConfiguration>, ()>(frame, |ui| {
                        ui.layer_id().order = Order::Background;
                        ui.allocate_space(ui.available_size());

                        // Lock the config to access children
                        if let Ok(config) = self.config.lock() {
                            for child in config.children.iter() {
                                // Lock the child to access its data
                                if let Ok(child) = child.lock() {
                                    ui.label(format!("Child: {}", child.name));
                                }
                            }
                        }
                    });

                    if let Some(new_child) = dropped_payload {
                        if let Ok(mut config) = self.config.lock() {
                            config.children.push(new_child);
                        } else {
                            println!("Failed to acquire lock on BaseConfiguration.");
                        }
                    }
                });
        }
    }

}




