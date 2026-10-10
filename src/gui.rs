use crate::{audio_player, workout_plan::WorkoutPlan};
use eframe::egui;
use std::time::{Duration, Instant};

const MAX_DURATION_SECONDS: u32 = 3600;
const MAX_REPEATS: u32 = 20;
const MAX_SETS: u32 = 20;
const ACCENT: egui::Color32 = egui::Color32::from_rgb(245, 142, 68);
const BACKGROUND: egui::Color32 = egui::Color32::from_rgb(62, 59, 56);
const MUTED_TEXT: egui::Color32 = egui::Color32::from_rgb(190, 173, 158);

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
        ui.label(
            egui::RichText::new("WORKOUT PLAN")
                .size(11.0)
                .strong()
                .color(MUTED_TEXT),
        );
        ui.add_space(12.0);
        egui::Grid::new("workout_settings")
            .num_columns(2)
            .spacing([18.0, 13.0])
            .show(ui, |ui| {
                ui.label("Hang");
                ui.add(
                    egui::DragValue::new(&mut self.hang_time)
                        .range(1..=MAX_DURATION_SECONDS)
                        .suffix(" sec"),
                );
                ui.end_row();

                ui.label("Rest");
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

                ui.label("Set rest");
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
        set_theme(ctx);

        let mut start_requested = false;
        let mut reset_requested = false;
        let is_running = self.is_running();

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BACKGROUND)
                    .inner_margin(egui::Margin::ZERO)
                    .stroke(egui::Stroke::NONE),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(18.0);
                    egui::Frame::new()
                        .stroke(egui::Stroke::new(
                            1.0_f32,
                            egui::Color32::from_rgba_unmultiplied(255, 191, 130, 100),
                        ))
                        .corner_radius(egui::CornerRadius::same(18))
                        .inner_margin(egui::Margin::same(14))
                        .show(ui, |ui| {
                            ui.set_width(280.0);
                            ui.vertical_centered(|ui| {
                                if is_running {
                                    self.show_running_session(ui);
                                } else {
                                    if self.phase == Phase::Complete {
                                        ui.label(
                                            egui::RichText::new(self.phase_label())
                                                .size(22.0)
                                                .strong(),
                                        );
                                        ui.add_space(18.0);
                                    }
                                    self.show_settings(ui);
                                }

                                ui.add_space(20.0);
                                if !is_running {
                                    let button_text = if self.phase == Phase::Complete {
                                        "Start another workout"
                                    } else {
                                        "Start workout"
                                    };
                                    start_requested = ui
                                        .add_sized(
                                            [240.0, 46.0],
                                            egui::Button::new(
                                                egui::RichText::new(button_text)
                                                    .strong()
                                                    .color(egui::Color32::from_rgb(38, 28, 22)),
                                            )
                                            .fill(ACCENT)
                                            .corner_radius(egui::CornerRadius::same(12)),
                                        )
                                        .clicked();
                                } else {
                                    reset_requested = ui
                                        .add_sized(
                                            [240.0, 42.0],
                                            egui::Button::new("Stop workout")
                                                .fill(egui::Color32::from_rgb(77, 51, 43))
                                                .stroke(egui::Stroke::new(1.0_f32, ACCENT))
                                                .corner_radius(egui::CornerRadius::same(12)),
                                        )
                                        .clicked();
                                }
                            });
                        });
                });
            });

        if reset_requested {
            self.reset();
        } else if start_requested {
            self.start();
        }
    }
}

impl HangboardApp {
    fn show_running_session(&self, ui: &mut egui::Ui) {
        ui.label(
            egui::RichText::new(self.phase_label())
                .size(24.0)
                .strong()
                .color(ACCENT),
        );
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new(format!(
                "SET {} OF {}   ·   HANG {} OF {}",
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
            ))
            .size(11.0)
            .strong()
            .color(MUTED_TEXT),
        );
        ui.add_space(12.0);

        let seconds = self.remaining_seconds();
        ui.label(
            egui::RichText::new(seconds.to_string())
                .size(96.0)
                .strong()
                .color(egui::Color32::from_rgb(255, 232, 206)),
        );
        ui.label(
            egui::RichText::new("SECONDS")
                .size(10.0)
                .strong()
                .color(MUTED_TEXT),
        );

        if self.phase_duration > 0 {
            let progress = (1.0 - seconds as f32 / self.phase_duration as f32).clamp(0.0, 1.0);
            ui.add_space(12.0);
            let response = ui.add(
                egui::ProgressBar::new(progress)
                    .desired_width(240.0)
                    .desired_height(24.0)
                    .fill(ACCENT)
                    .corner_radius(egui::CornerRadius::same(6)),
            );
            let percentage = format!("{}%", (progress * 100.0) as u32);
            let painter = ui.painter();
            let label_rect =
                egui::Rect::from_center_size(response.rect.center(), egui::vec2(42.0, 18.0));
            painter.rect_filled(
                label_rect,
                egui::CornerRadius::same(5),
                egui::Color32::from_black_alpha(150),
            );
            painter.text(
                label_rect.center(),
                egui::Align2::CENTER_CENTER,
                percentage,
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );
        }
    }
}

fn set_theme(ctx: &egui::Context) {
    ctx.set_pixels_per_point(16.0 / 12.5);
    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(egui::Color32::from_rgb(248, 238, 226));
    visuals.window_fill = BACKGROUND;
    visuals.panel_fill = BACKGROUND;
    visuals.selection.bg_fill = ACCENT;
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(74, 58, 48);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(248, 238, 226);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(108, 72, 50);
    visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
    visuals.widgets.active.bg_fill = ACCENT;
    visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;
    ctx.set_visuals(visuals);
}

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 480.0])
            .with_min_inner_size([320.0, 340.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hangboard Timer",
        options,
        Box::new(|_creation_context| Ok(Box::<HangboardApp>::default())),
    )
}
