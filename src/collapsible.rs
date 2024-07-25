use iced::{widget::{button, column, Component, component}, Element};

#[derive(Debug, Clone)]
pub enum Event<M: Clone> {
    Toggle,
    Message(M),
}

pub struct Collapsible<'a, Message> {
    view: Box<dyn Fn(bool) -> Element<'a, Message> + 'a>
}

impl<'a, Message> Collapsible<'a, Message> {
    pub fn new(view: impl Fn(bool) -> Element<'a, Message> + 'a) -> Self {
        Self {
            view: Box::new(view)
        }
    }
}

impl <'a, Message> Component<Message, iced::Theme> for Collapsible<'a, Message> 
where Message : Clone
{
    type State = bool;

    type Event = Event<Message>;

    fn update(
        &mut self,
        state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            Event::Toggle => {
                *state = !(*state);
                None
            }
            Event::Message(msg) => {
                Some(msg)
            }
        }
    }

    fn view(
        &self,
        state: &Self::State,
    ) -> Element<'_, Self::Event, iced::Theme> {
        column![
            button("Toggle").on_press(Event::Toggle),
            (self.view)(!*state).map(Event::Message),
        ]
        .into()
    }
}


impl<'a, Message> From<Collapsible<'a, Message>> for Element<'a, Message>
    where Message: Clone + 'a,
{
    fn from(numeric_input: Collapsible<'a, Message>) -> Self {
        component(numeric_input)
    }
}