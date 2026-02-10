use iced::{
    widget::{
        button, checkbox, column, container, radio, row, scrollable, slider, text, text_input,
        Space,
    },
    Element, Length, Task, Theme,
};

use iced_mouse_layer::mouse_layer;

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(App::title)
        .theme(Theme::Dark)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    ClickedA,
    ClickedB,
    ToggleCheck(bool),
    InputChanged(String),
    SliderChanged(f32),

    SelectGhost(GhostKind),

    OffsetXChanged(f32),
    OffsetYChanged(f32),
    OffsetXTextChanged(String),
    OffsetYTextChanged(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GhostKind {
    Badge,
    MiniButton,
    Pill,
    InfoCard,

    // 신규: “위젯 같은” 고스트들
    MiniScrollPanel,
    MiniTable,
}

impl GhostKind {
    fn label(self) -> &'static str {
        match self {
            GhostKind::Badge => "Badge (text)",
            GhostKind::MiniButton => "Mini button",
            GhostKind::Pill => "Pill row",
            GhostKind::InfoCard => "Info card",
            GhostKind::MiniScrollPanel => "Mini scroll panel",
            GhostKind::MiniTable => "Mini table",
        }
    }
}

struct App {
    checked: bool,
    input: String,
    slider_v: f32,

    ghost_kind: GhostKind,
    offset_x: f32,
    offset_y: f32,
    offset_x_text: String,
    offset_y_text: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            checked: false,
            input: String::new(),
            slider_v: 0.3,

            ghost_kind: GhostKind::Badge,
            offset_x: 15.0,
            offset_y: 15.0,
            offset_x_text: "15".into(),
            offset_y_text: "15".into(),
        }
    }
}

impl App {
    fn title(&self) -> String {
        "MouseLayer testbed".into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ClickedA => {}
            Message::ClickedB => {}
            Message::ToggleCheck(v) => self.checked = v,
            Message::InputChanged(s) => self.input = s,
            Message::SliderChanged(v) => self.slider_v = v,

            Message::SelectGhost(k) => self.ghost_kind = k,

            Message::OffsetXChanged(v) => {
                self.offset_x = v;
                self.offset_x_text = format!("{:.0}", v);
            }
            Message::OffsetYChanged(v) => {
                self.offset_y = v;
                self.offset_y_text = format!("{:.0}", v);
            }
            Message::OffsetXTextChanged(s) => {
                self.offset_x_text = s.clone();
                if let Ok(v) = s.trim().parse::<f32>() {
                    self.offset_x = v.clamp(-200.0, 200.0);
                }
            }
            Message::OffsetYTextChanged(s) => {
                self.offset_y_text = s.clone();
                if let Ok(v) = s.trim().parse::<f32>() {
                    self.offset_y = v.clamp(-200.0, 200.0);
                }
            }
        }

        Task::none()
    }

    // 테이블 행 헬퍼(고스트용)
    fn table_row<'a>(k: &'a str, v: String) -> Element<'a, Message> {
        row![
            container(text(k).size(11))
                .width(Length::Fixed(78.0))
                .padding([2, 6])
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.14, 0.14, 0.16, 0.9
                    ))),
                    border: iced::Border {
                        width: 1.0,
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    text_color: Some(iced::Color::WHITE),
                    ..Default::default()
                }),
            Space::new().width(Length::Fixed(6.0)),
            container(text(v).size(11))
                .width(Length::Fill)
                .padding([2, 6])
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.08, 0.08, 0.10, 0.9
                    ))),
                    border: iced::Border {
                        width: 1.0,
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    text_color: Some(iced::Color::WHITE),
                    ..Default::default()
                }),
        ]
        .align_y(iced::Alignment::Center)
        .into()
    }

    fn view(&self) -> Element<'_, Message> {
        // ---------------- Left panel: lots of widgets to poke
        let demo = column![
            text("Playground").size(18),
            text("Click / type / scroll; ghost should keep following.").size(12),
            Space::new().height(Length::Fixed(8.0)),
            row![
                button("Button A").on_press(Message::ClickedA),
                Space::new().width(Length::Fixed(8.0)),
                button("Button B").on_press(Message::ClickedB),
            ]
            .spacing(0),
            Space::new().height(Length::Fixed(8.0)),
            checkbox(self.checked).on_toggle(Message::ToggleCheck),
            Space::new().height(Length::Fixed(8.0)),
            text_input("Type here…", &self.input).on_input(Message::InputChanged),
            Space::new().height(Length::Fixed(8.0)),
            text(format!("Slider: {:.3}", self.slider_v)),
            slider(0.0..=1.0, self.slider_v, Message::SliderChanged).step(0.001),
            Space::new().height(Length::Fixed(10.0)),
            text("Scrollable content"),
            scrollable(
                column![
                    text("Line 1"), text("Line 2"), text("Line 3"), text("Line 4"),
                    text("Line 5"), text("Line 6"), text("Line 7"), text("Line 8"),
                    text("Line 9"), text("Line 10"), text("Line 11"), text("Line 12"),
                    text("Line 13"), text("Line 14"), text("Line 15"), text("Line 16"),
                    text("Line 17"), text("Line 18"), text("Line 19"), text("Line 20"),
                    text("Line 21"), text("Line 22"), text("Line 23"), text("Line 24"),
                    text("Line 25"), text("Line 26"), text("Line 27"), text("Line 28"),
                ]
                .spacing(6)
                .padding(10)
            )
            .height(Length::Fill),
        ]
        .spacing(6)
        .padding(12);

        let demo_panel = container(demo)
            .width(Length::FillPortion(2))
            .height(Length::Fill);

        // ---------------- Right panel: radio + offset controls
        let radio_list = column![
            text("Ghost widget").size(16),
            radio(GhostKind::Badge.label(), GhostKind::Badge, Some(self.ghost_kind), Message::SelectGhost),
            radio(GhostKind::MiniButton.label(), GhostKind::MiniButton, Some(self.ghost_kind), Message::SelectGhost),
            radio(GhostKind::Pill.label(), GhostKind::Pill, Some(self.ghost_kind), Message::SelectGhost),
            radio(GhostKind::InfoCard.label(), GhostKind::InfoCard, Some(self.ghost_kind), Message::SelectGhost),
            Space::new().height(Length::Fixed(8.0)),
            radio(GhostKind::MiniScrollPanel.label(), GhostKind::MiniScrollPanel, Some(self.ghost_kind), Message::SelectGhost),
            radio(GhostKind::MiniTable.label(), GhostKind::MiniTable, Some(self.ghost_kind), Message::SelectGhost),
        ]
        .spacing(6);

        let offset_controls = column![
            text("Offset").size(16),
            row![
                text("X").width(Length::Fixed(18.0)),
                slider(-200.0..=200.0, self.offset_x, Message::OffsetXChanged).width(Length::Fill),
            ]
            .align_y(iced::Alignment::Center),
            row![
                Space::new().width(Length::Fixed(18.0)),
                text_input("x", &self.offset_x_text)
                    .on_input(Message::OffsetXTextChanged)
                    .width(Length::Fixed(80.0)),
            ]
            .align_y(iced::Alignment::Center),
            Space::new().height(Length::Fixed(10.0)),
            row![
                text("Y").width(Length::Fixed(18.0)),
                slider(-200.0..=200.0, self.offset_y, Message::OffsetYChanged).width(Length::Fill),
            ]
            .align_y(iced::Alignment::Center),
            row![
                Space::new().width(Length::Fixed(18.0)),
                text_input("y", &self.offset_y_text)
                    .on_input(Message::OffsetYTextChanged)
                    .width(Length::Fixed(80.0)),
            ]
            .align_y(iced::Alignment::Center),
            Space::new().height(Length::Fixed(12.0)),
            text("Tip: set offsets negative to place ghost above/left."),
        ]
        .spacing(6);

        let controls = column![text("Controls").size(18), radio_list, offset_controls]
            .spacing(12)
            .padding(12);

        let controls_panel = container(controls)
            .width(Length::FillPortion(1))
            .height(Length::Fill);

        let base = row![demo_panel, controls_panel].spacing(10);

        // ---------------- Ghost widget chosen by radio
        let ghost_content: Element<Message> = match self.ghost_kind {
            GhostKind::Badge => {
                container(text("👻 Ghost").size(14))
                    .padding(8)
                    .style(|_| iced::widget::container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba(
                            0.1, 0.1, 0.1, 0.9,
                        ))),
                        border: iced::Border {
                            width: 1.0,
                            radius: 10.0.into(),
                            ..Default::default()
                        },
                        text_color: Some(iced::Color::WHITE),
                        ..Default::default()
                    })
                    .into()
            }

            GhostKind::MiniButton => button("I follow you").on_press(Message::ClickedA).into(),

            GhostKind::Pill => {
                container(
                    row![
                        text("🧲").size(16),
                        Space::new().width(Length::Fixed(6.0)),
                        text("Pill").size(13),
                    ]
                    .align_y(iced::Alignment::Center),
                )
                .padding(8)
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.2, 0.2, 0.6, 0.85,
                    ))),
                    border: iced::Border {
                        width: 1.0,
                        radius: 999.0.into(),
                        ..Default::default()
                    },
                    text_color: Some(iced::Color::WHITE),
                    ..Default::default()
                })
                .into()
            }

            GhostKind::InfoCard => {
                container(
                    column![
                        text("Info").size(14),
                        text("mouse_layer overlay test").size(11),
                        text(format!("offset=({}, {})", self.offset_x as i32, self.offset_y as i32)).size(11),
                    ]
                    .spacing(4),
                )
                .padding(10)
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.0, 0.0, 0.0, 0.85,
                    ))),
                    border: iced::Border {
                        width: 1.0,
                        radius: 12.0.into(),
                        ..Default::default()
                    },
                    text_color: Some(iced::Color::WHITE),
                    ..Default::default()
                })
                .into()
            }

            GhostKind::MiniScrollPanel => {
                container(
                    column![
                        row![
                            text("🧾").size(16),
                            Space::new().width(Length::Fixed(6.0)),
                            text("Mini Panel").size(13),
                            Space::new().width(Length::Fill),
                            text(format!("{:.0},{:.0}", self.offset_x, self.offset_y)).size(11),
                        ]
                        .align_y(iced::Alignment::Center),
                        Space::new().height(Length::Fixed(6.0)),
                        scrollable(
                            column![
                                text("• event: hover").size(11),
                                text("• event: click").size(11),
                                text("• event: scroll").size(11),
                                text(format!("• checked: {}", self.checked)).size(11),
                                text(format!("• slider: {:.3}", self.slider_v)).size(11),
                                text(format!("• input: {}", if self.input.is_empty() { "(empty)" } else { &self.input })).size(11),
                                text("• status: running").size(11),
                                text("• status: ok").size(11),
                            ]
                            .spacing(4)
                            .padding(2)
                        )
                        .height(Length::Fixed(92.0)),
                    ]
                    .spacing(6),
                )
                .padding(10)
                .width(Length::Fixed(240.0))
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.05, 0.05, 0.07, 0.92,
                    ))),
                    border: iced::Border {
                        width: 1.0,
                        radius: 12.0.into(),
                        ..Default::default()
                    },
                    text_color: Some(iced::Color::WHITE),
                    ..Default::default()
                })
                .into()
            }

            GhostKind::MiniTable => {
                container(
                    column![
                        row![
                            text("📋").size(16),
                            Space::new().width(Length::Fixed(6.0)),
                            text("Mini Table").size(13),
                        ]
                        .align_y(iced::Alignment::Center),
                        Space::new().height(Length::Fixed(8.0)),
                        Self::table_row("Ghost", self.ghost_kind.label().to_string()),
                        Self::table_row("Offset X", format!("{:.0}", self.offset_x)),
                        Self::table_row("Offset Y", format!("{:.0}", self.offset_y)),
                        Self::table_row("Checked", format!("{}", self.checked)),
                        Self::table_row("Slider", format!("{:.3}", self.slider_v)),
                        Self::table_row("Input", if self.input.is_empty() { "(empty)".into() } else { self.input.clone() }),
                    ]
                    .spacing(6),
                )
                .padding(10)
                .width(Length::Fixed(260.0))
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.06, 0.06, 0.08, 0.92,
                    ))),
                    border: iced::Border {
                        width: 1.0,
                        radius: 12.0.into(),
                        ..Default::default()
                    },
                    text_color: Some(iced::Color::WHITE),
                    ..Default::default()
                })
                .into()
            }
        };

        let ghost = mouse_layer(ghost_content).offset(self.offset_x, self.offset_y);

        column![base, ghost].height(Length::Fill).into()
    }
}
