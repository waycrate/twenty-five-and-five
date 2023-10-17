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

use std::fmt;
use std::time::Instant;

/// Set states for setting the time
#[derive(Debug, Clone)]
pub enum PomodoroState {
    WorkTwentyFive(u64),
    BreakFive(u64),
    FifteenRelax(u64),
}

impl fmt::Display for PomodoroState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;
        let seconds = match self {
            PomodoroState::WorkTwentyFive(d)
            | PomodoroState::BreakFive(d)
            | PomodoroState::FifteenRelax(d) => d,
        };

        let duration = format!(
            "{:0>2}:{:0>2}:{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
        );

        let state_str: String = match self {
            PomodoroState::WorkTwentyFive(_) => {
                format!("Work ({})", duration)
            }
            PomodoroState::BreakFive(_) => {
                format!("Break ({})", duration)
            }
            PomodoroState::FifteenRelax(_) => {
                format!("Relax ({})", duration)
            }
        };
        write!(f, "{}", state_str)
    }
}

/// Container to store the value. Maybe this will be removed but let's see haha
#[derive(Debug, Clone)]
pub struct Pomodoro {
    pub mode: PomodoroState,
    pub state: TimeState,
}

#[derive(Debug, Clone)]
pub enum TimeState {
    Idle,
    Ticking { last_tick: Instant },
}

impl Default for Pomodoro {
    fn default() -> Self {
        Self {
            mode: PomodoroState::WorkTwentyFive(1500),
            state: TimeState::Idle,
        }
    }
}
