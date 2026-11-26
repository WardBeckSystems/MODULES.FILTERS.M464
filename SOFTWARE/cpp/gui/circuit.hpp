// M464: its block of 2030.env/vst/core/CircuitDiagramDefinitions.hpp.
        case 0: // M404 / M464 Filter
            d.modelCode = "M404";
            d.circuitTitle = "Ward-Beck M404/M464 Cutoff Filter Circuit";
            d.theoreticalExplanation =
                "Balanced audio drives a 600-ohm:10k-ohm Jensen transformer for iron saturation. "
                "The signal passes through cascaded 18 dB/oct (3rd-order) active filters: a Butterworth "
                "HPF (2nd-order active + 1st-order passive pole) and a Bessel linear-phase LPF, with zero phase distortion.";
            d.nodes = {
                { "IN",   "Jensen In",   "600:10k Iron",  { 20, 20, 90, 48 }, true, Palette::CopperWire },
                { "HPF",  "HPF 18dB/o",  "Butterworth",   { 130, 20, 100, 48 }, true, Palette::CircuitBorder },
                { "LPF",  "LPF 18dB/o",  "Bessel Phase",  { 250, 20, 100, 48 }, true, Palette::CircuitBorder },
                { "IRON", "Iron Core",   "80-Series Sat", { 370, 20, 90, 48 }, true, Palette::CopperWire },
                { "OUT",  "Line Out",    "+28 dBu Driver",{ 480, 20, 90, 48 }, true, Palette::LedGreen }
            };
            d.wires = {
                { "IN", "HPF", "Input Drive" },
                { "HPF", "LPF", "High-Pass Sig" },
                { "LPF", "IRON", "Band-Limited" },
                { "IRON", "OUT", "Harmonic Iron" }
            };
            break;
