use gpui::{
    App, Application, Bounds, Context, Entity, Window, WindowBounds, WindowOptions, prelude::*, px,
    size,
};
use gpui_component::{
    Root,
    button::Button,
    form::{field, v_form},
    h_flex,
    input::{Input, InputState},
};

struct MainView {
    pal_path: Entity<InputState>,
    shp_path: Entity<InputState>,
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_form()
            .gap(px(12.))
            .child(
                field().label("Pal path:").child(
                    h_flex()
                        .gap(px(8.))
                        .px(px(8.))
                        .child(Input::new(&self.pal_path))
                        .child(Button::new("b_pal").label("Open").on_click(|_, _, _| {
                            println!("pal_path:");
                        })),
                ),
            )
            .child(
                field().label("Shp path:").child(
                    h_flex()
                        .gap(px(8.))
                        .px(px(8.))
                        .child(Input::new(&self.shp_path))
                        .child(Button::new("b_shp").label("Open").on_click(|_, _, _| {
                            println!("shp_path:");
                        })),
                ),
            )
            .child(
                field().child(
                    h_flex()
                        .px(px(8.))
                        .child(
                            Button::new("b_convert")
                                .label("Convert")
                                .w_full()
                                .on_click(|_, _, _| {
                                    println!("Convert");
                                }),
                        ),
                ),
            )
    }
}

impl MainView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let pal_path = cx.new(|cx| InputState::new(window, cx).placeholder("Enter pal path"));
        let shp_path = cx.new(|cx| InputState::new(window, cx).placeholder("Enter shp path"));

        Self { pal_path, shp_path }
    }
}

fn main() {
    Application::new()
        .with_assets(gpui_component_assets::Assets)
        .run(|cx: &mut App| {
            gpui_component::init(cx);

            let bounds = Bounds::centered(None, size(px(600.0), px(180.0)), cx);
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
