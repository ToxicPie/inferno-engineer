use bevy::prelude::*;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(resize_canvas);
    }
}

// resizes window to fill the entire canvas (when in web)
fn resize_canvas(mut windows: ResMut<Windows>) {
    let Some(bevy_window) = windows.get_primary_mut() else { return; };
    let Some(web_window) = web_sys::window() else { return; };

    let width = web_window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height = web_window.inner_height().unwrap().as_f64().unwrap() as f32;

    if bevy_window.width() != width || bevy_window.height() != height {
        bevy_window.set_resolution(width, height);
    }
}
