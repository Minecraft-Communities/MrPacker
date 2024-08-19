#![allow(unused_imports)]

use iced::{Element, Sandbox, Settings};
use iced::widget::text;

fn main() -> iced::Result {
    StatePacker::run(Settings::default())
}

struct StatePacker {
    localPacks: Vec<PackVirtual>,
}

#[derive(Debug, Clone)]
enum MessagePacker {

}

impl Sandbox for StatePacker {
    type Message = MessagePacker;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Mr. Packer")
    }

    fn update(&mut self, _message: MessagePacker) {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, MessagePacker> {
        text("Hello, world!").into()
    }
}
