use relm4::{gtk::prelude::*, prelude::*};

use crate::{message::Msg, ui::APP_BROKER};

pub struct InputBox {
    pub input: gtk::EntryBuffer,
}

#[relm4::component(pub)]
impl SimpleComponent for InputBox {
    type Init = String;
    type Input = ();
    type Output = ();

    view! {
        gtk::Box {
            add_css_class: "input-box-container",
            set_orientation: gtk::Orientation::Horizontal,

            append = &gtk::Image {
                add_css_class: "input-box-icon",
                set_icon_name: Some("accessories-text-editor"),
            },

            #[name = "input_entry"]
            append = &gtk::Entry {
                add_css_class: "input-box-entry",
                set_placeholder_text: Some("Input"),
                set_sensitive: true,
                set_expand: true,
                set_buffer: &model.input,

                connect_changed => move | buffer | {
                    APP_BROKER.send(Msg::InputChanged(buffer.text().to_string()));
                }
            },
        }
    }

    fn init(
        input: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = InputBox {
            input: gtk::EntryBuffer::new(Some(&input)),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _: Self::Input, _sender: ComponentSender<Self>) {}
}
