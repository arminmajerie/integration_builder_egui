

use egui::{Context, TextureHandle};

// Import the images from the respective modules
mod main_flow_image {
    pub use crate::components::main_flow::image::get_bytes as get_main_flow_bytes;
}

mod choice_image {
    pub use crate::components::choice::image::get_bytes as get_choice_bytes;
}
#[derive(Clone)]
pub(crate) struct DraggableItem {
    pub(crate) texture: Option<TextureHandle>,
    pub(crate) component_level: String,
    pub(crate) name: String,
}

impl DraggableItem {
    pub(crate) fn new(ctx: &Context, name: &str, component_level: &str) -> Self {

        if name == "Choice" {
            Self {
                texture: Self::load_texture(ctx, name),
                component_level: component_level.to_string(),
                name: name.to_string(),
            }
        } else if name == "MainFlow" {
            Self {
                texture: Self::load_texture(ctx, name),
                component_level: component_level.to_string(),
                name: name.to_string(),
            }
        }
        else {
            Self {
                texture: None,
                component_level: component_level.to_string(),
                name: name.to_string(),
            }
        }


    }

    fn load_texture(ctx: &Context, name: &str) -> Option<TextureHandle> {
        // Dynamically determine which image bytes to load
        let image_bytes = match name {
            "MainFlow" => main_flow_image::get_main_flow_bytes(),
            "Choice" => choice_image::get_choice_bytes(),
            _ => return None, // Return None if the name is not recognized
        };

        // Load the texture using the image bytes
        if let Ok(image) = image::load_from_memory(image_bytes) {
            let size = [image.width() as _, image.height() as _];
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image.to_rgba8());
            Some(ctx.load_texture(name, color_image, Default::default()))
        } else {
            None
        }
    }


}