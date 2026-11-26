// Copyright (c) 2026 Ward-Beck Systems (Ward-Beck.com - Ward-Beck.ca). All rights reserved.
// M464: its part of 2030.env/vst/clap_wrapper/ClapModuleRegistry.cpp (suite index 0).

// --- descriptor ---
static const clap_plugin_descriptor_t s_desc_m404 = {
    CLAP_VERSION,
    "ca.wbsps.m404_m464_filter",
    "Ward-Beck M404/M464 Cutoff Filters",
    "Ward-Beck Systems Preservation Society",
    "https://wbsps.ca",
    "",
    "https://wbsps.ca/contact",
    "1.0.0",
    "Classic 18 dB/oct high-pass and low-pass cutoff filter with transformer iron saturation",
    kFilterFeatures
};

// --- parameters ---
std::vector<ParameterInfo> buildM404Params() {
    std::vector<ParameterInfo> p;
    p.push_back({
        0, "HPF Frequency", "Filter", 20.0, 400.0, 20.0,
        CLAP_PARAM_IS_AUTOMATABLE,
        [](engine::SharedPluginFrame* f, double val) { static_cast<m404::M404PluginProcessor*>(f)->setHpfFrequency(static_cast<float>(val)); },
        nullptr,
        [](double v) { char buf[32]; std::snprintf(buf, sizeof(buf), "%.1f Hz", v); return std::string(buf); },
        nullptr
    });
    p.push_back({
        1, "LPF Frequency", "Filter", 4000.0, 25000.0, 25000.0,
        CLAP_PARAM_IS_AUTOMATABLE,
        [](engine::SharedPluginFrame* f, double val) { static_cast<m404::M404PluginProcessor*>(f)->setLpfFrequency(static_cast<float>(val)); },
        nullptr,
        [](double v) { char buf[32]; std::snprintf(buf, sizeof(buf), "%.1f Hz", v); return std::string(buf); },
        nullptr
    });
    p.push_back({
        2, "HPF Bypass", "Filter", 0.0, 1.0, 0.0,
        CLAP_PARAM_IS_AUTOMATABLE | CLAP_PARAM_IS_STEPPED,
        [](engine::SharedPluginFrame* f, double val) { static_cast<m404::M404PluginProcessor*>(f)->setHpfBypass(val >= 0.5); },
        nullptr,
        [](double v) { return v >= 0.5 ? "Bypassed" : "Active"; },
        nullptr
    });
    p.push_back({
        3, "LPF Bypass", "Filter", 0.0, 1.0, 0.0,
        CLAP_PARAM_IS_AUTOMATABLE | CLAP_PARAM_IS_STEPPED,
        [](engine::SharedPluginFrame* f, double val) { static_cast<m404::M404PluginProcessor*>(f)->setLpfBypass(val >= 0.5); },
        nullptr,
        [](double v) { return v >= 0.5 ? "Bypassed" : "Active"; },
        nullptr
    });
    p.push_back({
        4, "Input Drive", "Saturation", -18.0, 18.0, 0.0,
        CLAP_PARAM_IS_AUTOMATABLE,
        [](engine::SharedPluginFrame* f, double val) { static_cast<m404::M404PluginProcessor*>(f)->setInputDrive(static_cast<float>(val)); },
        nullptr,
        [](double v) { char buf[32]; std::snprintf(buf, sizeof(buf), "%+.1f dB", v); return std::string(buf); },
        nullptr
    });
    return p;
}

// --- instance ---
        case 0:
            inst = new ClapPluginInstance(
                &s_desc_m404, host,
                []() { return std::make_unique<m404::M404PluginProcessor>(); },
                buildM404Params(),
                0
            );
            break;

