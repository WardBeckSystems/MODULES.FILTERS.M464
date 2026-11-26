// Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
//! The M464 front panel as drawn on 464-04-003 (issue 2, 10/77): HF CUT switch, HF insert button and LED,
//! LF CUT switch, LF insert button and LED, top to bottom. Positions measured off the drawing in panel units
//! (159 across × 702 down, the WBS badge at the top).

use crate::plugin::Params464;
use nih_plug::prelude::*;
use std::sync::Arc;
use wbs_panel::egui::{self, Frame};
use wbs_panel::nih_plug_egui::{create_egui_editor, EguiState};
use wbs_panel::{ink, Button, Cap, Face, Led, Size};

const LEN: f32 = 702.0;

pub fn default_state() -> Arc<EguiState> {
    EguiState::from_size(214, 893)
}

pub fn create(params: Arc<Params464>) -> Option<Box<dyn Editor>> {
    create_egui_editor(params.editor_state.clone(), (), |_, _| {}, move |ctx, setter, _| {
        egui::CentralPanel::default().frame(Frame::NONE.fill(ink::RACK)).show(ctx, |ui| {
            let f = Face::fit(ui.max_rect().shrink(8.0), LEN);
            f.panel(ui.painter(), "M464");
            draw(ui, &f, setter, &params);
        });
    })
}

/// Eleven legends round a switch, evenly over its 270° travel.
fn legends(l: &'static [&'static str; 11]) -> [(f32, &'static str); 11] {
    std::array::from_fn(|i| (i as f32 / 10.0, l[i]))
}

const HF: [&str; 11] = ["500", "1k", "2k", "3k", "4k", "5k", "6k", "8k", "10k", "12k", "15k"];
const LF: [&str; 11] = ["27", "45", "70", "100", "150", "220", "330", "500", "1k", "2k", "4k"];

fn draw(ui: &mut egui::Ui, f: &Face, s: &ParamSetter, p: &Params464) {
    let at = |across: f32, down: f32| f.at(across, down);

    let (x, y) = at(80.0, 140.0);
    f.knob(ui, s, &p.hf, x, y, Size::Large, Cap::Black, &legends(&HF), 11);
    let (x, y) = at(80.0, 197.0);
    f.text(ui.painter(), x, y, "Hz", 8.5, ink::SUB);
    let (x, y) = at(18.0, 258.0);
    f.text(ui.painter(), x, y, "HF", 11.0, ink::LABEL);
    let (x, y) = at(44.0, 258.0);
    f.push(ui, s, &p.hf_in, x, y, Size::Small, Button::White);
    let (x, y) = at(116.0, 258.0);
    f.led(ui.painter(), x, y, Led::Red, p.hf_in.value());

    let (x, y) = at(80.0, 462.0);
    f.knob(ui, s, &p.lf, x, y, Size::Large, Cap::Black, &legends(&LF), 11);
    let (x, y) = at(80.0, 519.0);
    f.text(ui.painter(), x, y, "Hz", 8.5, ink::SUB);
    let (x, y) = at(18.0, 554.0);
    f.text(ui.painter(), x, y, "LF", 11.0, ink::LABEL);
    let (x, y) = at(44.0, 554.0);
    f.push(ui, s, &p.lf_in, x, y, Size::Small, Button::White);
    let (x, y) = at(116.0, 554.0);
    f.led(ui.painter(), x, y, Led::Red, p.lf_in.value());
}
