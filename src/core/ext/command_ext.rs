use clap::Command;

use crate::core::model::CliCommand;

pub trait CommandExt {
    fn load_subcommands(self, subs: &[Box<dyn CliCommand>]) -> Self;
}

impl CommandExt for Command {
    fn load_subcommands(mut self, subs: &[Box<dyn CliCommand>]) -> Self {
        for cmd in subs {
            self = self.subcommand(cmd.build());
        }
        self
    }
}
