// Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
//! The WBS 18 dB/octave cut filter: a 6 dB/octave section and a 12 dB/octave section in series, as on the
//! M404 and M464 schematics, aligned as a 3rd-order Butterworth so the switch calibration is the -3 dB point.
//! Bilinear transform with pre-warping: the corner lands on the marked frequency at any sample rate.

pub const MAX_CH: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    /// LF CUT (high pass)
    HighPass,
    /// HF CUT (low pass)
    LowPass,
}

/// First-order section, y = b0 x + b1 x[-1] - a1 y[-1]
#[derive(Clone, Copy, Default)]
struct Pole {
    b0: f32,
    b1: f32,
    a1: f32,
    x1: [f32; MAX_CH],
    y1: [f32; MAX_CH],
}

/// Second-order section, transposed direct form II
#[derive(Clone, Copy, Default)]
struct Biquad {
    b: [f32; 3],
    a: [f32; 2],
    z: [[f32; 2]; MAX_CH],
}

#[derive(Clone, Copy, Default)]
pub struct Cut {
    pole: Pole,
    quad: Biquad,
}

impl Cut {
    pub fn new(kind: Kind, hz: f32, rate: f32) -> Self {
        let mut c = Cut::default();
        c.set(kind, hz, rate);
        c
    }

    /// Corner `hz` (-3 dB), kept below Nyquist.
    pub fn set(&mut self, kind: Kind, hz: f32, rate: f32) {
        let f = hz.clamp(1.0, rate * 0.45);
        let k = (std::f64::consts::PI * f as f64 / rate as f64).tan();
        // 3rd-order Butterworth: one real pole and a pair with Q = 1
        let q = 1.0;
        let a1p = (k - 1.0) / (k + 1.0);
        let n = 1.0 / (1.0 + k / q + k * k);
        let (a1, a2) = (2.0 * (k * k - 1.0) * n, (1.0 - k / q + k * k) * n);
        match kind {
            Kind::LowPass => {
                let g = k / (1.0 + k);
                self.pole.b0 = g as f32;
                self.pole.b1 = g as f32;
                let b0 = k * k * n;
                self.quad.b = [b0 as f32, (2.0 * b0) as f32, b0 as f32];
            }
            Kind::HighPass => {
                let g = 1.0 / (1.0 + k);
                self.pole.b0 = g as f32;
                self.pole.b1 = -g as f32;
                self.quad.b = [n as f32, (-2.0 * n) as f32, n as f32];
            }
        }
        self.pole.a1 = a1p as f32;
        self.quad.a = [a1 as f32, a2 as f32];
    }

    pub fn reset(&mut self) {
        self.pole.x1 = [0.0; MAX_CH];
        self.pole.y1 = [0.0; MAX_CH];
        self.quad.z = [[0.0; 2]; MAX_CH];
    }

    #[inline]
    pub fn run(&mut self, ch: usize, x: f32) -> f32 {
        let p = &mut self.pole;
        let y = p.b0 * x + p.b1 * p.x1[ch] - p.a1 * p.y1[ch];
        p.x1[ch] = x;
        p.y1[ch] = y;
        let q = &mut self.quad;
        let z = &mut q.z[ch];
        let out = q.b[0] * y + z[0];
        z[0] = q.b[1] * y - q.a[0] * out + z[1];
        z[1] = q.b[2] * y - q.a[1] * out;
        out
    }

    /// Flush denormals and any blow-up once per block.
    pub fn ok(&mut self) {
        for ch in 0..MAX_CH {
            for v in self.quad.z[ch].iter_mut().chain([&mut self.pole.x1[ch], &mut self.pole.y1[ch]]) {
                if !v.is_finite() || v.abs() < 1e-20 {
                    *v = 0.0;
                }
            }
        }
    }

    /// Magnitude in dB at `hz` (for tests and for drawing the curve).
    pub fn response_db(&self, hz: f32, rate: f32) -> f32 {
        let w = 2.0 * std::f64::consts::PI * hz as f64 / rate as f64;
        let mag = |b: &[f64], a: &[f64]| {
            let num = (b.iter().enumerate().map(|(i, v)| v * (i as f64 * w).cos()).sum::<f64>(),
                       -b.iter().enumerate().map(|(i, v)| v * (i as f64 * w).sin()).sum::<f64>());
            let den = (a.iter().enumerate().map(|(i, v)| v * (i as f64 * w).cos()).sum::<f64>(),
                       -a.iter().enumerate().map(|(i, v)| v * (i as f64 * w).sin()).sum::<f64>());
            ((num.0 * num.0 + num.1 * num.1) / (den.0 * den.0 + den.1 * den.1)).sqrt()
        };
        let p = &self.pole;
        let q = &self.quad;
        let m = mag(&[p.b0 as f64, p.b1 as f64], &[1.0, p.a1 as f64])
            * mag(&[q.b[0] as f64, q.b[1] as f64, q.b[2] as f64], &[1.0, q.a[0] as f64, q.a[1] as f64]);
        (20.0 * m.log10()) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corner_is_minus_3_db_and_slope_is_18_db_per_octave() {
        for rate in [44100.0, 48000.0, 96000.0] {
            for hz in [27.0, 100.0, 1000.0, 4000.0] {
                let c = Cut::new(Kind::HighPass, hz, rate);
                assert!((c.response_db(hz, rate) + 3.01).abs() < 0.05, "HP {hz} @ {rate}");
                assert!(c.response_db(hz * 8.0, rate).abs() < 0.05);
                let down = c.response_db(hz / 4.0, rate) - c.response_db(hz / 8.0, rate);
                assert!((down - 18.0).abs() < 0.3, "HP slope {down}");
            }
            for hz in [500.0, 5000.0, 15000.0] {
                let c = Cut::new(Kind::LowPass, hz, rate);
                assert!((c.response_db(hz, rate) + 3.01).abs() < 0.05, "LP {hz} @ {rate}");
                assert!(c.response_db(hz / 8.0, rate).abs() < 0.05);
            }
        }
    }

    #[test]
    fn processing_matches_the_response() {
        let rate = 48000.0;
        let mut c = Cut::new(Kind::LowPass, 1000.0, rate);
        let hz = 4000.0;
        let mut peak = 0.0f32;
        for i in 0..48000 {
            let y = c.run(0, (2.0 * std::f32::consts::PI * hz * i as f32 / rate).sin());
            if i > 24000 {
                peak = peak.max(y.abs());
            }
        }
        let db = 20.0 * peak.log10();
        assert!((db - c.response_db(hz, rate)).abs() < 0.2, "{db}");
    }
}
