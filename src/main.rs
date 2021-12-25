use iced::{
    text_input, Align, Column, Container, Element, Length, Row, Rule, Sandbox, Settings, Text,
    TextInput,
};
use spire_calculator::draw_rates::{deck_draw_rate, single_turn_sum};

pub fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

#[derive(Debug, Default, Clone)]
struct Calculator {
    // Deck Size
    deck_string: text_input::State,
    deck_value: String,
    num_deck: f32,
    // Number of Target cards in deck
    target_string: text_input::State,
    target_value: String,
    num_target: f32,
    // Number of cards drawn per turn
    drawn_string: text_input::State,
    drawn_value: String,
    drawn_per_turn: f32,

    // Results
    draw_single: f32,
    draw_deck: f32,
    // // Block cards played
    // blocks: Vec<u32>,
    // // Enemy attacks
    // attacks: Vec<u32>,
}

#[derive(Debug, Clone)]
enum Message {
    DeckInputChanged(String),
    TargetInputChanged(String),
    DrawnInputChanged(String),
}

impl Calculator {
    fn update_stats(&mut self) {
        self.num_deck = self.deck_value.trim().parse::<f32>().unwrap_or(1.0);
        self.num_target = self.target_value.trim().parse::<f32>().unwrap_or(1.0);
        self.drawn_per_turn = self.drawn_value.trim().parse::<f32>().unwrap_or(1.0);

        self.draw_single =
            single_turn_sum(self.drawn_per_turn, self.num_target, self.num_deck, true);
        self.draw_deck = deck_draw_rate(self.drawn_per_turn, self.num_target, self.num_deck);
    }
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Calculator::default()
    }

    fn title(&self) -> String {
        String::from("Slay the Spire Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DeckInputChanged(item) => self.deck_value = item,
            Message::TargetInputChanged(item) => self.target_value = item,
            Message::DrawnInputChanged(item) => self.drawn_value = item,
        };
        self.update_stats();
    }

    fn view(&mut self) -> Element<Message> {
        let deck_title = Text::new(String::from("Cards in Deck")).size(20);

        let deck_input = TextInput::new(
            &mut self.deck_string,
            "25",
            &self.deck_value,
            Message::DeckInputChanged,
        )
        .padding(10)
        .size(20);

        let target_title = Text::new(String::from("Target Cards in Deck")).size(20);

        let target_input = TextInput::new(
            &mut self.target_string,
            "5",
            &self.target_value,
            Message::TargetInputChanged,
        )
        .padding(10)
        .size(20);

        let drawn_title = Text::new(String::from("Drawn per Turn")).size(20);

        let drawn_input = TextInput::new(
            &mut self.drawn_string,
            "5",
            &self.drawn_value,
            Message::DrawnInputChanged,
        )
        .padding(10)
        .size(20);

        let res_deck = Text::new((&mut self.draw_deck).to_string() + "%").size(20);
        let res_single = Text::new((&mut self.draw_single).to_string() + "%").size(20);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(deck_title)
            .push(deck_input)
            .push(target_title)
            .push(target_input)
            .push(drawn_title)
            .push(drawn_input)
            .push(
                Row::new()
                    .spacing(10)
                    .height(Length::Units(100))
                    .align_items(Align::Center)
                    .push(Text::new(String::from("% Draw 1")).size(20))
                    .push(Rule::vertical(38))
                    .push(res_single),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .height(Length::Units(100))
                    .align_items(Align::Center)
                    .push(Text::new(String::from("% Draw 1 per turn through Deck")).size(20))
                    .push(Rule::vertical(38))
                    .push(res_deck),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
