mod collapsible;
mod card_view;

use collapsible::Collapsible;
use iced::widget::{button, column, text, Column};
use iced::{Element, Length};
use card_view::card_view;


pub fn main() -> iced::Result {
    iced::run("Component - Iced", ComponentTest::update, ComponentTest::view)
}

struct ComponentTest {
    inputs: Vec<usize>
}

impl Default for ComponentTest {
    fn default() -> Self {
        Self { inputs: vec![0,1] }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Add
}

impl ComponentTest {
    fn update(&mut self, message: Message) {
        match message {
            Message::Add => {
                self.inputs.push(self.inputs.len());
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let el = self.inputs.iter().fold(Column::new().spacing(10), |col, item| {
            col.push(Collapsible::new(move|visible| {
                // Using Length::Fill here doesn't work
                if visible {
                    column![
                        card_view("Title", text!("Card view: {}", item).into(), Length::Shrink),
                    ].into()
                } else {
                    column![].into()
                }
            }))

            // This works
            // col.push(column![
            //     card_view("Title", text!("Card view: {}", item).into(), Length::Fill)
            // ])
        });

        column![
            button("Add").on_press(Message::Add),
            el
        ].into()
    }
}
