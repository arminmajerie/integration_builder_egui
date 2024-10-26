# Integration Builder with `egui`

## Overview

This project is an Integration Builder application built using the `egui` GUI library in Rust. The application provides a graphical interface for users to create and manage integration flows by dragging and dropping components within a panel-based layout.

## Features

- **Panel-Based Layout**: The interface includes collapsible left, right, and bottom panels, along with a central panel for main content.
- **Drag-and-Drop Components**: Users can drag components like `MainFlow` from the left panel and drop them into the central panel to build integration workflows.
- **Dynamic UI Elements**: Components added to the central panel are dynamically displayed and can contain nested items.
- **Customizable Components**: Each `MainFlow` component can be expanded or collapsed and is intended to support adding child components.

## Application Structure

- **Left Panel**: Contains draggable components that can be used to build integration flows.
- **Central Panel**: The main workspace where users can drop components and arrange them vertically.
- **Right Panel**: Intended for additional settings or information (placeholder in this project).
- **Bottom Panel**: Can be toggled for extra functionality or logs (placeholder in this project).

## Usage

1. **Run the Application**: Start the application using `cargo run`.
2. **Drag Components**: From the left panel, drag the `MainFlow` component.
3. **Drop into Central Panel**: Drop the component into the central panel's drop zone.
4. **Interact with Components**: The `MainFlow` components will appear stacked vertically in the central panel.

## Limitations

### Lack of Nested Drag-and-Drop Support in `egui`

The project was intended to support nested drag-and-drop functionality, where users could drag items into and out of nested components (e.g., dragging child components into a `MainFlow` component). However, the current version of `egui` and its drag-and-drop extension `egui_dnd` do not support nested drag-and-drop operations or dragging items between different lists and layers.

#### Details
- **Nested Drag-and-Drop**: Essential for building complex integration flows where components can contain other components.
- **`egui_dnd` Limitations**: The `egui_dnd` crate, which provides drag-and-drop functionality for `egui`, currently lacks the capability to handle nested drag-and-drop interactions.
- **Community Discussions**: This limitation is acknowledged in the community, with discussions and open issues indicating that this feature is not yet implemented.

## Reason for Dropping the Project

Due to the critical requirement for nested drag-and-drop support in the Integration Builder application, and the current inability of `egui` to provide this functionality, the decision was made to discontinue development using `egui`.

### Key Points
- **Essential Feature Missing**: Nested drag-and-drop is a non-negotiable feature for the application's goals.
- **No Workarounds**: Attempts to implement the required functionality with existing `egui` features were unsuccessful.
- **Framework Limitation**: The limitation is inherent to `egui` and would require significant changes or waiting for future updates to the library.

## Future Considerations

- **Alternative GUI Libraries**: Exploring other Rust GUI libraries like `fltk-rs` that support nested drag-and-drop out of the box.
- **Contributing to `egui`**: Potentially contributing to the `egui` project to add the missing functionality, although this may be time-consuming and beyond the project's scope.
- **Waiting for Updates**: Monitoring `egui`'s development for future support of nested drag-and-drop, and possibly revisiting the project at that time.
