#pragma once
#include "PluginProcessor.hpp"

namespace wbs::m404 {

class M404PluginEditor {
public:
    explicit M404PluginEditor(M404PluginProcessor& processor)
        : processor_(processor) {}

    void paint() const noexcept;
    void resized() noexcept;

    int getWidth() const noexcept { return 360; }
    int getHeight() const noexcept { return 520; }

private:
    M404PluginProcessor& processor_;
};

} // namespace wbs::m404
