use std::path::Path;

use gpui::{Context, Entity, Window, prelude::*, px};
use gpui_component::{
    IndexPath,
    button::Button,
    form::{field, v_form},
    h_flex,
    input::{Input, InputState},
    select::{Select, SelectState},
};
use rfd::AsyncFileDialog;

use crate::decoder::shp::shp_reader::rgba_image_to_png;

pub struct MainView {
    pal_path: Entity<InputState>,
    shp_path: Entity<InputState>,
    output_path: Entity<InputState>,

    select_state: Entity<SelectState<Vec<String>>>,
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
                        .child(Input::new(&self.output_path))
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
                                                    this.set_output_path(path, window, cx);
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
                field()
                    .label("Half shp:")
                    .child(h_flex().px(px(8.)).child(Select::new(&self.select_state))),
            )
            .child(
                field().child(
                    h_flex().px(px(8.)).child(
                        Button::new("b_convert")
                            .label("Convert")
                            .w_full()
                            .on_click(cx.listener(|this, _, _, cx| {
                                if let Err(err) = this.validate_paths(cx) {
                                    println!("{:?}", err);
                                    return;
                                }

                                let is_half = match this.select_state.read(cx).selected_index(cx) {
                                    Some(index ) => index.row == 0,
                                    None => true,
                                };

                                rgba_image_to_png(
                                    Path::new(&this.shp_path.read(cx).value().to_string()),
                                    Path::new(&this.pal_path.read(cx).value().to_string()),
                                    is_half,
                                    Path::new(&this.output_path.read(cx).value().to_string()),
                                )
                                .ok();
                            })),
                    ),
                ),
            )
    }
}

impl MainView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let pal_path = cx.new(|cx| InputState::new(window, cx).placeholder("Enter pal path"));
        let shp_path = cx.new(|cx| InputState::new(window, cx).placeholder("Enter shp path"));
        let output_path = cx.new(|cx| InputState::new(window, cx).placeholder("Enter output path"));

        let select_state = cx.new(|cx| {
            SelectState::new(
                vec!["true".to_string(), "false".to_string()],
                Some(IndexPath::default()),
                window,
                cx,
            )
        });

        Self {
            pal_path,
            shp_path,
            output_path,
            select_state,
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

    fn set_output_path(&self, path: String, window: &mut Window, cx: &mut Context<Self>) {
        self.output_path.update(cx, |state, cx| {
            state.set_value(path, window, cx);
        });
    }

    fn validate_paths(&self, cx: &mut Context<Self>) -> anyhow::Result<()> {
        let pal = self.pal_path.read(cx).value();
        let shp = self.shp_path.read(cx).value();
        let extraction = self.output_path.read(cx).value();

        if pal.is_empty() {
            anyhow::bail!("PAL path is empty");
        }

        if shp.is_empty() {
            anyhow::bail!("SHP path is empty");
        }

        if extraction.is_empty() {
            anyhow::bail!("Extraction path is empty");
        }

        Ok(())
    }
}
