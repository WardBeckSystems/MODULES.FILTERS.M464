// Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
//! M464 High Pass / Low Pass Filter (1976|9) signal path, free of any plugin framework (the Audio Tap uses this alone).
//!
//! From the M464 technical data and front panel 464-04-003: unity gain; LOW FREQUENCY CUT (high pass, SW1) and
//! HIGH FREQUENCY CUT (low pass, SW2), each one of eleven discrete frequencies calibrated at the 3 dB corner,
//! 18 dB/octave; each filter is inserted by its own push-button (SW3 LF, SW4 HF) and lights its LED.
//! The signal passes the LF cut first, then the HF cut.

use crate::filter::{Cut, Kind, MAX_CH};

/// SW1, LOW FREQUENCY CUT positions (Hz), as engraved on the panel.
pub const LF_HZ: [f32; 11] = [27.0, 45.0, 70.0, 100.0, 150.0, 220.0, 330.0, 500.0, 1000.0, 2000.0, 4000.0];
/// SW2, HIGH FREQUENCY CUT positions (Hz).
pub const HF_HZ: [f32; 11] = [500.0, 1000.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 8000.0, 10000.0, 12000.0, 15000.0];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// SW3 LF CUT insert
    pub lf_in: bool,
    /// SW1 position 0 … 10
    pub lf: usize,
    /// SW4 HF CUT insert
    pub hf_in: bool,
    /// SW2 position 0 … 10
    pub hf: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { lf_in: false, lf: 3, hf_in: false, hf: 8 }
    }
}

pub struct M464 {
    rate: f32,
    s: Settings,
    lf: Cut,
    hf: Cut,
}

impl M464 {
    pub fn new(sample_rate: f32) -> Self {
        let mut m = M464 { rate: sample_rate, s: Settings::default(), lf: Cut::default(), hf: Cut::default() };
        m.update();
        m
    }
    pub fn set_sample_rate(&mut self, r: f32) {
        self.rate = r;
        self.reset();
        self.update();
    }
    pub fn reset(&mut self) {
        self.lf.reset();
        self.hf.reset();
    }
    pub fn settings(&self) -> Settings {
        self.s
    }
    pub fn set(&mut self, s: Settings) {
        let s = Settings { lf: s.lf.min(10), hf: s.hf.min(10), ..s };
        if s != self.s {
            // a filter switched in starts from rest, as the hardware's does
            if s.lf_in && !self.s.lf_in {
                self.lf.reset();
            }
            if s.hf_in && !self.s.hf_in {
                self.hf.reset();
            }
            self.s = s;
            self.update();
        }
    }
    fn update(&mut self) {
        self.lf.set(Kind::HighPass, LF_HZ[self.s.lf], self.rate);
        self.hf.set(Kind::LowPass, HF_HZ[self.s.hf], self.rate);
    }
    pub fn process(&mut self, chans: &mut [&mut [f32]]) {
        let Settings { lf_in, hf_in, .. } = self.s;
        if !lf_in && !hf_in {
            return;
        }
        for (ch, data) in chans.iter_mut().enumerate().take(MAX_CH) {
            for v in data.iter_mut() {
                let mut x = *v;
                if lf_in {
                    x = self.lf.run(ch, x);
                }
                if hf_in {
                    x = self.hf.run(ch, x);
                }
                *v = x;
            }
        }
        self.lf.ok();
        self.hf.ok();
    }
    /// The overall response in dB at `hz` with the current settings.
    pub fn response_db(&self, hz: f32) -> f32 {
        (if self.s.lf_in { self.lf.response_db(hz, self.rate) } else { 0.0 })
            + (if self.s.hf_in { self.hf.response_db(hz, self.rate) } else { 0.0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_is_straight_through() {
        let mut m = M464::new(48000.0);
        let mut a = [0.5f32, -0.25, 0.125];
        m.process(&mut [&mut a[..]]);
        assert_eq!(a, [0.5, -0.25, 0.125]);
    }

    #[test]
    fn every_switch_position_is_its_3_db_point() {
        let mut m = M464::new(48000.0);
        for i in 0..11 {
            m.set(Settings { lf_in: true, lf: i, hf_in: false, hf: 0 });
            assert!((m.response_db(LF_HZ[i]) + 3.0).abs() < 0.1);
            m.set(Settings { lf_in: false, lf: 0, hf_in: true, hf: i });
            assert!((m.response_db(HF_HZ[i]) + 3.0).abs() < 0.1);
        }
    }

    #[test]
    fn both_in_make_a_band_pass() {
        let mut m = M464::new(48000.0);
        m.set(Settings { lf_in: true, lf: 3, hf_in: true, hf: 8 });
        assert!(m.response_db(1000.0).abs() < 0.1);
        assert!(m.response_db(25.0) < -18.0);
        assert!(m.response_db(20000.0) < -10.0);
    }
}
