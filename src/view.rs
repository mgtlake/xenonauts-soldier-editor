use std::error::Error;
use std::fs;
use std::option::Option::{None, Some};
use std::path::PathBuf;
use std::result::Result::{Err, Ok};

use iced::widget::{button, horizontal_space, row, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use native_dialog::{FileDialog, MessageDialog, MessageType};

use crate::save::{self, Save};

pub fn run() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    filepath: Option<PathBuf>,
    save: Option<Save>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OpenFile,
    SaveFile,
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            filepath: None,
            save: None,
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OpenFile => {
                let path = match FileDialog::new()
                    .set_location("~/Desktop")
                    .add_filter("Save file", &["sav"])
                    .show_open_single_file()
                {
                    Ok(path) => path,
                    Err(e) => {
                        let _ = MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Could not parse open file!")
                            .set_text(&format!("{:#?}", e))
                            .show_alert();
                        None
                    }
                };

                if let Some(path) = path {
                    let save_or_error = load_save(&path);
                    self.save = match save_or_error {
                        Ok(save) => Some(save),
                        Err(e) => {
                            let _ = MessageDialog::new()
                                .set_type(MessageType::Error)
                                .set_title("Could not parse save file!")
                                .set_text(&format!("{:#?}", e))
                                .show_alert();
                            None
                        }
                    };
                    if self.save.is_some() {
                        self.filepath = Some(path);
                    }
                }
            }
            Message::SaveFile => {}
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            button("Open ").padding(10).on_press(Message::OpenFile),
            horizontal_space().width(Length::Fixed(20.0)),
            text(
                &self
                    .filepath
                    .as_ref()
                    .map(|x| x.as_os_str())
                    .and_then(|x| x.to_str())
                    .unwrap_or("")
            )
            .size(20),
            horizontal_space().width(Length::Fill),
            button("Save")
                .padding(10)
                .on_press_maybe(self.save.as_ref().map(|_| Message::SaveFile))
        ]
        .padding(10)
        .align_items(Alignment::Center)
        .into()
    }
}

fn load_save(filepath: &PathBuf) -> Result<Save, Box<dyn Error>> {
    let file = fs::read(filepath)?;
    let (_, save) = save::parse_save(&file).map_err(|err| err.to_owned())?;
    Result::Ok(save)
}
