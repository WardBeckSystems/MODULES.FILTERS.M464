#include "PluginProcessor.hpp"
#include <cmath>

namespace wbs::m404 {

M404PluginProcessor::M404PluginProcessor()
    : engine::SharedPluginFrame("Ward-Beck M404/M464 Cutoff Filters", "M404") {
    defaultOversampling_ = engine::OversamplingRate::TwoX;
}

void M404PluginProcessor::prepareModule(double hostRate, double oversampledRate, size_t maxBlockSize, size_t numChannels) {
    for (size_t ch = 0; ch < numChannels; ++ch) {
        transformer_[ch].prepare(hostRate);
        for (size_t stage = 0; stage < 2; ++stage) {
            hpfCores_[ch][stage].prepare(oversampledRate);
            lpfCores_[ch][stage].prepare(oversampledRate);
        }
    }
    updateParametersFromHost();
}

void M404PluginProcessor::releaseModule() {
    for (size_t ch = 0; ch < numChannels_; ++ch) {
        transformer_[ch].reset();
        for (size_t stage = 0; stage < 2; ++stage) {
            hpfCores_[ch][stage].reset();
            lpfCores_[ch][stage].reset();
        }
    }
}

void M404PluginProcessor::updateParametersFromHost() noexcept {
    for (size_t ch = 0; ch < numChannels_; ++ch) {
        // Stage 0: 2nd order Butterworth (Q = 1.0)
        // Stage 1: 1st order real pole (Q = 0.5) to synthesize 18 dB/oct 3rd order response
        hpfCores_[ch][0].update(hpfFreq_, 1.0, 0.0, engine::FilterType::HighPass18dB);
        hpfCores_[ch][1].update(hpfFreq_, 0.5, 0.0, engine::FilterType::HighPass18dB);

        lpfCores_[ch][0].update(lpfFreq_, 1.0, 0.0, engine::FilterType::LowPass18dB);
        lpfCores_[ch][1].update(lpfFreq_, 0.5, 0.0, engine::FilterType::LowPass18dB);
    }
}

void M404PluginProcessor::processModuleSlice(engine::AudioBlock& slice) noexcept {
    for (size_t ch = 0; ch < slice.numChannels; ++ch) {
        float* channelData = slice.getChannelPointer(ch);

        for (size_t i = 0; i < slice.numSamples; ++i) {
            float s = channelData[i];

            // 1. Input Iron Saturation
            s = transformer_[ch].processSample(s, inputDrive_, 0.15f);

            // 2. High-Pass Filter (18 dB/oct Bessel/Butterworth)
            if (!hpfBypass_) {
                s = hpfCores_[ch][0].processSample(s);
                s = hpfCores_[ch][1].processSample(s);
            }

            // 3. Low-Pass Filter (18 dB/oct)
            if (!lpfBypass_) {
                s = lpfCores_[ch][0].processSample(s);
                s = lpfCores_[ch][1].processSample(s);
            }

            channelData[i] = s;
        }
    }
}

void M404PluginProcessor::updateMeters(const engine::AudioBlock& audioBlock) noexcept {
    for (size_t ch = 0; ch < audioBlock.numChannels; ++ch) {
        const float* data = audioBlock.getChannelPointer(ch);
        float peak = 0.0f;
        for (size_t i = 0; i < audioBlock.numSamples; ++i) {
            peak = std::max(peak, std::abs(data[i]));
        }
        outMeter_[ch] = (outMeter_[ch] * 0.9f) + (peak * 0.1f);
    }
}

void M404PluginProcessor::setHpfFrequency(float freqHz) noexcept {
    hpfFreq_ = std::clamp(freqHz, 20.0f, 400.0f);
}

void M404PluginProcessor::setLpfFrequency(float freqHz) noexcept {
    lpfFreq_ = std::clamp(freqHz, 4000.0f, 25000.0f);
}

void M404PluginProcessor::setHpfBypass(bool bypassed) noexcept {
    hpfBypass_ = bypassed;
}

void M404PluginProcessor::setLpfBypass(bool bypassed) noexcept {
    lpfBypass_ = bypassed;
}

void M404PluginProcessor::setInputDrive(float driveDb) noexcept {
    inputDrive_ = std::pow(10.0f, driveDb / 20.0f);
}

float M404PluginProcessor::getInputMeter(size_t ch) const noexcept {
    return (ch < engine::kMaxChannels) ? inMeter_[ch] : 0.0f;
}

float M404PluginProcessor::getOutputMeter(size_t ch) const noexcept {
    return (ch < engine::kMaxChannels) ? outMeter_[ch] : 0.0f;
}

} // namespace wbs::m404
