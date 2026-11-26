#pragma once
#include "../core/SharedPluginFrame.hpp"
#include "../core/StateVariableCore.hpp"
#include "../core/NonLinearCore.hpp"

namespace wbs::m404 {

class M404PluginProcessor : public engine::SharedPluginFrame {
public:
    M404PluginProcessor();
    ~M404PluginProcessor() override = default;

    void prepareModule(double hostRate, double oversampledRate, size_t maxBlockSize, size_t numChannels) override;
    void releaseModule() override;
    void updateParametersFromHost() noexcept override;
    void processModuleSlice(engine::AudioBlock& slice) noexcept override;
    void updateMeters(const engine::AudioBlock& audioBlock) noexcept override;

    // Parameter setters (called by UI or DAW host automation)
    void setHpfFrequency(float freqHz) noexcept;
    void setLpfFrequency(float freqHz) noexcept;
    void setHpfBypass(bool bypassed) noexcept;
    void setLpfBypass(bool bypassed) noexcept;
    void setInputDrive(float driveDb) noexcept;

    [[nodiscard]] float getInputMeter(size_t ch) const noexcept;
    [[nodiscard]] float getOutputMeter(size_t ch) const noexcept;

private:
    engine::StateVariableFilterCore hpfCores_[engine::kMaxChannels][2]; // Cascaded 18 dB/oct (2nd order + 1st order)
    engine::StateVariableFilterCore lpfCores_[engine::kMaxChannels][2];
    engine::NonLinearTransformerCore transformer_[engine::kMaxChannels];

    float hpfFreq_{ 20.0f };
    float lpfFreq_{ 25000.0f };
    bool hpfBypass_{ false };
    bool lpfBypass_{ false };
    float inputDrive_{ 1.0f };

    float inMeter_[engine::kMaxChannels]{ 0.0f };
    float outMeter_[engine::kMaxChannels]{ 0.0f };
};

} // namespace wbs::m404
