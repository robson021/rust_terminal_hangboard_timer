use crate::{audio_player, workout_plan::WorkoutPlan};
use eframe::egui;
use std::time::{Duration, Instant};

const MAX_DURATION_SECONDS: u32 = 3600;
const MAX_REPEATS: u32 = 20;
const MAX_SETS: u32 = 20;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Idle,
    GetReady,
    Hang,
    Rest,
    SetRest,
    Complete,
}

struct HangboardApp {
    hang_time: u32,
    rest_time: u32,
    number_of_hang_repeats: u32,
    rest_time_between_sets: u32,
    number_of_sets: u32,
    plan: Option<WorkoutPlan>,
    phase: Phase,
    phase_ends_at: Option<Instant>,
    phase_duration: u32,
    ready_cue_played: bool,
    current_set: u32,
    current_rep: u32,
}

impl Default for HangboardApp {
    fn default() -> Self {
        Self {
            hang_time: 7,
            rest_time: 3,
            number_of_hang_repeats: 6,
            rest_time_between_sets: 120,
            number_of_sets: 3,
            plan: None,
            phase: Phase::Idle,
            phase_ends_at: None,
            phase_duration: 0,
            ready_cue_played: false,
            current_set: 1,
            current_rep: 1,
        }
    }
}

impl HangboardApp {
    fn is_running(&self) -> bool {
        !matches!(self.phase, Phase::Idle | Phase::Complete)
    }

    fn start(&mut self) {
        let plan = WorkoutPlan {
            hang_time: self.hang_time,
            rest_time: self.rest_time,
            number_of_hang_repeats: self.number_of_hang_repeats,
            rest_time_between_sets: self.rest_time_between_sets,
            number_of_sets: self.number_of_sets,
        };
        self.current_set = 1;
        self.current_rep = 1;
        self.plan = Some(plan);
        self.set_phase(Phase::GetReady, 5, Instant::now());
    }

    fn reset(&mut self) {
        self.plan = None;
        self.phase = Phase::Idle;
        self.phase_ends_at = None;
        self.phase_duration = 0;
        self.ready_cue_played = false;
        self.current_set = 1;
        self.current_rep = 1;
    }

    fn set_phase(&mut self, phase: Phase, duration: u32, starts_at: Instant) {
        self.phase = phase;
        self.phase_duration = duration;
        self.phase_ends_at = Some(starts_at + Duration::from_secs(u64::from(duration)));
        self.ready_cue_played = false;
    }

    fn start_hang(&mut self, starts_at: Instant) {
        let hang_time = self.plan.as_ref().expect("workout plan is set").hang_time;
        audio_player::ding();
        self.set_phase(Phase::Hang, hang_time, starts_at);
    }

    fn advance(&mut self) {
        let now = Instant::now();

        if matches!(self.phase, Phase::Rest | Phase::SetRest)
            && self.phase_duration > 15
            && !self.ready_cue_played
            && self.phase_ends_at.is_some_and(|deadline| {
                deadline.saturating_duration_since(now) <= Duration::from_secs(10)
            })
        {
            audio_player::get_ready();
            self.ready_cue_played = true;
        }

        for _ in 0..1000 {
            let Some(deadline) = self.phase_ends_at.filter(|deadline| *deadline <= now) else {
                break;
            };
            self.advance_phase(deadline);
        }
    }

    fn advance_phase(&mut self, transition_at: Instant) {
        match self.phase {
            Phase::GetReady => self.start_hang(transition_at),
            Phase::Hang => {
                let plan = self.plan.as_ref().expect("workout plan is set");
                if self.current_rep < plan.number_of_hang_repeats {
                    audio_player::bell();
                    self.set_phase(Phase::Rest, plan.rest_time, transition_at);
                } else {
                    audio_player::end_of_round();
                    if self.current_set < plan.number_of_sets {
                        self.set_phase(Phase::SetRest, plan.rest_time_between_sets, transition_at);
                    } else {
                        audio_player::finish();
                        self.phase = Phase::Complete;
                        self.phase_ends_at = None;
                    }
                }
            }
            Phase::Rest => {
                self.current_rep += 1;
                self.start_hang(transition_at);
            }
            Phase::SetRest => {
                self.current_set += 1;
                self.current_rep = 1;
                self.start_hang(transition_at);
            }
            Phase::Idle | Phase::Complete => {
                self.phase_ends_at = None;
            }
        }
    }

    fn phase_label(&self) -> &'static str {
        match self.phase {
            Phase::Idle => "Ready when you are",
            Phase::GetReady => "Get ready",
            Phase::Hang => "Hang",
            Phase::Rest => "Rest",
            Phase::SetRest => "Rest between sets",
            Phase::Complete => "Workout complete",
        }
    }

    fn remaining_seconds(&self) -> u64 {
        self.phase_ends_at
            .map(|deadline| {
                deadline
                    .saturating_duration_since(Instant::now())
                    .as_secs_f64()
                    .ceil() as u64
            })
            .unwrap_or(0)
    }

    fn show_settings(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("workout_settings")
            .num_columns(2)
            .spacing([24.0, 12.0])
            .show(ui, |ui| {
                ui.label("Hang time");
                ui.add(
                    egui::DragValue::new(&mut self.hang_time)
                        .range(1..=MAX_DURATION_SECONDS)
                        .suffix(" sec"),
                );
                ui.end_row();

                ui.label("Rest between hangs");
                ui.add(
                    egui::DragValue::new(&mut self.rest_time)
                        .range(1..=MAX_DURATION_SECONDS)
                        .suffix(" sec"),
                );
                ui.end_row();

                ui.label("Hangs per set");
                ui.add(
                    egui::DragValue::new(&mut self.number_of_hang_repeats).range(1..=MAX_REPEATS),
                );
                ui.end_row();

                ui.label("Rest between sets");
                ui.add(
                    egui::DragValue::new(&mut self.rest_time_between_sets)
                        .range(1..=MAX_DURATION_SECONDS)
                        .suffix(" sec"),
                );
                ui.end_row();

                ui.label("Number of sets");
                ui.add(egui::DragValue::new(&mut self.number_of_sets).range(1..=MAX_SETS));
                ui.end_row();
            });
    }
}

impl eframe::App for HangboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.advance();
        ctx.request_repaint_after(Duration::from_millis(100));

        let mut start_requested = false;
        let mut reset_requested = false;
        let is_running = self.is_running();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(16.0);
                ui.heading("Hangboard timer");
                ui.add_space(24.0);

                if is_running {
                    ui.label(self.phase_label());
                    ui.add_space(4.0);
                    ui.label(format!(
                        "Set {} of {}  |  Hang {} of {}",
                        self.current_set,
                        self.plan
                            .as_ref()
                            .expect("workout plan is set")
                            .number_of_sets,
                        self.current_rep,
                        self.plan
                            .as_ref()
                            .expect("workout plan is set")
                            .number_of_hang_repeats
                    ));
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(self.remaining_seconds().to_string()).size(96.0));
                    ui.label("seconds");
                } else if self.phase == Phase::Complete {
                    ui.label(egui::RichText::new(self.phase_label()).size(28.0));
                } else {
                    ui.label(self.phase_label());
                }

                ui.add_space(24.0);
                if !is_running {
                    self.show_settings(ui);
                    ui.add_space(24.0);
                    let button_text = if self.phase == Phase::Complete {
                        "Start another workout"
                    } else {
                        "Start workout"
                    };
                    start_requested = ui
                        .add_sized([220.0, 44.0], egui::Button::new(button_text))
                        .clicked();
                } else {
                    reset_requested = ui
                        .add_sized([220.0, 40.0], egui::Button::new("Stop workout"))
                        .clicked();
                }
            });
        });

        if reset_requested {
            self.reset();
        } else if start_requested {
            self.start();
        }
    }
}

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 600.0])
            .with_min_inner_size([380.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hangboard Timer",
        options,
        Box::new(|_creation_context| Ok(Box::<HangboardApp>::default())),
    )
}
