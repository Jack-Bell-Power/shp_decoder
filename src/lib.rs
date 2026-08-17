use gpui::{
    App, Application, Bounds, Global, Subscription, WindowBounds, WindowOptions, prelude::*, px,
    size,
};
use gpui_component::Root;

use crate::{config::AppConfig, main_view::MainView};

mod config;
mod decoder;
mod main_view;

struct AppState {
    _quit_subscription: Subscription,
    config: AppConfig,
}

impl Global for AppState {}

pub fn run() {
    Application::new()
        .with_assets(gpui_component_assets::Assets)
        .run(|cx: &mut App| {
            gpui_component::init(cx);

            let bounds = Bounds::centered(None, size(px(600.0), px(320.0)), cx);

            let _quit_subscription = cx.on_app_quit(|cx| {
                let config = cx.global::<AppState>().config.clone();

                async move {
                    config.save();
                }
            });

            let config = AppConfig::load();

            cx.set_global(AppState {
                _quit_subscription,
                config,
            });

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
