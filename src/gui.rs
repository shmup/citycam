use iced::widget::{
    center, column, combo_box, scrollable, text, vertical_space,
};
use iced::{Center, Element, Fill};

pub fn run_gui() -> iced::Result {
    iced::run("citycam", CityCam::update, CityCam::view)
}

struct CityCam {
    languages: combo_box::State<Language>,
    selected_language: Option<Language>,
    text: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Selected(Language),
    OptionHovered(Language),
    Closed,
}

impl CityCam {
    fn new() -> Self {
        Self {
            languages: combo_box::State::new(Language::ALL.to_vec()),
            selected_language: None,
            text: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(language) => {
                self.selected_language = Some(language);
                self.text = language.hello().to_string();
            }
            Message::OptionHovered(language) => {
                self.text = language.hello().to_string();
            }
            Message::Closed => {
                self.text = self
                    .selected_language
                    .map(|language| language.hello().to_string())
                    .unwrap_or_default();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text(&self.text),
            "What is your language?",
            vertical_space().height(150),
        ]
        .width(Fill)
        .align_x(Center)
        .spacing(10);

        center(scrollable(content)).into()
    }
}

impl Default for CityCam {
    fn default() -> Self {
        CityCam::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    Danish,
    #[default]
    English,
    French,
    German,
    Italian,
    Portuguese,
    Spanish,
    Other,
}

impl Language {
    const ALL: [Language; 8] = [
        Language::Danish,
        Language::English,
        Language::French,
        Language::German,
        Language::Italian,
        Language::Portuguese,
        Language::Spanish,
        Language::Other,
    ];

    fn hello(&self) -> &str {
        match self {
            Language::Danish => "Halloy!",
            Language::English => "Hello!",
            Language::French => "Salut!",
            Language::German => "Hallo!",
            Language::Italian => "Ciao!",
            Language::Portuguese => "Olá!",
            Language::Spanish => "¡Hola!",
            Language::Other => "... hello?",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Danish => "Danish",
                Language::English => "English",
                Language::French => "French",
                Language::German => "German",
                Language::Italian => "Italian",
                Language::Portuguese => "Portuguese",
                Language::Spanish => "Spanish",
                Language::Other => "Some other language",
            }
        )
    }
}

