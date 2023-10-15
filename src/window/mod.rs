// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![deny(warnings)]
#![warn(unused_extern_crates)]
// Enable some groups of clippy lints.
#![deny(clippy::suspicious)]
#![deny(clippy::perf)]
// Specific lints to enforce.
#![warn(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::disallowed_types)]
#![deny(clippy::manual_let_else)]
#![allow(clippy::unreachable)]

use crate::timer::{Pomodoro, PomodoroState, TimeState};

use notify_rust::Notification;

use iced::alignment;
use iced::executor;
use iced::keyboard;
use iced::theme;
use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::Event::Keyboard;
use iced::{Alignment, Application, Command, Element, Length, Subscription, Theme};
pub use tracing::{self, Level};

use std::time::Duration;
use std::time::Instant;

const APPLICATION_ID: &str = "org.uncomfy.twentyfiveandfive";

#[derive(Debug, Clone)]
pub enum Message {
    Break,
    Relax,
    // Reset time of PomodoroState
    Reset,
    Tick(Instant),
    // Pause and Start
    Toggle,
    Work,
}

impl Application for Pomodoro {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Pomodoro, Command<Message>) {
        (
            Pomodoro {
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Twenty Five and Five — {}", self.mode)
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                TimeState::Idle => {
                    self.state = TimeState::Ticking {
                        last_tick: Instant::now(),
                    };
                }
                TimeState::Ticking { .. } => {
                    self.state = TimeState::Idle;
                }
            },
            Message::Reset => {
                *self = Self {
                    ..Default::default()
                };
            }
            Message::Tick(now) => {
                tracing::warn!(?now);
                if let TimeState::Ticking { last_tick } = self.state {
                    match self.mode {
                        PomodoroState::WorkTwentyFive(d) => {
                            if !d.is_zero() {
                                tracing::warn!("Seconds left: {}", d.as_secs());
                                self.mode =
                                    PomodoroState::WorkTwentyFive(d - Duration::from_secs(1));
                            } else {
                                self.mode =
                                    PomodoroState::BreakFive(Duration::from_secs_f64(300.0));
                                self.state = TimeState::Idle;
                                Notification::new()
                                    .summary(format!("{}", self.mode).as_str())
                                    .body("Work Done 🫂")
                                    .icon("alarm-clock")
                                    .show()
                                    .expect("Failed to notify");
                            };
                        }
                        PomodoroState::BreakFive(d) => {
                            tracing::warn!("Seconds left: {}", d.as_secs());
                            if !d.is_zero() {
                                self.mode = PomodoroState::BreakFive(d - Duration::from_secs(1));
                            } else {
                                self.mode =
                                    PomodoroState::WorkTwentyFive(Duration::from_secs_f64(300.0));
                                self.state = TimeState::Idle;
                                Notification::new()
                                    .summary(format!("{}", self.mode).as_str())
                                    .body("Break Done 🥖")
                                    .icon("alarm-clock")
                                    .show()
                                    .expect("Failed to notify");
                            };
                        }
                        PomodoroState::FifteenRelax(d) => {
                            tracing::warn!("Seconds left: {}", d.as_secs());
                            if !d.is_zero() {
                                self.mode = PomodoroState::FifteenRelax(d - Duration::from_secs(1));
                            } else {
                                self.mode =
                                    PomodoroState::WorkTwentyFive(Duration::from_secs_f64(300.0));
                                self.state = TimeState::Idle;
                                Notification::new()
                                    .summary(format!("{}", self.mode).as_str())
                                    .body("Relaxation Done 🔫")
                                    .icon("alarm-clock")
                                    .show()
                                    .expect("Failed to notify");
                            };
                        }
                    };
                    // self.duration += now - *last_tick;
                    tracing::warn!(?last_tick);
                }
            }
            Message::Work => {
                self.mode = PomodoroState::WorkTwentyFive(Duration::from_secs_f64(1500.0));
            }
            Message::Break => {
                self.mode = PomodoroState::BreakFive(Duration::from_secs_f64(300.0));
            }
            Message::Relax => {
                self.mode = PomodoroState::FifteenRelax(Duration::from_secs_f64(900.0));
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

        let seconds = match self.mode {
            PomodoroState::WorkTwentyFive(d)
            | PomodoroState::BreakFive(d)
            | PomodoroState::FifteenRelax(d) => d.as_secs(),
        };

        let duration = text(format!(
            "{:0>2}:{:0>2}",
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
        ))
        .size(50);

        let button = |label| {
            button(text(label).horizontal_alignment(alignment::Horizontal::Center))
                .padding(10)
                .width(80)
        };

        let toggle_button = {
            let label = match self.state {
                TimeState::Idle => "Start",
                TimeState::Ticking { .. } => "Stop",
            };

            button(label).on_press(Message::Toggle)
        };

        let reset_button = button("Reset")
            .style(theme::Button::Destructive)
            .on_press(Message::Reset);

        let work_button = button("Work")
            .style(theme::Button::Destructive)
            .on_press(Message::Work);

        let break_button = button("Break")
            .style(theme::Button::Positive)
            .on_press(Message::Break);

        let relax_button = button("Relax!")
            .style(theme::Button::Secondary)
            .on_press(Message::Relax);

        let controls = row![toggle_button, reset_button].spacing(20);

        let mode_controls = row![work_button, break_button, relax_button,];

        let content = column![duration, controls, mode_controls]
            .align_items(Alignment::Center)
            .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::default()
    }

    fn style(&self) -> theme::Application {
        theme::Application::default()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let tick = match self.state {
            TimeState::Idle => Subscription::none(),
            TimeState::Ticking { .. } => time::every(Duration::from_secs(1)).map(Message::Tick),
        };

        let handle = iced::subscription::events_with(|event, status| match (event, status) {
            (Keyboard(ev), iced::event::Status::Ignored) => {
                if let keyboard::Event::KeyReleased {
                    key_code,
                    modifiers: _,
                } = ev
                {
                    match key_code {
                        keyboard::KeyCode::Space => Some(Message::Toggle),
                        keyboard::KeyCode::R => Some(Message::Reset),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        });
        tracing::warn!(?handle);

        Subscription::batch(vec![tick, handle])
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}

#[cfg(target_os = "linux")]
pub fn settings() -> iced::window::Settings {
    iced::window::Settings {
        platform_specific: iced::window::PlatformSpecific {
            application_id: APPLICATION_ID.to_string(),
        },
        size: (540, 540),
        max_size: Some((720, 720)),
        min_size: Some((300, 300)),
        ..Default::default()
    }
}
