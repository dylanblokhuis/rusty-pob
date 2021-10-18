use iced::{button, Align, Button, Canvas, Column, Element, Sandbox, Settings, Text};

pub mod tree;

fn main() -> iced::Result {
    TreeCanvas::run(Settings::default())
}

struct TreeCanvas {
    tree: tree::Tree,
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for TreeCanvas {
    type Message = Message;

    fn new() -> Self {
        let path = std::path::Path::new("./data.json");
        let data = std::fs::read_to_string(path).unwrap();
        let tree: tree::Tree = serde_json::from_str(&data).unwrap();

        Self { tree: tree }
    }

    fn title(&self) -> String {
        String::from("TreeCanvas")
    }

    fn update(&mut self, message: Message) {}

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(format!("hello, the tree is {}", self.tree.tree)))
            .into()
    }
}
