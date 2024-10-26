use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
// Placeholder for TransformScriptConfig (since details are missing)
pub struct TransformScriptConfig;

#[derive(Debug, Clone)]
pub struct DefaultWidgetFactory;

// Enum to represent ComponentLevel
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ComponentLevel {
    ParentOnly,
    ChildOnly,
    Other,
}

impl ComponentLevel {
    pub fn as_str(&self) -> &str {
        match self {
            ComponentLevel::ParentOnly => "ParentOnly",
            ComponentLevel::ChildOnly => "ChildOnly",
            ComponentLevel::Other => "Other",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ParentOnly" => Some(ComponentLevel::ParentOnly),
            "ChildOnly" => Some(ComponentLevel::ChildOnly),
            "Other" => Some(ComponentLevel::Other),
            _ => Some(ComponentLevel::Other),
        }
    }
}

#[derive(Debug, Clone)]
struct SelectedScriptIndex {
    value: i32,
}

impl SelectedScriptIndex {
    fn new(value: i32) -> Self {
        Self { value }
    }
}

// Size struct similar to Flutter's Size
#[derive(Debug, Clone, Copy)]
struct Size {
    width: f64,
    height: f64,
}

impl Size {
    const ZERO: Self = Self { width: 0.0, height: 0.0 };
}

pub(crate) struct ConfigParams {
    widget_type: String,
    is_expanded: bool,
    name: String,

}

// The equivalent of the BaseConfiguration class in Rust
#[derive(Debug, Clone)]
pub(crate) struct BaseConfiguration {
    pub(crate) uuid: String,
    pub(crate) children: Vec<Arc<Mutex<BaseConfiguration>>>,
    pub(crate) child_uuids: Vec<String>,
    pub(crate) box_initial_width: f64,
    pub(crate) widget_type: String,
    pub(crate) component_level: ComponentLevel,
    pub(crate) name: String,
    pub(crate) parent: Option<Arc<Mutex<BaseConfiguration>>>,
    pub(crate) configuration_file_path: String,
    pub(crate) project_folder: String,
    pub(crate) is_first: bool,
    pub(crate) is_last: bool,
    pub(crate) widget_factory: Option<DefaultWidgetFactory>, // Placeholder, implement as needed
    pub(crate) new_component: bool,
    pub(crate) size: Size,
    pub(crate) child_max_height: f64,
    pub(crate) child_max_width: f64,
    pub(crate) child_total_width: f64,
    pub(crate) child_total_height: f64,
    pub(crate) child_arrow_size: Size,
    pub(crate) label: String,
    pub(crate) initial_state: String,
    pub(crate) max_concurrency: i32,
    pub(crate) is_tracking_enabled: bool,
    pub(crate) is_expanded: bool,
    pub(crate) is_selected: bool,
    pub(crate) error_handling_expanded: bool,
    pub(crate) variable_changed: String,
    pub(crate) debug: bool,
    pub(crate) is_deleted: bool,
    pub(crate) transform_script_configs: Vec<Arc<Mutex<TransformScriptConfig>>>,
    pub(crate) initialize_field_getters: bool,
    pub(crate) initialize_field_setters: bool,
    pub(crate) selected_script_index: SelectedScriptIndex,
    pub(crate) script_index: i32,
}

impl BaseConfiguration {
    // Constructor for BaseConfiguration

    pub(crate) fn default() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            children: Vec::new(),
            child_uuids: Vec::new(),
            box_initial_width: 0.0,
            widget_type: String::new(),
            component_level: ComponentLevel::ParentOnly,
            name: String::new(),
            parent: None,
            configuration_file_path: String::new(),
            project_folder: String::new(),
            is_first: false,
            is_last: false,
            widget_factory: None, // Implement as necessary
            new_component: true,
            size: Size::ZERO,
            child_max_height: 0.0,
            child_max_width: 0.0,
            child_total_width: 0.0,
            child_total_height: 0.0,
            child_arrow_size: Size::ZERO,
            label: String::new(),
            initial_state: String::new(),
            max_concurrency: 5,
            is_tracking_enabled: false,
            is_expanded: false,
            is_selected: false,
            error_handling_expanded: false,
            variable_changed: String::new(),
            debug: false,
            is_deleted: false,
            transform_script_configs: Vec::new(),
            initialize_field_getters: false,
            initialize_field_setters: false,
            selected_script_index: SelectedScriptIndex::new(0),
            script_index: 0,
        }
    }

    // Setter methods for optional fields
    pub(crate) fn widget_type(mut self, widget_type: String) -> Self { self.widget_type = widget_type;self }
    pub(crate) fn box_initial_width(mut self, width: f64) -> Self { self.box_initial_width = width;self }
    pub(crate) fn is_expanded(mut self, expanded: bool) -> Self { self.is_expanded = expanded;self }
    pub(crate) fn name(mut self, name: String) -> Self { self.name = name;self }
}



