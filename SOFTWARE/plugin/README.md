# M464 High Pass / Low Pass Filter: plugin

The Ward-Beck M464 High Pass / Low Pass Filter (1976|9) as a **VST3 / CLAP plugin** with its front panel, and as
the DSP the **Ward-Beck Audio Tap** can run as an insert.

**Download** the VST3 / CLAP for Windows, macOS and Linux from this repo's [releases](../../../../releases/latest). All Ward-Beck plugins: [Ward-Beck Plugin Core](https://github.com/WardBeckSystems/Ward-Beck-Plugin-Core#plugins).

| Control | Positions | What it does |
|---|---|---|
| HF CUT (SW2) | 500, 1k, 2k, 3k, 4k, 5k, 6k, 8k, 10k, 12k, 15k Hz | low pass, 18 dB/octave, calibrated at the 3 dB corner |
| HF (SW4) + LED | in / out | inserts the HF cut and lights its LED |
| LF CUT (SW1) | 27, 45, 70, 100, 150, 220, 330, 500, 1k, 2k, 4k Hz | high pass, 18 dB/octave, calibrated at the 3 dB corner |
| LF (SW3) + LED | in / out | inserts the LF cut and lights its LED |

Unity gain. Each cut is a 6 dB/octave section and a 12 dB/octave section in series (as on the schematic),
aligned as a 3rd-order Butterworth so every switch position is its -3 dB point at any sample rate.

```bash
cargo xtask bundle wbs-m464 --release        # target/bundled/wbs-m464.vst3 and wbs-m464.clap
cargo test --no-default-features
python3 tools/build_plugins.py M464          # (from WBSps.ca) build, deploy to ~/.vst3 ~/.clap, load in REAPER
```

**Sources.** Technical data and circuit description (manual pages 1-1, 2-1, 2-2); front panel 464-04-003 issue 2
(the switch positions, layout and insert buttons).

Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
