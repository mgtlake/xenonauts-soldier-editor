use std::error::Error;
use std::fs;
use std::option::Option::{None, Some};
use std::path::PathBuf;
use std::result::Result::{Err, Ok};

use iced::theme::Button;
use iced::widget::text_editor::Edit;
use iced::widget::{button, column, horizontal_space, keyed_column, row, text, text_input};
use iced::{Alignment, Element, Length, Padding, Sandbox, Settings, Theme};
use rfd::{FileDialog, MessageDialog, MessageLevel};

use crate::save::{self, Save};
use crate::soldier::{self, Soldier, SoldierStats};

pub fn run() -> iced::Result {
    Editor::run(Settings::default())
}

// struct SaveFile {
//     path: PathBuf,
//     save: Save,
// }

enum Editor {
    NoData,
    Save {
        path: PathBuf,
        save: Save,
        selected_soldier_id: usize,
    },
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OpenFile,
    SaveFile,
    SelectSoldier { id: usize },
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
                        Ok(save) => {
                            let selected_soldier_id =
                                save.soldiers.get(0).map(|soldier| soldier.id).unwrap_or(0)
                                    as usize;
                            Editor::Save {
                                path,
                                save,
                                selected_soldier_id,
                            }
                        }
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
                if let Editor::Save { path, save, .. } = &self {
                    if let Err(e) = fs::write(path, save.serialise()) {
                        MessageDialog::new()
                            .set_level(MessageLevel::Error)
                            .set_title("Could not write updated save file!")
                            .set_description(format!("{:#?}", e))
                            .show();
                    }
                }
            }
            Message::SelectSoldier { id } => {
                if let Editor::Save {
                    selected_soldier_id,
                    ..
                } = self
                {
                    *selected_soldier_id = id;
                }
                // *self = Editor::Save { path: self.path, save, selected_soldier_id: id }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let file_controls = view_file_controls(self);

        let editor_panes: Element<_> = match &self {
            Editor::Save {
                save,
                selected_soldier_id,
                ..
            } => row![
                view_soldier_list(save, *selected_soldier_id),
                view_soldier_editor(save.soldiers.get(*selected_soldier_id).unwrap())
            ]
            .into(),
            Editor::NoData => column![text("No soldier selected").size(30)]
                .align_items(Alignment::Center)
                .into(),
        };

        column![file_controls, editor_panes].into()
    }
}

fn view_file_controls(editor: &Editor) -> Element<Message> {
    row![
        button("Open ").padding(10).on_press(Message::OpenFile),
        text(match editor {
            Editor::Save { path, .. } => path.as_os_str().to_str().unwrap_or(""),
            Editor::NoData => "",
        })
        .size(20),
        horizontal_space().width(Length::Fill),
        button("Save").padding(10).on_press_maybe(match editor {
            Editor::Save { .. } => Some(Message::SaveFile),
            Editor::NoData => None,
        })
    ]
    .spacing(20)
    .padding(10)
    .align_items(Alignment::Center)
    .into()
}

fn view_soldier_list(save: &Save, selected_soldier_id: usize) -> Element<Message> {
    keyed_column(save.soldiers.iter().map(|soldier| {
        (
            soldier.id,
            button(text(
                String::from_utf8(soldier.name.clone()).unwrap_or("Malformed name".to_owned()),
            ))
            .on_press(Message::SelectSoldier {
                id: soldier.id as usize,
            })
            .style(if soldier.id as usize == selected_soldier_id {
                Button::Primary
            } else {
                Button::Text
            })
            .into(),
        )
    }))
    .spacing(5)
    .padding(20)
    .align_items(Alignment::End)
    .into()
}

fn view_soldier_editor(soldier: &Soldier) -> Element<Message> {
    column![
        row![column![row![
            text("Name"),
            text_input(
                "Soldier name",
                &String::from_utf8(soldier.name.clone()).unwrap_or("Malformed name".to_owned())
            )
        ],]],
        view_soldier_stats_editor(&soldier.stats),
    ]
    .into()
}

fn view_soldier_stats_editor(stats: &SoldierStats) -> Element<Message> {
    column(Vec::new()).into()
}

fn load_save(filepath: &PathBuf) -> Result<Save, Box<dyn Error>> {
    let file = fs::read(filepath)?;
    let (_, save) = save::parse_save(&file).map_err(|err| err.to_owned())?;
    Result::Ok(save)
}
