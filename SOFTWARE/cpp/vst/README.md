# Ward-Beck Systems M404 / M464 High & Low Cut Filter VST/AU Plugin

**Hardware Unit:** Ward-Beck Systems M404 & M464 High/Low Cut Filters (Drawings `404-00-001` & `464-00-002`)  
**Format:** VST3, AU, AAX (C++20 / JUCE 7)  
**Core Shared Engine:** [`wbs_core`](file:///home/anthony/Documents/GitProjects/WBSps.ca/2030.env/vst/core/README.md)

---

## Technical Specifications
- **High-Pass Filter:** $20\text{ Hz}$ to $400\text{ Hz}$ continuous sweep, $18\text{ dB/octave}$ 3rd-order Bessel/Butterworth alignment.
- **Low-Pass Filter:** $4.0\text{ kHz}$ to $25.0\text{ kHz}$ continuous sweep, $18\text{ dB/octave}$ 3rd-order Bessel/Butterworth alignment.
- **Input Transformer:** Jensen iron transformer saturation modeling core flux hysteresis and gentle 2nd/3rd harmonics.
- **Oversampling:** $2\times$ polyphase half-band FIR upsampling to eliminate Nyquist phase warping near $20\text{ kHz}$.
