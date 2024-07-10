use std::error::Error;
use std::fs;
use std::option::Option::{None, Some};
use std::path::PathBuf;
use std::result::Result::{Err, Ok};

use iced::widget::{button, horizontal_space, row, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use rfd::{FileDialog, MessageDialog, MessageLevel};

use crate::save::{self, Save};

pub fn run() -> iced::Result {
    Editor::run(Settings::default())
}

// struct SaveFile {
//     path: PathBuf,
//     save: Save,
// }

enum Editor {
    NoData,
    Save { path: PathBuf, save: Save },
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OpenFile,
    SaveFile,
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Editor::NoData
    }

    fn title(&self) -> String {
        String::from("Xenonauts Soldier Editor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OpenFile => {
                let path = FileDialog::new()
                    .add_filter("Save file", &["sav"])
                    .pick_file();

                if let Some(path) = path {
                    let save_or_error = load_save(&path);
                    *self = match save_or_error {
                        Ok(save) => Editor::Save { path, save },
                        Err(e) => {
                            MessageDialog::new()
                                .set_level(MessageLevel::Error)
                                .set_title("Could not open save file!")
                                .set_description(format!("{:#?}", e))
                                .show();
                            Editor::NoData
                        }
                    };
                }
            }
            Message::SaveFile => {
                if let Editor::Save { path, save } = &self {
                    if let Err(e) = fs::write(path, save.serialise()) {
                        MessageDialog::new()
                            .set_level(MessageLevel::Error)
                            .set_title("Could not write updated save file!")
                            .set_description(format!("{:#?}", e))
                            .show();
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            button("Open ").padding(10).on_press(Message::OpenFile),
            text(match self {
                Editor::Save { path, .. } => path.as_os_str().to_str().unwrap_or(""),
                Editor::NoData => "",
            })
            .size(20),
            horizontal_space().width(Length::Fill),
            button("Save").padding(10).on_press_maybe(match self {
                Editor::Save { .. } => Some(Message::SaveFile),
                Editor::NoData => None,
            })
        ]
        .spacing(20)
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
