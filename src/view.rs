use std::error::Error;
use std::fs;
use std::option::Option::{None, Some};
use std::path::PathBuf;
use std::result::Result::{Err, Ok};

use iced::alignment::{Horizontal, Vertical};
use iced::theme::Button;
use iced::widget::{
    button, column, horizontal_space, keyed_column, pick_list, row, slider, text, text_input,
};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use rfd::{FileDialog, MessageDialog, MessageLevel};

use crate::save::{self, Save};
use crate::soldier::{Gender, Soldier, SoldierStats};

pub fn run() -> iced::Result {
    Editor::run(Settings::default())
}

enum Editor {
    NoData,
    Save {
        path: PathBuf,
        save: Save,
        selected_soldier_id: u32,
    },
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OpenFile,
    SaveFile,
    SelectSoldier { id: u32 },
    GenderSelected(Gender),
    UpdateTimeUnits(u32),
    UpdateTimeUnitsBase(u32),
    UpdateHealth(u32),
    UpdateHealthBase(u32),
    UpdateStrength(u32),
    UpdateStrengthBase(u32),
    UpdateAccuracy(u32),
    UpdateAccuracyBase(u32),
    UpdateRelfexes(u32),
    UpdateReflexBase(u32),
    UpdateBraveryUnits(u32),
    UpdateBraveryBase(u32),
    // UpdateStat {
    //     // update: Box<dyn Fn(&mut SoldierStats, u32)>,
    //     update: F,
    //     val: u32,
    // },
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
                                save.soldiers.get(0).map(|soldier| soldier.id).unwrap_or(0);
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
                    *selected_soldier_id = id as u32;
                }
            }
            // Message::UpdateStat { update, val } => {
            //     if let Editor::Save {
            //         selected_soldier_id,
            //         save,
            //         ..
            //     } = self
            //     {
            //         if let Some(soldier) = save.get_soldier_mut(*selected_soldier_id) {
            //             update(&mut soldier.stats, val);
            //         }
            //     }
            // }
            _ => {}
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
                match save.get_soldier(*selected_soldier_id as u32) {
                    Some(soldier) => view_soldier_editor(soldier),
                    None => text("Select a soldier to edit")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .vertical_alignment(Vertical::Center)
                        .horizontal_alignment(Horizontal::Center)
                        .size(30)
                        .into(),
                }
            ]
            .into(),
            Editor::NoData => text("Open a Xenonauts save file")
                .width(Length::Fill)
                .height(Length::Fill)
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Center)
                .size(30)
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

fn view_soldier_list(save: &Save, selected_soldier_id: u32) -> Element<Message> {
    keyed_column(save.soldiers.iter().map(|soldier| {
        (
            soldier.id,
            button(text(soldier.name.as_str()))
                .on_press(Message::SelectSoldier { id: soldier.id })
                .style(if soldier.id == selected_soldier_id {
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
        row![column![
            row![
                text("Name").size(20),
                horizontal_space().width(Length::Fixed(10.0)),
                text_input("Soldier name", soldier.name.as_str()),
            ],
            row![
                text("Age").size(20),
                horizontal_space().width(Length::Fixed(10.0)),
                text_input("Soldier age", &soldier.age.to_string()),
                horizontal_space().width(Length::Fixed(20.0)),
                text("Gender").size(20),
                horizontal_space().width(Length::Fixed(10.0)),
                pick_list(
                    [Gender::Male, Gender::Female],
                    Some(soldier.gender),
                    Message::GenderSelected
                ),
                horizontal_space().width(Length::Fixed(20.0)),
                text("XP").size(20),
                horizontal_space().width(Length::Fixed(10.0)),
                text_input("Soldier xp", &soldier.xp.to_string()),
            ],
            row![
                text("Nationality").size(20),
                horizontal_space().width(Length::Fixed(10.0)),
                text_input("Soldier nationality", soldier.nationality.as_str()),
            ],
        ]
        .spacing(10)
        .padding(10)],
        view_soldier_stats_editor(&soldier.stats),
    ]
    .into()
}

fn view_soldier_stats_editor(stats: &SoldierStats) -> Element<Message> {
    // let rows = [(
    //     "Time units",
    //     stats.time_units_current,
    //     |stats: &mut SoldierStats, val: u32| stats.time_units_current = val,
    //     stats.time_units_original,
    //     |stats: &mut SoldierStats, val: u32| stats.time_units_current = val,
    // )];
    let rows = [(
        "Time units",
        stats.time_units_current,
        Message::UpdateTimeUnits,
        stats.time_units_original,
        Message::UpdateTimeUnitsBase,
    )];

    column(
        // TODO checks bounds in game
        rows.into_iter()
            .map(
                |(stat_name, current, update_current, original, update_original)| {
                    row![
                        text(stat_name).size(20),
                        horizontal_space().width(Length::Fixed(10.0)),
                        text(current).size(20),
                        horizontal_space().width(Length::Fixed(10.0)),
                        slider(1..=100, current, update_current),
                        // slider(1..=100, *current, |val: u32| {
                        //     Message::UpdateStat {
                        //         // update: |stats, val| stats.time_units_current = val,
                        //         // update: Box::new(update_current),
                        //         update: update_current,
                        //         val,
                        //     }
                        // }),
                        horizontal_space().width(Length::Fixed(20.0)),
                        text("Base value").size(20),
                        horizontal_space().width(Length::Fixed(10.0)),
                        text_input("Stat base value", &original.to_string()),
                    ]
                    .into()
                },
            )
            .collect::<Vec<_>>(),
    )
    .into()
}

fn load_save(filepath: &PathBuf) -> Result<Save, Box<dyn Error>> {
    let file = fs::read(filepath)?;
    let (_, save) = save::parse_save(&file).map_err(|err| err.to_owned())?;
    Result::Ok(save)
}
