use relm4::{
    WorkerController,
    gtk::{CssProvider, gdk, glib, prelude::*},
    prelude::*,
};
use std::convert::identity;

use crate::{
    action::{Action, ActionWorker},
    message::Msg,
    ui::{
        action_list_view::{ActionListView, ActionListViewItem},
        input_box::InputBox,
    },
};

use super::{APP_BROKER, action_list_view::ActionListViewInputMsg};

pub struct AppInit {
    pub actions: Vec<Action>,
    pub input: String,
}

pub struct App {
    input: String,
    input_box: Controller<InputBox>,
    action_list_view: Controller<ActionListView>,
    action_worker: WorkerController<ActionWorker>,
    output_buffer: gtk::TextBuffer,
    output_string: String,
    has_buffer: bool,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = AppInit;
    type Input = Msg;
    type Output = ();

    view! {
        #[name = "window"]
        gtk::ApplicationWindow {
            set_title: Some("roki"),
            set_resizable: false,
            // TODO: config
            set_default_width: 700,
            set_modal: true,
            set_decorated: false,
            set_expand: true,
            set_vexpand: true,

            #[name = "ui"]
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                append = model.input_box.widget(),
                append =  model.action_list_view.widget(),

                append = &gtk::ScrolledWindow {
                    add_css_class: "output-container",
                    #[watch]
                    set_visible: model.has_buffer,

                    set_min_content_height: 500,

                    #[wrap(Some)]
                    set_child = &gtk::TextView {
                        add_css_class: "output-textview",
                        set_editable: false,
                        set_margin_all: 5,
                        set_wrap_mode: gtk::WrapMode::WordChar,

                        set_buffer: Some(&model.output_buffer),
                    },
                }

            }
        }
    }

    fn init(
        app_init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let action_worker = ActionWorker::builder()
            .detach_worker(app_init.actions.clone())
            .forward(sender.input_sender(), identity);

        let list_view_items = app_init
            .actions
            .into_iter()
            .map(|action| ActionListViewItem { action })
            .collect::<Vec<_>>();

        let model = App {
            output_buffer: gtk::TextBuffer::new(None),
            has_buffer: false,
            output_string: String::default(),
            input: app_init.input.clone(),
            input_box: InputBox::builder().launch(app_init.input).detach(),
            action_list_view: ActionListView::builder().launch(list_view_items).detach(),
            action_worker,
        };

        let widgets = view_output!();

        add_key_pressed_event(&widgets.window);

        let css_provider = CssProvider::new();
        css_provider.load_from_data(include_str!("default.css"));

        gtk::style_context_add_provider_for_display(
            &widgets.ui.display(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::ActionActivated(action) => {
                self.action_worker.emit((action, self.input.clone()));
            }
            Msg::CtrlEnterPressed => {
                if let Some(action) = self.action_list_view.model().get_selected_action() {
                    self.action_worker.emit((action, self.input.clone()));
                }
            }
            Msg::CommandActivated => {
                self.has_buffer = true;
                self.output_string = String::default();
                self.output_buffer.set_text(&self.output_string);
            }
            Msg::OutputGenerated(output) => {
                self.has_buffer = true;
                self.output_string
                    .push_str(&format!("{}\n", output.replace("**", "")));
                self.output_buffer.set_text(&self.output_string);
            }
            Msg::InputChanged(input) => {
                self.input = input;
            }
            Msg::SelectNextAction => {
                self.action_list_view
                    .emit(ActionListViewInputMsg::SelectNextAction);
            }
            Msg::SelectPrevAction => {
                self.action_list_view
                    .emit(ActionListViewInputMsg::SelectPrevAction);
            }
        }
    }
}

fn add_key_pressed_event(window: &gtk::ApplicationWindow) {
    let event_controller = gtk::EventControllerKey::new();

    event_controller.connect_key_pressed(|_, key, _, modifier| {
        match key {
            gdk::Key::Escape => {
                relm4::main_application().quit();
            }
            gdk::Key::j => {
                if modifier.contains(gdk::ModifierType::CONTROL_MASK) {
                    APP_BROKER.send(Msg::SelectNextAction);
                }
            }
            gdk::Key::k => {
                if modifier.contains(gdk::ModifierType::CONTROL_MASK) {
                    APP_BROKER.send(Msg::SelectPrevAction);
                }
            }
            gdk::Key::Return => {
                if modifier.contains(gdk::ModifierType::CONTROL_MASK) {
                    APP_BROKER.send(Msg::CtrlEnterPressed);
                }
            }
            _ => (),
        }
        glib::Propagation::Proceed
    });

    window.add_controller(event_controller);
}
