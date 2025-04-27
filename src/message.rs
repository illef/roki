use crate::action::Action;

#[derive(Debug)]
pub enum Msg {
    ActionActivated(Action),
    CommandActivated,
    OutputGenerated(String),
    InputChanged(String),
    SelectNextAction,
    SelectPrevAction,
    CtrlEnterPressed,
}
