use relm4::{
    gtk::prelude::*,
    prelude::*,
    typed_view::list::{RelmListItem, TypedListView},
};

use crate::{action::Action, message::Msg, ui::APP_BROKER};

#[derive(Debug)]
pub enum ActionListViewInputMsg {
    SelectNextAction,
    SelectPrevAction,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ActionListViewItem {
    pub action: Action,
}

pub struct ActionListViewItemWidgets {
    name_label: gtk::Label,
}

impl RelmListItem for ActionListViewItem {
    type Root = gtk::Box;

    type Widgets = ActionListViewItemWidgets;

    fn setup(_item: &gtk::ListItem) -> (Self::Root, Self::Widgets) {
        relm4::view! {
            my_box = gtk::Box {
                set_vexpand: true,
                add_css_class: "action-list-view-item-container",

                #[name = "name_label"]
                gtk:: Label {
                    set_use_markup: true,
                    set_justify: gtk::Justification::Left,
                    set_xalign: 0f32,
                    set_expand: true,
                    add_css_class: "action-list-view-item-name",
                },

            }
        }
        (my_box, ActionListViewItemWidgets { name_label })
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        let Self::Widgets { name_label } = widgets;

        name_label.set_label(&self.action.name);
    }
}

#[derive(Debug)]
pub struct ActionListView {
    list_view_wrapper: TypedListView<ActionListViewItem, gtk::SingleSelection>,
}

impl ActionListView {
    pub fn get_selected_action(&self) -> Option<Action> {
        let selected_idx = self.list_view_wrapper.selection_model.selected();

        self.list_view_wrapper
            .get(selected_idx)
            .map(|typed_list_item| typed_list_item.borrow().action.clone())
    }
}

#[relm4::component(pub)]
impl SimpleComponent for ActionListView {
    type Init = Vec<ActionListViewItem>;
    type Input = ActionListViewInputMsg;
    type Output = Msg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            #[local_ref]
            my_view -> gtk::ListView {}
        }
    }

    fn init(
        list_view_items: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut list_view_wrapper: TypedListView<ActionListViewItem, gtk::SingleSelection> =
            TypedListView::new();

        let list_view_items_clone = list_view_items.clone();
        for item in list_view_items {
            list_view_wrapper.append(item);
        }

        list_view_wrapper.view.connect_activate(move |_, b| {
            APP_BROKER.send(Msg::ActionActivated(
                list_view_items_clone[b as usize].action.clone(),
            ));
        });

        let model = ActionListView { list_view_wrapper };

        let my_view = &model.list_view_wrapper.view;

        let widgets = view_output!();

        root.add_css_class("action-listview");

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ActionListViewInputMsg::SelectNextAction => {
                let selected_idx = self.list_view_wrapper.selection_model.selected();

                self.list_view_wrapper
                    .selection_model
                    .set_selected((selected_idx + 1) % self.list_view_wrapper.len());
            }
            ActionListViewInputMsg::SelectPrevAction => {
                let selected_idx = self.list_view_wrapper.selection_model.selected();

                self.list_view_wrapper.selection_model.set_selected(
                    (selected_idx + self.list_view_wrapper.len() - 1)
                        % self.list_view_wrapper.len(),
                );
            }
        }
    }
}
