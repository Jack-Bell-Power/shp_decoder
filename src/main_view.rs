use gpui::{Context, Entity, Window, prelude::*, px};
use gpui_component::{
    button::Button,
    form::{field, v_form},
    h_flex,
    input::{Input, InputState},
};
use rfd::{AsyncFileDialog, FileDialog};

pub struct MainView {
    pal_path: Entity<InputState>,
    shp_path: Entity<InputState>,
    extraction_path: Entity<InputState>,
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal_path = self.pal_path.clone();
        v_form()
            .gap(px(12.))
            .child(
                field().label("Pal path:").child(
                    h_flex()
                        .gap(px(8.))
                        .px(px(8.))
                        .child(Input::new(&self.pal_path))
                        .child(Button::new("b_pal").label("Open").on_click(cx.listener(
                            |_, _, window, cx| {
                                cx.spawn_in(window, async |this, cx| {
                                    if let Some(file) = AsyncFileDialog::new()
                                        .set_title("Select Palette File")
                                        .add_filter("Palette", &["pal"])
                                        .pick_file()
                                        .await
                                    {
                                        let path = file.path().to_string_lossy().to_string();
                                        cx.update(|window, cx| {
                                            this.update(cx, |this, cx| {
                                                this.set_pal_path(path, window, cx);
                                            })
                                            .ok();
                                        })
                                        .ok();
                                    }
                                })
                                .detach();
                            },
                        ))),
                ),
            )
            .child(
                field().label("Shp path:").child(
                    h_flex()
                        .gap(px(8.))
                        .px(px(8.))
                        .child(Input::new(&self.shp_path))
                        .child(Button::new("b_shp").label("Open").on_click(cx.listener(
                            |_, _, window, cx| {
                                cx.spawn_in(window, async |this, cx| {
                                    if let Some(file) = AsyncFileDialog::new()
                                        .set_title("Select Shape File")
                                        .add_filter("Shape", &["shp"])
                                        .pick_file()
                                        .await
                                    {
                                        let path = file.path().to_string_lossy().to_string();
                                        cx.update(|window, cx| {
                                            this.update(cx, |this, cx| {
                                                this.set_shp_path(path, window, cx);
                                            })
                                            .ok();
                                        })
                                        .ok();
                                    }
                                })
                                .detach();
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
                                .on_click(cx.listener(|_, _, window, cx| {
                                    cx.spawn_in(window, async |this, cx| {
                                        if let Some(file) = AsyncFileDialog::new()
                                            .set_title("Select extraction path")
                                            .pick_folder()
                                            .await
                                        {
                                            let path = file.path().to_string_lossy().to_string();
                                            cx.update(|window, cx| {
                                                this.update(cx, |this, cx| {
                                                    this.set_extraction_path(path, window, cx);
                                                })
                                                .ok();
                                            })
                                            .ok();
                                        }
                                    })
                                    .detach();
                                })),
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

    fn set_pal_path(&self, path: String, window: &mut Window, cx: &mut Context<Self>) {
        self.pal_path.update(cx, |state, cx| {
            state.set_value(path, window, cx);
        });
    }

    fn set_shp_path(&self, path: String, window: &mut Window, cx: &mut Context<Self>) {
        self.shp_path.update(cx, |state, cx| {
            state.set_value(path, window, cx);
        });
    }

    fn set_extraction_path(&self, path: String, window: &mut Window, cx: &mut Context<Self>) {
        self.extraction_path.update(cx, |state, cx| {
            state.set_value(path, window, cx);
        });
    }
}
