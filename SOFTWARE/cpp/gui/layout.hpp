// M464: its block of 2030.env/vst/core/ModuleGuiLayouts.hpp (176 x 770 = the faceplate).
        case 0: { // M404 / M464 Filter (176 x 770)
            m.title = "M404 Cutoff Filter";
            m.subtitle = "18 dB/oct Cascaded Active Filters";
            m.modelCode = "M404";

            // HPF Section
            m.knobs.push_back({ 0, "HPF FREQ", "Hz", 88.0f, 115.0f, 24.0f, 20.0, 400.0, 20.0, false, KnobStyle::WbsElmaWing, Palette::ElmaSlate });
            m.buttons.push_back({ 2, "HPF BYPASS", { 43.0f, 175.0f, 90.0f, 24.0f }, false, ButtonStyle::LatchingPush, Palette::LedAmber });

            // LPF Section
            m.knobs.push_back({ 1, "LPF FREQ", "Hz", 88.0f, 260.0f, 24.0f, 4000.0, 25000.0, 25000.0, false, KnobStyle::WbsElmaWing, Palette::ElmaSlate });
            m.buttons.push_back({ 3, "LPF BYPASS", { 43.0f, 320.0f, 90.0f, 24.0f }, false, ButtonStyle::LatchingPush, Palette::LedAmber });

            // Drive Section
            m.knobs.push_back({ 4, "IRON DRIVE", "dB", 88.0f, 410.0f, 22.0f, -18.0, 18.0, 0.0, false, KnobStyle::WbsElmaPointer, Palette::CopperWire });

            m.themeToggleRect = { 25.0f, 726.0f, 126.0f, 22.0f };
            break;
        }
