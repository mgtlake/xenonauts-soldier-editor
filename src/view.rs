use std::error::Error;
use std::fs;
use std::option::Option::{None, Some};
use std::path::PathBuf;
use std::result::Result::{Err, Ok};
use std::u16::MAX;

use iced::alignment::{Horizontal, Vertical};
use iced::theme::Button;
use iced::widget::{
    button, column, horizontal_space, keyed_column, pick_list, row, scrollable, slider, text,
    text_input,
};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use iced_aw::{number_input, BOOTSTRAP_FONT};
use rfd::{FileDialog, MessageDialog, MessageLevel};

use crate::save::{self, Save};
use crate::soldier::{Gender, Soldier, SoldierStats};

pub fn run() -> iced::Result {
    let mut settings: Settings<()> = Settings::default();
    settings.fonts.push(std::borrow::Cow::Owned(
        iced_aw::BOOTSTRAP_FONT_BYTES.to_vec(),
    ));
    Editor::run(settings)
}

enum Editor {
    NoData,
    Save {
        path: PathBuf,
        save: Save,
        selected_soldier_id: u32,
    },
}

#[derive(Debug, Clone)]
enum Message {
    OpenFile,
    SaveFile,
    SelectSoldier { id: u32 },
    UpdateName(String),
    UpdateNationality(String),
    UpdateRace(String),
    UpdateRegiment(String),
    UpdateExperience(String),
    UpdateFlag(String),
    GenderSelected(Gender),
    UpdateAge(f32),
    UpdateXP(u32),
    UpdateTimeUnits(u32),
    UpdateTimeUnitsBase(u32),
    UpdateHealth(u32),
    UpdateHealthBase(u32),
    UpdateStrength(u32),
    UpdateStrengthBase(u32),
    UpdateAccuracy(u32),
    UpdateAccuracyBase(u32),
    UpdateReflexes(u32),
    UpdateReflexesBase(u32),
    UpdateBravery(u32),
    UpdateBraveryBase(u32),
    UpdateFaceNumber(u32),
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
        if let Message::OpenFile = message {
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

        if let Editor::Save {
            path,
            save,
            selected_soldier_id,
        } = self
        {
            if let Message::SaveFile = message {
                if let Err(e) = fs::write(path, save.serialise()) {
                    MessageDialog::new()
                        .set_level(MessageLevel::Error)
                        .set_title("Could not write updated save file!")
                        .set_description(format!("{:#?}", e))
                        .show();
                }
            }
            if let Message::SelectSoldier { id } = message {
                *selected_soldier_id = id as u32;
            }

            if let Some(soldier) = save.get_soldier_mut(*selected_soldier_id) {
                match message {
                    Message::UpdateName(name) => {
                        soldier.name = name;
                    }
                    Message::UpdateNationality(nationality) => {
                        soldier.nationality = nationality;
                    }
                    Message::UpdateRace(race) => {
                        soldier.race = race.clone().into_bytes();
                    }
                    Message::UpdateRegiment(regiment) => {
                        soldier.regiment = regiment.clone().into_bytes();
                    }
                    Message::UpdateExperience(experience) => {
                        soldier.experience = experience.clone().into_bytes();
                    }
                    Message::UpdateFlag(flag) => {
                        soldier.nation = flag.clone().into_bytes();
                    }
                    Message::GenderSelected(gender) => {
                        soldier.gender = gender;
                    }
                    Message::UpdateAge(val) => {
                        soldier.age = val;
                    }
                    Message::UpdateXP(val) => {
                        soldier.xp = val;
                    }
                    Message::UpdateTimeUnits(val) => {
                        if val < soldier.stats.time_units_original {
                            return;
                        }
                        soldier.stats.time_units_current = val;
                    }
                    Message::UpdateTimeUnitsBase(val) => {
                        soldier.stats.time_units_original = val;
                    }
                    Message::UpdateHealth(val) => {
                        if val < soldier.stats.health_original {
                            return;
                        }
                        soldier.stats.health_current = val;
                    }
                    Message::UpdateHealthBase(val) => {
                        soldier.stats.health_original = val;
                    }
                    Message::UpdateStrength(val) => {
                        if val < soldier.stats.strength_original {
                            return;
                        }
                        soldier.stats.strength_current = val;
                    }
                    Message::UpdateStrengthBase(val) => {
                        soldier.stats.strength_original = val;
                    }
                    Message::UpdateAccuracy(val) => {
                        if val < soldier.stats.accuracy_original {
                            return;
                        }
                        soldier.stats.accuracy_current = val;
                    }
                    Message::UpdateAccuracyBase(val) => {
                        soldier.stats.accuracy_original = val;
                    }
                    Message::UpdateReflexes(val) => {
                        if val < soldier.stats.reflexes_original {
                            return;
                        }
                        soldier.stats.reflexes_current = val;
                    }
                    Message::UpdateReflexesBase(val) => {
                        soldier.stats.reflexes_original = val;
                    }
                    Message::UpdateBravery(val) => {
                        if val < soldier.stats.bravery_original {
                            return;
                        }
                        soldier.stats.bravery_current = val;
                    }
                    Message::UpdateBraveryBase(val) => {
                        soldier.stats.bravery_original = val;
                    }
                    Message::UpdateFaceNumber(val) => {
                        soldier.face_number = val;
                    }
                    _ => {}
                }
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
        button(row![icon('\u{F3D8}'), "Open"].spacing(5))
            .padding(10)
            .on_press(Message::OpenFile),
        text(match editor {
            Editor::Save { path, .. } => path.as_os_str().to_str().unwrap_or(""),
            Editor::NoData => "",
        })
        .size(20),
        horizontal_space().width(Length::Fill),
        button(row![icon('\u{F7D8}'), "Save"].spacing(5))
            .padding(10)
            .on_press_maybe(match editor {
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
    scrollable(
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
        .align_items(Alignment::End),
    )
    .into()
}

fn view_soldier_editor(soldier: &Soldier) -> Element<Message> {
    column![
        row![
            column![
                row![
                    text("Name").size(20),
                    horizontal_space().width(Length::Fixed(10.0)),
                    text_input("Soldier name", soldier.name.as_str()).on_input(Message::UpdateName),
                ],
                row![
                    text("Age").size(20),
                    horizontal_space().width(Length::Fixed(10.0)),
                    number_input(soldier.age, f32::MAX, Message::UpdateAge).min(0.0).step(1.0),
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
                    number_input(soldier.xp, u32::MAX, Message::UpdateXP).min(0),
                ],
                row![
                    text("Nationality").size(20),
                    horizontal_space().width(Length::Fixed(10.0)),
                    text_input("Soldier nationality", soldier.nationality.as_str())
                        .on_input(Message::UpdateNationality),
                    horizontal_space().width(Length::Fixed(20.0)),
                    text("Flag").size(20),
                    horizontal_space().width(Length::Fixed(10.0)),
                    text_input("Soldier flag", &String::from_utf8(soldier.nation.clone()).unwrap())
                        .on_input(Message::UpdateFlag),
                ],
            ]
            .spacing(10),
            column![
                row![
                    text("Race").size(20),
                    horizontal_space().width(Length::Fixed(10.0)),
                    text_input(
                        "Soldier race",
                        &String::from_utf8(soldier.race.clone()).unwrap()
                    )
                    .width(50)
                    .on_input(Message::UpdateRace),
                    horizontal_space().width(Length::Fixed(20.0)),
                    text("Face").size(20),
                    horizontal_space().width(Length::Fixed(10.0)),
                    number_input(soldier.face_number, u32::MAX, Message::UpdateFaceNumber).min(0),
                ],
                row![
                    text("Regiment").size(20),
                    horizontal_space().width(Length::Fixed(10.0)),
                    text_input(
                        "Soldier regiment",
                        &String::from_utf8(soldier.regiment.clone()).unwrap()
                    )
                    .width(150)
                    .on_input(Message::UpdateRegiment),
                ],
                row![
                    text("Experience").size(20),
                    horizontal_space().width(Length::Fixed(10.0)),
                    text_input(
                        "Soldier experience",
                        &String::from_utf8(soldier.experience.clone()).unwrap()
                    )
                    .width(150)
                    .on_input(Message::UpdateExperience),
                ],
            ].spacing(10)
        ].spacing(20),
        view_soldier_stats_editor(&soldier.stats),
    ]
    .spacing(20)
    .padding(10)
    .into()
}

fn view_soldier_stats_editor(stats: &SoldierStats) -> Element<Message> {
    column![
        view_soldier_stats_editor_row(
            "Time units",
            stats.time_units_current,
            Message::UpdateTimeUnits,
            stats.time_units_original,
            Message::UpdateTimeUnitsBase,
        ),
        view_soldier_stats_editor_row(
            "Health",
            stats.health_current,
            Message::UpdateHealth,
            stats.health_original,
            Message::UpdateHealthBase,
        ),
        view_soldier_stats_editor_row(
            "Strength",
            stats.strength_current,
            Message::UpdateStrength,
            stats.strength_original,
            Message::UpdateStrengthBase,
        ),
        view_soldier_stats_editor_row(
            "Accuracy",
            stats.accuracy_current,
            Message::UpdateAccuracy,
            stats.accuracy_original,
            Message::UpdateAccuracyBase,
        ),
        view_soldier_stats_editor_row(
            "Reflexes",
            stats.reflexes_current,
            Message::UpdateReflexes,
            stats.accuracy_original,
            Message::UpdateReflexesBase,
        ),
        view_soldier_stats_editor_row(
            "Bravery",
            stats.bravery_current,
            Message::UpdateBravery,
            stats.bravery_original,
            Message::UpdateBraveryBase,
        ),
    ]
    .spacing(10)
    .into()
}

fn view_soldier_stats_editor_row(
    stat_name: &str,
    current: u32,
    update_current: fn(u32) -> Message,
    base: u32,
    update_base: fn(u32) -> Message,
) -> Element<Message> {
    row![
        text(stat_name).size(20),
        horizontal_space().width(Length::Fixed(10.0)),
        text(current)
            .size(20)
            .width(Length::Fixed(30.0))
            .horizontal_alignment(Horizontal::Center),
        horizontal_space().width(Length::Fixed(10.0)),
        slider(1..=100, current, update_current),
        horizontal_space().width(Length::Fixed(20.0)),
        text("Base value").size(20),
        horizontal_space().width(Length::Fixed(10.0)),
        number_input(base, current, update_base).min(0),
    ]
    .into()
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    text(codepoint).font(BOOTSTRAP_FONT).into()
}

fn load_save(filepath: &PathBuf) -> Result<Save, Box<dyn Error>> {
    let file = fs::read(filepath)?;
    let (_, save) = save::parse_save(&file).map_err(|err| err.to_owned())?;
    Result::Ok(save)
}
