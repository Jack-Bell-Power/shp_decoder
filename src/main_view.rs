use gpui::{Context, Entity, Window, prelude::*, px};
use gpui_component::{
    button::Button,
    form::{field, v_form},
    h_flex,
    input::{Input, InputState},
};
use rfd::FileDialog;

pub struct MainView {
    pal_path: Entity<InputState>,
    shp_path: Entity<InputState>,
    extraction_path: Entity<InputState>,
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_form()
            .gap(px(12.))
            .child(
                field().label("Pal path:").child(
                    h_flex()
                        .gap(px(8.))
                        .px(px(8.))
                        .child(Input::new(&self.pal_path))
                        .child(
                            Button::new("b_pal")
                                .label("Open")
                                .on_click(cx.listener(|this, _, window, cx| {
                                    if let Some(path) = FileDialog::new()
                                        .set_title("Select Palette File")
                                        .add_filter("Palette", &["pal"])
                                        .pick_file()
                                    {
                                        this.pal_path.update(cx, |state, cx| {
                                        state.set_value(
                                            path.to_string_lossy().to_string(),
                                            window,
                                            cx,
                                        );
                                    });
                                    }
                                })),
                        ),
                ),
            )
            .child(
                field().label("Shp path:").child(
                    h_flex()
                        .gap(px(8.))
                        .px(px(8.))
                        .child(Input::new(&self.shp_path))
                        .child(Button::new("b_shp").label("Open").on_click(cx.listener(
                            |this, _, window, cx| {
                                if let Some(path) = FileDialog::new()
                                    .set_title("Select Shape File")
                                    .add_filter("Shape", &["shp"])
                                    .pick_file()
                                {
                                    this.shp_path.update(cx, |state, cx| {
                                        state.set_value(
                                            path.to_string_lossy().to_string(),
                                            window,
                                            cx,
                                        );
                                    });
                                }
                            },
                        ))),
                ),
            )
            .child(
                field().label("Extraction path:").child(
                    h_flex()
                        .gap(px(8.))
                        .px(px(8.))
                        .child(Input::new(&self.extraction_path))
                        .child(
                            Button::new("b_extraction")
                                .label("Open")
                                .on_click(|_, _, _| {
                                    println!("extraction_path:");
                                }),
                        ),
                ),
            )
            .child(
                field().child(
                    h_flex().px(px(8.)).child(
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
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let pal_path = cx.new(|cx| InputState::new(window, cx).placeholder("Enter pal path"));
        let shp_path = cx.new(|cx| InputState::new(window, cx).placeholder("Enter shp path"));
        let extraction_path =
            cx.new(|cx| InputState::new(window, cx).placeholder("Enter extraction path"));

        Self {
            pal_path,
            shp_path,
            extraction_path,
        }
    }
}
