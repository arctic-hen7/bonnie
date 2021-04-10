use std::collections::HashMap;
use crate::command::Command;

pub struct CommandsRegistry<'a> {
    map: HashMap<String, Command<'a>>
}
impl<'a> CommandsRegistry<'a> {
    pub fn new() -> CommandsRegistry<'a> {
        CommandsRegistry {
            map: HashMap::new()
        }
    }

    pub fn add(&mut self, name: &str, command: Command<'a>) {
        self.map.insert(name.to_string(), command);
    }

    pub fn remove(&mut self, name: &str) {
        self.map.remove(&name.to_string());
    }

    pub fn get(&self, name: &str) -> &Command {
        let entry = self.map.get(name);
        let entry = match entry {
            Some(entry) => entry,
            None => panic!("Command '{}' not found.", &name),
        };

        entry
    }
}
