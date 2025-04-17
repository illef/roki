use app::{App, AppInit};
use relm4::{MessageBroker, prelude::*};

use crate::message::Msg;

pub mod action_list_view;
pub mod app;
pub mod input_box;

pub(crate) static APP_BROKER: MessageBroker<Msg> = MessageBroker::new();

pub fn run_app(app_init: AppInit) {
    let app = RelmApp::new("illef.roki")
        .with_broker(&APP_BROKER)
        .with_args(vec![]);
    app.run::<App>(app_init);
}
