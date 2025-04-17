use anyhow::Result;
use relm4::{ComponentSender, Worker};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use crate::message::Msg;

pub struct CommandOutput {
    pub stdout: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Action {
    pub name: String,
    pub command: Option<String>,
}

pub struct ActionWorker {
    pub actions: Vec<Action>,
}

impl Worker for ActionWorker {
    type Init = Vec<Action>;
    type Input = (Action, String);
    type Output = Msg;

    fn init(actions: Self::Init, _sender: ComponentSender<Self>) -> Self {
        ActionWorker { actions }
    }

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>) {
        let (action, input) = input;
        if let Some(action) = self.actions.iter().find(|a| **a == action) {
            sender.output(Msg::CommandActivated).unwrap();
            match action.run_command(&input, sender.clone()) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("command error {}", err);
                }
            }
        }
    }
}

impl Action {
    pub fn run_command(&self, input: &str, sender: ComponentSender<ActionWorker>) -> Result<()> {
        if let Some(command) = &self.command {
            let input = input.trim().replace("'", "\\'").replace("\n", " ");
            let command_str = command.replace("{}", &input);

            println!("command_str: {}", command_str);

            let child = Command::new("fish")
                .arg("-c")
                .arg(&command_str)
                .stdout(Stdio::piped())
                .spawn()?;

            for line in BufReader::new(child.stdout.unwrap()).lines() {
                if let Ok(line) = line {
                    sender.output(Msg::OutputGenerated(line)).unwrap();
                }
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Command not found"))
        }
    }
}
