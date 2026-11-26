# Module Build Plan: WBS M404 / M464 Active High-Pass / Low-Pass Filter

**Module Identifier:** `WBS-M404-FILTER`  
**Classification:** Active Cutoff Filter Module (FILTERS series)  
**Historical Precedent:** 
- Ward-Beck M404 Hi-Lo Filter (1968, Drawing `404-02-010`, Checked by R. Beck)
- Ward-Beck M464 Continuous Filter (1976, Schematic `464-02-001`)  
**Shared Frame Engine:** `wbs::engine::SharedPluginFrame`  

---

## 1. Circuit Heritage & Transfer Function

The M404 features an 18 dB/octave (3-pole) active filter combining a 1st-order passive R-C stage with a 2nd-order active stage using a high-impedance dual-FET isolation buffer with positive feedback. 

$$\begin{aligned}
H_{\text{HPF}}(s) &= \frac{s^3}{(s + \omega_{p1})(s^2 + \frac{\omega_{p2}}{Q}s + \omega_{p2}^2)} \\
H_{\text{LPF}}(s) &= \frac{\omega_{p1} \omega_{p2}^2}{(s + \omega_{p1})(s^2 + \frac{\omega_{p2}}{Q}s + \omega_{p2}^2)}
\end{aligned}$$

* **Telephone Cue Bandpass Mode:** Engaging both HPF and LPF simultaneously generates the authentic broadcast "telephone cue" bandpass response.
* **Bypass:** Unity-gain flat buffer bypass matching the original M404 bypass switch.

---

## 2. Parameter Specification

| Param ID | Name | Type | Range | Default | Skew / Curve | Units |
|---|---|---|---|---|---|---|
| `hpf_freq` | High-Pass Freq | Float | 20.0 to 400.0 | 40.0 | Logarithmic | Hz |
| `hpf_enable` | High-Pass In | Bool | 0 or 1 | 1 | Boolean | On/Off |
| `lpf_freq` | Low-Pass Freq | Float | 4000.0 to 20000.0 | 16000.0 | Logarithmic | Hz |
| `lpf_enable` | Low-Pass In | Bool | 0 or 1 | 1 | Boolean | On/Off |
| `cue_telephone` | Telephone Cue | Bool | 0 or 1 | 0 | Boolean | Mode |
| `filter_slope` | Filter Slope | Choice | 12 dB/oct, 18 dB/oct | 18 dB/oct | Discrete | dB/oct |
| `master_bypass` | Bypass | Bool | 0 or 1 | 0 | Boolean | Bypass |

---

## 3. DSP Implementation & Inner Loop

```cpp
void M404Processor::processModuleSlice(AudioBlock& slice) noexcept {
    const float hpfF = hpfSmoothed_.getNextValue();
    const float lpfF = lpfSmoothed_.getNextValue();
    const bool hpfOn = hpfEnableParam_->get();
    const bool lpfOn = lpfEnableParam_->get();

    hpfFilterCore_.update(hpfF, 0.7071, 0.0, FilterType::HighPass18dB);
    lpfFilterCore_.update(lpfF, 0.7071, 0.0, FilterType::LowPass18dB);

    for (size_t ch = 0; ch < slice.numChannels; ++ch) {
        Sample* data = slice.getChannelPointer(ch);
        for (size_t i = 0; i < slice.numSamples; ++i) {
            Sample s = data[i];
            if (hpfOn) s = hpfFilterCore_.processSample(s);
            if (lpfOn) s = lpfFilterCore_.processSample(s);
            data[i] = s;
        }
    }
}
```

---

## 4. Resource Budgets & Performance Targets

- **CPU Budget:** < 0.3% per stereo instance on Apple M1 / Intel Core i7 @ 48 kHz.
- **Latency:** 0 samples in 1x mode; 7 samples in minimum-phase oversampling mode.
- **Memory Footprint:** < 4 KB static state per instance; zero heap allocations during playback.

---

## 5. Verification & Test Plan

1. **Attenuation Slope Verification:** Verify -18 dB attenuation at one octave below cutoff for HPF and one octave above cutoff for LPF.
2. **Telephone Bandpass Test:** Verify center frequency response at 1.2 kHz with -3 dB points at 300 Hz and 3.4 kHz when `cue_telephone` is active.
3. **DAW Automation Stress Test:** Modulate `hpf_freq` from 20 Hz to 400 Hz via 128 Hz sine automation; verify zero clicks or filter explosions.
