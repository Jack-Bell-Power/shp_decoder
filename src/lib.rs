use gpui::{
    App, Application, Bounds, WindowBounds, WindowOptions, prelude::*, px,
    size,
};
use gpui_component::Root;

use crate::main_view::MainView;

mod main_view;
mod decoder;

pub fn run() {
    Application::new()
        .with_assets(gpui_component_assets::Assets)
        .run(|cx: &mut App| {
            gpui_component::init(cx);

            let bounds = Bounds::centered(None, size(px(600.0), px(320.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some("Shp Decoder".into()),
                        ..Default::default()
                    }),
                    is_resizable: false,
                    is_minimizable: false,
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| MainView::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .unwrap();
        });
}
