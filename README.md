# M464 High Pass Low Pass Filter · Filters

Ward-Beck Systems M464, 1976|9. Wiki page: [M464 on WBSps.ca](https://wardbecksystems.github.io/WBSps.ca/PAGES/M464.html)

| Folder | What is in it | Files |
|---|---|---|
| [SCHEMATICS/](SCHEMATICS/) | electrical drawings: schematics, functional diagrams, component layouts, wiring | 27 |
| [MECHANICAL/](MECHANICAL/) | front panels, cover plates, metalwork, assemblies; the front panel as vectors | 15 |
| [PINOUTS/](PINOUTS/) | the RTO-144 connector: connection sheets, connector layouts, rear views | 15 |
| [MANUALS/](MANUALS/) | manuals, manual pages, price lists, brochures, documents | 207 |
| [PHOTOS/](PHOTOS/) | photographs of the module | 23 |
| [SOFTWARE/](SOFTWARE/) | program listings, firmware, the plugin | 2 |
| [KICAD/](KICAD/) | KiCad schematics traced from the drawings, the symbol, PCB projects | 8 |
| [OCR/](OCR/) | the note of every file: date, origin, title block, OCR text | 78 |

<!-- module data: tools/module_specs.py, tools/forum_quotes.py -->

**On the WBSps forum (2006|39 - 2014|47):** [9 topics](FORUM.md): what owners and technicians said about it.

## Specifications

From the technical data sheet: [19xx - M464 - DOC - WBS sales 20.jpg](MANUALS/19xx%20-%20M464%20-%20DOC%20-%20WBS%20sales%2020.jpg) page 1 (read by OCR; check against the page). As data: [`Specifications.json`](Specifications.json).

| | |
|---|---|
| **Gain** | Unity _ ±0.25 dB from 20 Hz to 20 kHz (with both insert switches in bypass position) |
| **Frequency response** |  |
| &nbsp;&nbsp;Equalization | High frequency and Tow frequency Cut _ choice of 11 discrete frequencies each. Frequency tolerance: +5% Slope: 18 dBloctave |
| **Noise** | -80 dBm. (20 Hz to 20 kHz) |
| **Harmonic distortion** | Less than 0.35% THD from 30 Hz to 15 kHz at +24 dBm input Less than 0.1 % at kHz at +28 dBm input level |
| **Source impedance** | 600 ohms maximum, ba lanced/unba lanced. |
| **Input impedance** | 30k ohms +10% _ baLanced |
| **Input level** | -10 to +8 dBm nomina ] maximum_ +30 dBm |
| **Output impedance** | Less than 100 ohms |
| **Load impedance** | 600 ohms minimum, ba Tanced_ Output Leve] -10 to +8 dBm nomina ] maximum_ +30 dBm |
| **Phasing** | Low number terminals of input and output are in phase |
| **Power requirements** | Ambient Temperature: +25 volts dc, 120 ma maximum 0 to 60 degrees Celsius 1.6&quot; wide X 7&quot; high X 6.5 &quot; deep (41 mm X 178 mm X 165 mm) |

## Circuit description

From [1976 - M464 - HP LP Filter.pdf](MANUALS/1976%20-%20M464%20-%20HP%20LP%20Filter.pdf) pages 6-11 (read by OCR; check against the pages). As data: [`CircuitDescription.json`](CircuitDescription.json).

<details><summary>Read it</summary>

M464 HIGH PASS-Lov Pass FILLER Input The inputs igna ] enters the M464 module via pins and 2 of thc edgc connector ZI and 22 serve as the preamplifier RI7 i$ part of the preamp]ificr and i$ used Lo trim the overall gain of the module to unity The prcamp output is routed to the input of the LON FREQUENCY CUT filter 4 $ well a $ to one 0 [ the norila Tly closed contacts 0 [ the LF CUT INSERT switch Sn3 LON_FREQUENCY CUT (high_pass Filter After passing through the isolation amplifier 13 the signa 1 enters thc first section of lhe high pass filter comprised of C15 SwiA and 14, and producing 6 dB/oclave attenuation A further 12 dBfoctave section consi sl so f C18 C19 SNIB, ShIc and 75 resulting in combined attenuation rale of 18. dB/octave SwI a |ows to select any one of eleven discrete cut-off frequencies ranging from 27 Hertz Lo 4 kilo Hertz The reference frequency represents thc -3 dR corner point from the nomina is igra] level The filter outpul i5 connected to one of the normally open contacts of the LF CVT INSERT switch Sw3 The output of S3 Ts routed to the input of the HIGH FREQUENCY CUt filter a $ well a5 to one of the normally closed contacts of the IF CUt INSERT switch SW4 HIGH FREQUENCY CuT (low pass) Filter As; is the case with the high pas $ filter, the Tow pa s$ filter is divided into 6 dB/octave and 12 dBloctave section, for total of 18 dB/octave The dBoctave section consists of Snza C23, and 26 while the 12 dB/octave section includes SWZB, SnZc, C27, C31 and 27 _ SW2 allows selection of any one of eleven discrete cut-off frequencies extending from 500. Hertz to 15_ kilo Hertz The reference frequency designatcs the -3 dB corner point from the nOm ina ] signa] level The filter output '15 connected to one of the norma y 'open contacts of the HF CUt INSERT switch Sn4

**CIRCUII__DESCRIPIION**

M464 HIGH PAss-Low Pass FILLER Output The output of Sw4 15 connected directly to the 4-109a output amplifier The actively balanced output of the 4-109 15 brought oul aL pins 15 and 16 of the edge connector INSERL Switches ad Indicators The LF CUt INSERT swilch Sw3 and thce HF CUT INSERT switch SW4 maybe selected in any desired combination The filter section inserted into the signa] path will bc indicated by the LED indicator adjacent to cach insert switch Power_Supply Decoupl ing An active power. supply decoup] ing network consisting of Q1 through Q6: provides high degree of isolation between the M464 and the +25 vo It supply ra i 1 $ Transistors 03 and Q4 served 5 current monitors and Wil7 imit the current flowing through the pass transistors Q1 and Q6 in the event that excessive power supply demarids should arise 4$ result of component failure

FREQUENCY RESPONSE CHARACTERISTICS M464 HIGIL PASS-Low PASS FILLER

8 2&amp; # 8

Low FREQUENCY CUI

10 Hz 100Hz

**WIGH_FREQUENCY CUT**

724 20Hz 100 Hz 1kHz '1Ok 20k 30k 40k

**MFG PART NUMBER**

C1 c2 C3 C4 c5 Capacitor 22 uF/25 siemens

**B41313-45226-2**

33 uF/10 V Kemet

**T3900336M01045**

C6 C7 C8 c9 C10 220 pF 4.1 pF 220 PF 6.8 pF Philips 2222-630-03221 2222-638-09478 2222-630-03221 2222-638-09688

**CIT C12 C13 C14 C15**

150 pF 33 uf /10 V 2222-638-70151 T3900336m01a5 Kemet 10 pF 0.04 uF, +5% Philips Aselco

**2222-638-10109 WBS 13-002**

C16 C17 C]8 c19 C20 0.47 uF/35 V 8.2 PF 0.04 uF = +52 0.02 uF +53 0, 47 uF/35 Kemet Philips Aselco T3904474m035a5 2222 638-09828 NBS 13-002 WBS 13-003 13904474M03545 Kemet

**621 C22 C23 C24 C25**

22 pF 150 ur/6 V 4000 pF +5 % 0.47 uF/35 Philips Keme l Aselco Kemet 2222-638-58229 1390E157m00645 WBS 13-005 T3904474m035a5

**C26 C27 C28 C29 C30**

10 pF 5000 pF 45% 0: 47 uFi35 V Philips Aselco Kemet 2222-638-10109 VBS 13 -007 T3904474m035a5 8.2 pF Philips 2222-638-09828

**'C31 C32**

0.01 uF 152 33 uF/io Aselco Kemet WBS 13 -004 T390133611010a5

**CRI CR2 CR 3 CR4**

Diode Motoro]d

**DSI DS?**

LCD red Fairchild

**FLV 117**

Transistor Texa5 Nationa 1

**83 2 3 Q 4 5**

Pn3645

Texas Philips

**RI R2 R3 R4 R5**

Resistor #W.5% 15k 1k 3.3 3 TK

**R6 RZ R8 R9 RI0**

15k 470k T75k 75k 18k

**RII R12 R13 RI4 R1S**

18K 620 18K 18k 620

**R16 RI7 R18 RT9 R20**

5k6 Trim pot Sk Resistor #W 5 % 1K2 6k8 820 Piher Philips PT 1OV 5k

</details>

<!-- /module data -->

## Photos

_Photographs of the module itself: front, back, inside, racked._

<table>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKcU5kbEpmUGh3dWs/view?resourcekey=0-gU9FZGDoDcbTQQMomdjhOQ" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKcU5kbEpmUGh3dWs&resourcekey=0-gU9FZGDoDcbTQQMomdjhOQ" width="240" alt="1976 - M464 - HP LP Fileters.jpg"></a><br><sub>1976|9 · HP LP Fileters</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKTzJWbmZTRko0Mm8/view?resourcekey=0-NJJh1LuEk3QcC0YrYOHlGw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKTzJWbmZTRko0Mm8&resourcekey=0-NJJh1LuEk3QcC0YrYOHlGw" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 002.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 002</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKeGNMeldKb3hZSjg/view?resourcekey=0-u8ubpOWN7MrtxWBBG0txxw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKeGNMeldKb3hZSjg&resourcekey=0-u8ubpOWN7MrtxWBBG0txxw" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 003.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 003</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKak8yMEVaQ0Z4bWs/view?resourcekey=0-Ieqpwz1FdQeEAZUi7IUTDA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKak8yMEVaQ0Z4bWs&resourcekey=0-Ieqpwz1FdQeEAZUi7IUTDA" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 004.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 004</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKOTcteThsZXpudE0/view?resourcekey=0-qLVJglMDHumA-QA8FvX2Aw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKOTcteThsZXpudE0&resourcekey=0-qLVJglMDHumA-QA8FvX2Aw" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 005.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 005</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKTVhXUUhMNUN2V0k/view?resourcekey=0-5H5UBF71-8-squsJOKethQ" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKTVhXUUhMNUN2V0k&resourcekey=0-5H5UBF71-8-squsJOKethQ" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 006.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 006</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKcXRrN2piU2dQTFE/view?resourcekey=0-Bq7fFbBbhba6cMpuS8uO1g" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKcXRrN2piU2dQTFE&resourcekey=0-Bq7fFbBbhba6cMpuS8uO1g" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 007.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 007</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKb29qRW1lNUNqVHM/view?resourcekey=0-DwnFFu7wbLCjVyq0XXc3MA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKb29qRW1lNUNqVHM&resourcekey=0-DwnFFu7wbLCjVyq0XXc3MA" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 008.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 008</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKejB2bWlUR3RJNzA/view?resourcekey=0-_GO2lM2SJsTWtzGvBgAHig" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKejB2bWlUR3RJNzA&resourcekey=0-_GO2lM2SJsTWtzGvBgAHig" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 009.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 009</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKWS1fZ2o0VnN2THc/view?resourcekey=0-DngTePHu4Fzx0ICwOPNgDw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKWS1fZ2o0VnN2THc&resourcekey=0-DngTePHu4Fzx0ICwOPNgDw" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 010.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 010</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKcW1OWVJjQ2JJbk0/view?resourcekey=0-L5pR9E9UEAfJRkc-obVj0A" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKcW1OWVJjQ2JJbk0&resourcekey=0-L5pR9E9UEAfJRkc-obVj0A" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 011.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 011</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKRVcwb2t6SVFvQjg/view?resourcekey=0-QV-kp5LYa-9eZohcioswOg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKRVcwb2t6SVFvQjg&resourcekey=0-QV-kp5LYa-9eZohcioswOg" width="240" alt="1976 - M464 - wbsps - module - m464 hp lp filters 012.jpg"></a><br><sub>1976|9 · wbsps - module - m464 hp lp filters 012</sub></td></tr>
</table>

Shared privately on Google Drive (opens after sign-in): [1976|9 · wbsps - module - m464 hp lp fileters 001_small](https://drive.google.com/file/d/0B0yR0b5T78SKbzlvSVIzSVdMc3c/view?resourcekey=0-pySM64wnVpQyvcO4zdMAhg) · [1976|9 · wbsps - module - m464 hp lp fileters 002_small](https://drive.google.com/file/d/0B0yR0b5T78SKeHVYdUplV2xoT0k/view?resourcekey=0-YVQS_C_719njPM4QKQjnww) · [1976|9 · wbsps - module - m464 hp lp fileters 003_small](https://drive.google.com/file/d/0B0yR0b5T78SKRW1SWGgtT2k3NWs/view?resourcekey=0-B0yrFIVES-l7zd3mDPw4hg) · [1976|9 · wbsps - module - m464 hp lp fileters 004_small](https://drive.google.com/file/d/0B0yR0b5T78SKeXQwejFNMGR6SHM/view?resourcekey=0-vQErOUFPaXxWcdr9VgA58A) · [1976|9 · wbsps - module - m464 hp lp fileters 005_small](https://drive.google.com/file/d/0B0yR0b5T78SKZGFIREtQSC1pemM/view?resourcekey=0-RWk3wtdziGQ5w1UtqAStcQ) · [1976|9 · wbsps - module - m464 hp lp fileters 006_small](https://drive.google.com/file/d/0B0yR0b5T78SKRnpHenNndnVyMUE/view?resourcekey=0-RIriC9S9gsgQltOiQhJgxg) · [1976|9 · wbsps - module - m464 hp lp fileters 007_small](https://drive.google.com/file/d/0B0yR0b5T78SKc3lEYkZmMk9XazQ/view?resourcekey=0-lGctDighZS8hTGKXvwYdVg) · [1976|9 · wbsps - module - m464 hp lp fileters 008_small](https://drive.google.com/file/d/0B0yR0b5T78SKSEJaUUxZUXhFeWs/view?resourcekey=0-INjWbf_Qd-db4gMg2a8QfQ) · [1976|9 · wbsps - module - m464 hp lp fileters 009_small](https://drive.google.com/file/d/0B0yR0b5T78SKV2V3ZUtNZmRJcEU/view?resourcekey=0-SBlnDA0KDJ4NIjN9rX2XFQ) · [1976|9 · wbsps - module - m464 hp lp fileters 012_small](https://drive.google.com/file/d/0B0yR0b5T78SKaWM5TzZiRnNIZkk/view?resourcekey=0-2nvutlmQOFx9bxGW20H8Yg)

## Mechanical

_Mechanical drawings: front panels, cover plates, metalwork and hardware (-03- drawings), AutoCAD files, each with its `(EXTRACTED).svg` vectors; `<module> - front panel.svg` is the front panel as vectors._

<table>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKbG9qNGpGNWhvRGM/view?resourcekey=0-5H4EGSIGPpl9YXGXebjKcw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKbG9qNGpGNWhvRGM&resourcekey=0-5H4EGSIGPpl9YXGXebjKcw" width="240" alt="1976 - M464 - 464-04-003 - PASS-LO PASS FILTER.jpg"></a><br><sub>1976|9 · 464-04-003 - PASS-LO PASS FILTER · <a href="MECHANICAL/1976%20-%20M464%20-%20464-04-003%20-%20PASS-LO%20PASS%20FILTER%20(EXTRACTED).svg">vectors</a></sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKMzd6clc4NmloWnc/view?resourcekey=0-_XRSxxOosfvR8U1NMD6Xsw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKMzd6clc4NmloWnc&resourcekey=0-_XRSxxOosfvR8U1NMD6Xsw" width="240" alt="1977 - M464 - 464-04-003 - GNA PATH AND IUMINATES ASSOC IATED.jpg"></a><br><sub>1977|10 · 464-04-003 - GNA PATH AND IUMINATES ASSOC IATED · <a href="MECHANICAL/1977%20-%20M464%20-%20464-04-003%20-%20GNA%20PATH%20AND%20IUMINATES%20ASSOC%20IATED%20(EXTRACTED).svg">vectors</a></sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKZkR4U2c3NlRVQXc/view?resourcekey=0-kB3tcmFJNcB8YZRw9uLLUw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKZkR4U2c3NlRVQXc&resourcekey=0-kB3tcmFJNcB8YZRw9uLLUw" width="240" alt="19xx - M464 - DOC - 464-04-003 - PASS-LO PASS FILTER FRONT PANEL.jpg"></a><br><sub>19xx - M464 - DOC - 464-04-003 - PASS-LO PASS FILTER FRONT PANEL · <a href="MECHANICAL/19xx%20-%20M464%20-%20DOC%20-%20464-04-003%20-%20PASS-LO%20PASS%20FILTER%20FRONT%20PANEL%20(EXTRACTED).svg">vectors</a></sub></td></tr>
</table>

## 3-D model

[`MECHANICAL/M464.stl`](MECHANICAL/M464.stl) (GitHub turns it in 3-D) · [view, change and generate it on WBSps.ca](https://wardbecksystems.github.io/WBSps.ca/2026/stl.html?m=M464)

- **Size:** 53.54 × 199.71 mm panel, 165 mm deep, 3.2 mm plate (the drawing sheet, taken as drawn 1:1)
- **From the drawing:** 6 holes and cut-outs, 0 knobs (front panel: `MECHANICAL/M464 - front panel.svg`)
- the drawn panel is 0.21 wide to high, the size 0.27: placed by its height
- Made by `tools/module_models.py` (→ `MECHANICAL/M464.model.json`) and `tools/stl/make_stl.mjs` in [WBSps.ca](https://github.com/WardBeckSystems/WBSps.ca); the site makes the same STL from the same JSON.

## Schematics

_Electrical drawings: schematics (-02-), functional diagrams (-01-), component layouts (-06-), wiring (-09-), PCB files (.pcb .dsn), and beside each scan its `(EXTRACTED).svg`: the vectors traced from it._

<table>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKZmhjSWxfc05TT28/view?resourcekey=0-94cDU8WAAavaVAYIKyTErA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKZmhjSWxfc05TT28&resourcekey=0-94cDU8WAAavaVAYIKyTErA" width="240" alt="1976 - M464 - 464-02-001 - TWO COMPONENTS WITH THE PARALLEL SUM EQUALLING THE.jpg"></a><br><sub>1976|9 · 464-02-001 - TWO COMPONENTS WITH THE PARALLEL SUM EQUALLING THE · <a href="SCHEMATICS/1976%20-%20M464%20-%20464-02-001%20-%20TWO%20COMPONENTS%20WITH%20THE%20PARALLEL%20SUM%20EQUALLING%20THE%20(EXTRACTED).svg">vectors</a></sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/1qGnDXtWdXpuRFP5KM5lhmXeSrUlB7_Fm/view" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=1qGnDXtWdXpuRFP5KM5lhmXeSrUlB7_Fm" width="240" alt="1976 - M464 - DOC - 464-02-001 - HIGH PASS LOW PASS FILTER SCHEMATIC INISH TAAHMTA.jpg"></a><br><sub>1976|9 · DOC - 464-02-001 - HIGH PASS LOW PASS FILTER SCHEMATIC INISH TAAHMTA · <a href="SCHEMATICS/1976%20-%20M464%20-%20DOC%20-%20464-02-001%20-%20HIGH%20PASS%20LOW%20PASS%20FILTER%20SCHEMATIC%20INISH%20TAAHMTA%20(EXTRACTED).svg">vectors</a></sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/1RBYXSV7TMDXcv9GIndg1aHA0bZexf0rK/view" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=1RBYXSV7TMDXcv9GIndg1aHA0bZexf0rK" width="240" alt="1976 - M464 - DOC - 464-02-001 - TWO COMPONENTS WITH THE PARALLEL SUM EQUALLING THE.png"></a><br><sub>1976|9 · DOC - 464-02-001 - TWO COMPONENTS WITH THE PARALLEL SUM EQUALLING THE · <a href="SCHEMATICS/1976%20-%20M464%20-%20DOC%20-%20464-02-001%20-%20TWO%20COMPONENTS%20WITH%20THE%20PARALLEL%20SUM%20EQUALLING%20THE%20(EXTRACTED).svg">vectors</a></sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKY2pYMkdMZjB4Slk/view?resourcekey=0-DDDH3_JMTr2LdWM4-GYAVw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKY2pYMkdMZjB4Slk&resourcekey=0-DDDH3_JMTr2LdWM4-GYAVw" width="240" alt="1976 - M464 - HIGH PASS LOW PASS FILTER COMPONENT LAYOUT.jpg"></a><br><sub>1976|9 · HIGH PASS LOW PASS FILTER COMPONENT LAYOUT · <a href="SCHEMATICS/1976%20-%20M464%20-%20HIGH%20PASS%20LOW%20PASS%20FILTER%20COMPONENT%20LAYOUT%20(EXTRACTED).svg">vectors</a></sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKalpmNWY3Mm43cDQ/view?resourcekey=0-LPwpCBlnQDh11fW3AemyLw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKalpmNWY3Mm43cDQ&resourcekey=0-LPwpCBlnQDh11fW3AemyLw" width="240" alt="1977 - M464 - 464-01-001 - HIGH PASS LOW PASS FILTER FUNCTIONAL DIAGRAM FIMISH.jpg"></a><br><sub>1977|10 · 464-01-001 - HIGH PASS LOW PASS FILTER FUNCTIONAL DIAGRAM FIMISH · <a href="SCHEMATICS/1977%20-%20M464%20-%20464-01-001%20-%20HIGH%20PASS%20LOW%20PASS%20FILTER%20FUNCTIONAL%20DIAGRAM%20FIMISH%20(EXTRACTED).svg">vectors</a></sub></td></tr>
</table>

## Pinouts

_Pinouts: the RTO-144 edge connector (connection sheets, connector layouts and rear views, strapping), and the pinout / connections drawings made from them. The module root `RTO.json` lists every contact._

<table>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKTzJKb3ZqdlUxckE/view?resourcekey=0-s6gnByU90w8qFfRCHAQ6Ow" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKTzJKb3ZqdlUxckE&resourcekey=0-s6gnByU90w8qFfRCHAQ6Ow" width="240" alt="1976 - M464 - 27 - Pin Out.jpg"></a><br><sub>1976|9 · 27 - Pin Out · <a href="PINOUTS/1976%20-%20M464%20-%2027%20-%20Pin%20Out%20(EXTRACTED).svg">vectors</a></sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKVldzT3RfeGw3dkE/view?resourcekey=0-LD-e1QojQoGO5hIjfZ6Eww" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKVldzT3RfeGw3dkE&resourcekey=0-LD-e1QojQoGO5hIjfZ6Eww" width="240" alt="1976 - M464 - P4 - Pin Out.jpg"></a><br><sub>1976|9 · P4 - Pin Out · <a href="PINOUTS/1976%20-%20M464%20-%20P4%20-%20Pin%20Out%20(EXTRACTED).svg">vectors</a></sub></td></tr>
</table>

## RTO-144 pinout

Read from 1 pinout picture (each its own `.pinout.json` beside it); the RTO-144 sheet agrees on 8 of 8 contacts. Every contact, with where it was read: [`RTO.json`](RTO.json). How a picture becomes this table: [the pinout decision tree](https://github.com/WardBeckSystems/WBSps.ca/blob/main/2026/Engineering/PINOUTS.md).

| Pin | Signal | Kind | Port | Confidence |
|---|---|---|---|---|
| 1 | INPUT A | in | INPUT | high |
| 2 | INPUT B | in | INPUT | high |
| 3 | INPUT SHIELD | shield | INPUT shield | high |
| 15 | OUTPUT A | out | OUTPUT | high |
| 16 | OUTPUT B | out | OUTPUT | high |
| 19 | -25 V | power |  | high |
| 20 | COMMON | ground |  | high |
| 21 | +25 V | power |  | high |
| 22 | CHASSIS GROUND | ground |  | high |

## Manuals and documents

_Manuals and manual pages, price lists, RTO-144 connection sheets, brochures, ads and other document scans._

<table>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKNl8wem1kMDA4Wk0/view?resourcekey=0-MKhFQyznnIB2ukc4yHgxTA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKNl8wem1kMDA4Wk0&resourcekey=0-MKhFQyznnIB2ukc4yHgxTA" width="240" alt="1976 - M464 - 01 - Cover.jpg"></a><br><sub>1976|9 · 01 - Cover</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKa0g2cm9tbHRmdG8/view?resourcekey=0-dfHayqO2YFJdLUJkKfEoBQ" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKa0g2cm9tbHRmdG8&resourcekey=0-dfHayqO2YFJdLUJkKfEoBQ" width="240" alt="1976 - M464 - 02.jpg"></a><br><sub>1976|9 · 02</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKakN6UW1zNkpMMFk/view?resourcekey=0-W2wCa6dt2Y8cV_2Uh-v6kA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKakN6UW1zNkpMMFk&resourcekey=0-W2wCa6dt2Y8cV_2Uh-v6kA" width="240" alt="1976 - M464 - 04 - Warranty.jpg"></a><br><sub>1976|9 · 04 - Warranty</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKYTgzS3NRQ2JkVkU/view?resourcekey=0-cgOQ2_I1XnSB6KDuJc0rgw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKYTgzS3NRQ2JkVkU&resourcekey=0-cgOQ2_I1XnSB6KDuJc0rgw" width="240" alt="1976 - M464 - 05 - 1-1.jpg"></a><br><sub>1976|9 · 05 - 1-1</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKZjdpSmRZbG5lTVk/view?resourcekey=0-nG_f-NSvT7qF3jHk_amFCw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKZjdpSmRZbG5lTVk&resourcekey=0-nG_f-NSvT7qF3jHk_amFCw" width="240" alt="1976 - M464 - 06 - 2-1.jpg"></a><br><sub>1976|9 · 06 - 2-1</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKeGpsbE1jSlMtMFk/view?resourcekey=0-TZH9QLYfvBFFoHI5q8Sqqw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKeGpsbE1jSlMtMFk&resourcekey=0-TZH9QLYfvBFFoHI5q8Sqqw" width="240" alt="1976 - M464 - 07 - 2-2.jpg"></a><br><sub>1976|9 · 07 - 2-2</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKaG8tMTN6ZmQtbnM/view?resourcekey=0-f7XAfoMbMsdzyGUW0vEJfQ" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKaG8tMTN6ZmQtbnM&resourcekey=0-f7XAfoMbMsdzyGUW0vEJfQ" width="240" alt="1976 - M464 - 08 - 3-1.jpg"></a><br><sub>1976|9 · 08 - 3-1</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKZHFaX3ZxRWxjZTA/view?resourcekey=0-xXE3uI57L6hRlP22kCGCJg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKZHFaX3ZxRWxjZTA&resourcekey=0-xXE3uI57L6hRlP22kCGCJg" width="240" alt="1976 - M464 - 09 - 4-1.jpg"></a><br><sub>1976|9 · 09 - 4-1</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKeXM0WnNva3M5b2c/view?resourcekey=0-gH29aXVNYTdMtkSUfxpkIg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKeXM0WnNva3M5b2c&resourcekey=0-gH29aXVNYTdMtkSUfxpkIg" width="240" alt="1976 - M464 - 10 - 4-2.jpg"></a><br><sub>1976|9 · 10 - 4-2</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKMTNmQ3ZUQmJzOHM/view?resourcekey=0-ZEL21TL6NDggS-0QQ7vHig" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKMTNmQ3ZUQmJzOHM&resourcekey=0-ZEL21TL6NDggS-0QQ7vHig" width="240" alt="1976 - M464 - 11 - 4-3.jpg"></a><br><sub>1976|9 · 11 - 4-3</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKUXRwamdaLXVFb0k/view?resourcekey=0-CZuNc1g3FBY7KbkRbvbL3w" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKUXRwamdaLXVFb0k&resourcekey=0-CZuNc1g3FBY7KbkRbvbL3w" width="240" alt="1976 - M464 - 12 - 4-4.jpg"></a><br><sub>1976|9 · 12 - 4-4</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKMElIV1FUNktqUWc/view?resourcekey=0-GoG-3XjeF0URJMed6bbsuQ" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKMElIV1FUNktqUWc&resourcekey=0-GoG-3XjeF0URJMed6bbsuQ" width="240" alt="1976 - M464 - 13 - 4-5.jpg"></a><br><sub>1976|9 · 13 - 4-5</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKbEo2YUZUZ2Zaa0k/view?resourcekey=0-8swYsYxjtPSE08TPowIfWA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKbEo2YUZUZ2Zaa0k&resourcekey=0-8swYsYxjtPSE08TPowIfWA" width="240" alt="1976 - M464 - 14 - 4-6.jpg"></a><br><sub>1976|9 · 14 - 4-6</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKSlhlaDFnSHU2OXc/view?resourcekey=0-Ijpj9OZx2Go2-oyA0gQtxA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKSlhlaDFnSHU2OXc&resourcekey=0-Ijpj9OZx2Go2-oyA0gQtxA" width="240" alt="1976 - M464 - 15 - 4-109.jpg"></a><br><sub>1976|9 · 15 - 4-109</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKWEdMNXd1VFBxYU0/view?resourcekey=0-kkpTQveAcMpXW0F8dGtTmg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKWEdMNXd1VFBxYU0&resourcekey=0-kkpTQveAcMpXW0F8dGtTmg" width="240" alt="1976 - M464 - 16 - 4-109-1-1.jpg"></a><br><sub>1976|9 · 16 - 4-109-1-1</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKTUxVUnJqQXRkdFk/view?resourcekey=0-trZcOeZIOPdkA9uxAKuQ0g" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKTUxVUnJqQXRkdFk&resourcekey=0-trZcOeZIOPdkA9uxAKuQ0g" width="240" alt="1976 - M464 - 17 - 4-109-1-2.jpg"></a><br><sub>1976|9 · 17 - 4-109-1-2</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKUFE1Xy1Ga0QtcW8/view?resourcekey=0-VHQ3Q07aqSDJwAGxPec6Kg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKUFE1Xy1Ga0QtcW8&resourcekey=0-VHQ3Q07aqSDJwAGxPec6Kg" width="240" alt="1976 - M464 - 18 - 4-109-2-1.jpg"></a><br><sub>1976|9 · 18 - 4-109-2-1</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKNzZOS2FHU3RTYWs/view?resourcekey=0-orcTHaJoT7_kChIhDOpetA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKNzZOS2FHU3RTYWs&resourcekey=0-orcTHaJoT7_kChIhDOpetA" width="240" alt="1976 - M464 - 19 - 4-109-2-2.jpg"></a><br><sub>1976|9 · 19 - 4-109-2-2</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKYnlaMWFJaUNzcGM/view?resourcekey=0-L00rBbbEV-hMT0eeFBFxTA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKYnlaMWFJaUNzcGM&resourcekey=0-L00rBbbEV-hMT0eeFBFxTA" width="240" alt="1976 - M464 - 20 - 4-109-2-3.jpg"></a><br><sub>1976|9 · 20 - 4-109-2-3</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKT2hOTjRDTGVyNEU/view?resourcekey=0-eXf0wvVcTGMoSZ_JogDBHg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKT2hOTjRDTGVyNEU&resourcekey=0-eXf0wvVcTGMoSZ_JogDBHg" width="240" alt="1976 - M464 - 21 - 4-109-2-4.jpg"></a><br><sub>1976|9 · 21 - 4-109-2-4</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKSGhzeVF0ajdsXzQ/view?resourcekey=0-G_RoPJyjFngqR2YYPmjCcw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKSGhzeVF0ajdsXzQ&resourcekey=0-G_RoPJyjFngqR2YYPmjCcw" width="240" alt="1976 - M464 - 22 - 4-109-2-5.jpg"></a><br><sub>1976|9 · 22 - 4-109-2-5</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKSnZTaXZFQVhQc1U/view?resourcekey=0-UzryoD6YfEMlgva1kt29hw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKSnZTaXZFQVhQc1U&resourcekey=0-UzryoD6YfEMlgva1kt29hw" width="240" alt="1976 - M464 - HP LP Filter (2).pdf"></a><br><sub>1976|9 · HP LP Filter (2)</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKenZGQWF6TGJvSzQ/view?resourcekey=0--UsrbAUjd8i0haU2w6MqOQ" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKenZGQWF6TGJvSzQ&resourcekey=0--UsrbAUjd8i0haU2w6MqOQ" width="240" alt="1976 - M464 - HP LP Filter.pdf"></a><br><sub>1976|9 · HP LP Filter</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKUWppQmF5ZXNqQ1k/view?resourcekey=0-O0WE6dy5o-1yYG_deT8hBA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKUWppQmF5ZXNqQ1k&resourcekey=0-O0WE6dy5o-1yYG_deT8hBA" width="240" alt="1976 - M464 - P1 - Cover.jpg"></a><br><sub>1976|9 · P1 - Cover</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKbmhZUU5DMGk5OWs/view?resourcekey=0-x16igDJspY3rJw9-2o2gIg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKbmhZUU5DMGk5OWs&resourcekey=0-x16igDJspY3rJw9-2o2gIg" width="240" alt="1976 - M464 - P2 - 1-1.jpg"></a><br><sub>1976|9 · P2 - 1-1</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKODZOaG4zQjF4SkE/view?resourcekey=0-2Tbm-92mrYizdvwV01YW7w" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKODZOaG4zQjF4SkE&resourcekey=0-2Tbm-92mrYizdvwV01YW7w" width="240" alt="1976 - M464 - P3 - 3-1.jpg"></a><br><sub>1976|9 · P3 - 3-1</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKVUU4dDRpX2dVSFE/view?resourcekey=0-fmC865JtyOQapyPDCaAGRQ" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKVUU4dDRpX2dVSFE&resourcekey=0-fmC865JtyOQapyPDCaAGRQ" width="240" alt="1976 - M464 - WBS test procedures 0.jpeg"></a><br><sub>1976|9 · WBS test procedures 0</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKcGZMV1FPRjVBZEk/view?resourcekey=0-LOFXFfYMhz9cZdSeroLYZA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKcGZMV1FPRjVBZEk&resourcekey=0-LOFXFfYMhz9cZdSeroLYZA" width="240" alt="1976 - M464 - WBS test procedures 1.jpeg"></a><br><sub>1976|9 · WBS test procedures 1</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKWExwWlJZcmR3WDQ/view?resourcekey=0-eynqXLxq12tZkIV5LJBhzA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKWExwWlJZcmR3WDQ&resourcekey=0-eynqXLxq12tZkIV5LJBhzA" width="240" alt="1976 - M464 - WBS test procedures 2.jpeg"></a><br><sub>1976|9 · WBS test procedures 2</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKd2gyMnBUUGZmQTg/view?resourcekey=0-MjQ6yxSaHvJwSJJsXmGtfQ" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKd2gyMnBUUGZmQTg&resourcekey=0-MjQ6yxSaHvJwSJJsXmGtfQ" width="240" alt="1976 - M464 - WBS test procedures 3.jpeg"></a><br><sub>1976|9 · WBS test procedures 3</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKM0ZmblJVMHEtbnc/view?resourcekey=0-9sZ0__yVXZVtNdIYT5mFwg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKM0ZmblJVMHEtbnc&resourcekey=0-9sZ0__yVXZVtNdIYT5mFwg" width="240" alt="1977 - M464 - 03 - TOC.jpg"></a><br><sub>1977|10 · 03 - TOC</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKTVBUMTFlVlhZTzQ/view?resourcekey=0-jvWFawMwI8VYT-bcRfwtHg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKTVBUMTFlVlhZTzQ&resourcekey=0-jvWFawMwI8VYT-bcRfwtHg" width="240" alt="19xx - M464 - DOC - 006.jpg"></a><br><sub>19xx - M464 - DOC - 006</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKNGVOSFF1bC1MMmM/view?resourcekey=0-8tTfYusCHrA28knsEn_8JA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKNGVOSFF1bC1MMmM&resourcekey=0-8tTfYusCHrA28knsEn_8JA" width="240" alt="19xx - M464 - DOC - 010.jpg"></a><br><sub>19xx - M464 - DOC - 010</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKMExPenZ1OTAxV0E/view?resourcekey=0-ahFs-5zdNm4SW4RQyGKK4g" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKMExPenZ1OTAxV0E&resourcekey=0-ahFs-5zdNm4SW4RQyGKK4g" width="240" alt="19xx - M464 - DOC - 014.jpg"></a><br><sub>19xx - M464 - DOC - 014</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKcDV5R29Vc0pEQWs/view?resourcekey=0-32OeuYY1JB48dWStBmHEBw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKcDV5R29Vc0pEQWs&resourcekey=0-32OeuYY1JB48dWStBmHEBw" width="240" alt="19xx - M464 - DOC - 023.jpg"></a><br><sub>19xx - M464 - DOC - 023</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKR3BpMUtqaEpUOTg/view?resourcekey=0-GePoDQlHfxFcw_1u_VgSCw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKR3BpMUtqaEpUOTg&resourcekey=0-GePoDQlHfxFcw_1u_VgSCw" width="240" alt="19xx - M464 - DOC - 026.jpg"></a><br><sub>19xx - M464 - DOC - 026</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/1QGwOMfnFBPhzQXIxu21VU66PeQc8Qeej/view" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=1QGwOMfnFBPhzQXIxu21VU66PeQc8Qeej" width="240" alt="19xx - M464 - DOC - INPUT OUTPUT MON INPUT MON CUE SPEAKER TRANSFCRMERS.jpg"></a><br><sub>19xx - M464 - DOC - INPUT OUTPUT MON INPUT MON CUE SPEAKER TRANSFCRMERS</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKZkNNdlpPcmU0LWM/view?resourcekey=0-E_vjEZWmjI4zXD8QpocRHw" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKZkNNdlpPcmU0LWM&resourcekey=0-E_vjEZWmjI4zXD8QpocRHw" width="240" alt="19xx - M464 - DOC - L2042 - P05.jpg"></a><br><sub>19xx - M464 - DOC - L2042 - P05</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKTUh0YVhIN2hjclE/view?resourcekey=0-5BbrKsqSG2dcdybh1CqyLg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKTUh0YVhIN2hjclE&resourcekey=0-5BbrKsqSG2dcdybh1CqyLg" width="240" alt="19xx - M464 - DOC - L3242 - P05.jpg"></a><br><sub>19xx - M464 - DOC - L3242 - P05</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKVFBmX2RsU3ZpMmM/view?resourcekey=0-HxddTfJi3Ix3Fjzk54EIdQ" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKVFBmX2RsU3ZpMmM&resourcekey=0-HxddTfJi3Ix3Fjzk54EIdQ" width="240" alt="19xx - M464 - DOC - R1400-R2400 - 007.jpg"></a><br><sub>19xx - M464 - DOC - R1400-R2400 - 007</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/1ZzGLm0TlvSp-ghXGhACAF51fhZ3OT8Vz/view" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=1ZzGLm0TlvSp-ghXGhACAF51fhZ3OT8Vz" width="240" alt="19xx - M464 - DOC - SSUEL REVISION MON.jpg"></a><br><sub>19xx - M464 - DOC - SSUEL REVISION MON</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKdmpaZjFWeEllNmM/view?resourcekey=0-ufrNGxtkGGlrNRECtoy_zA" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKdmpaZjFWeEllNmM&resourcekey=0-ufrNGxtkGGlrNRECtoy_zA" width="240" alt="19xx - M464 - DOC - WBS sales 20.jpg"></a><br><sub>19xx - M464 - DOC - WBS sales 20</sub></td></tr>
<tr><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKM1h5cVBiQ2NDX00/view?resourcekey=0-Q1Fhe5LtCKYWlZQKanAF0g" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKM1h5cVBiQ2NDX00&resourcekey=0-Q1Fhe5LtCKYWlZQKanAF0g" width="240" alt="19xx - M464 - DOC - WBS sales a.jpg"></a><br><sub>19xx - M464 - DOC - WBS sales a</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKRmdIVzRDNXNYQlk/view?resourcekey=0-zX-_hZ_3-xCQPnLlS0iSrg" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKRmdIVzRDNXNYQlk&resourcekey=0-zX-_hZ_3-xCQPnLlS0iSrg" width="240" alt="19xx - M464 - DOC - WBS sales b.jpg"></a><br><sub>19xx - M464 - DOC - WBS sales b</sub></td><td align="center" valign="top" width="33%"><a href="https://drive.google.com/file/d/0B0yR0b5T78SKcG1kY0s4b1R4YUU/view?resourcekey=0-H7t6-y8PwZh6pcIsRcimaQ" title="Open on Google Drive"><img src="https://drive.google.com/thumbnail?sz=w480&id=0B0yR0b5T78SKcG1kY0s4b1R4YUU&resourcekey=0-H7t6-y8PwZh6pcIsRcimaQ" width="240" alt="19xx - M464 - Frequency Response Characteristics M464 High-Pass Low-Pass Filter.jpg"></a><br><sub>19xx - M464 - Frequency Response Characteristics M464 High-Pass Low-Pass Filter</sub></td></tr>
</table>

## KiCad

_KiCad: a schematic (`.kicad_sch`) traced from each electrical drawing, the module symbol (`.kicad_sym`) and its PCB projects. Traced schematics are the line work only: redraw with symbols over them._

- [1976 - M464 - 464-02-001 - TWO COMPONENTS WITH THE PARALLEL SUM EQUALLING THE.kicad_sch](KICAD/1976%20-%20M464%20-%20464-02-001%20-%20TWO%20COMPONENTS%20WITH%20THE%20PARALLEL%20SUM%20EQUALLING%20THE.kicad_sch)
- [1976 - M464 - DOC - 464-02-001 - HIGH PASS LOW PASS FILTER SCHEMATIC INISH TAAHMTA.kicad_sch](KICAD/1976%20-%20M464%20-%20DOC%20-%20464-02-001%20-%20HIGH%20PASS%20LOW%20PASS%20FILTER%20SCHEMATIC%20INISH%20TAAHMTA.kicad_sch)
- [1976 - M464 - DOC - 464-02-001 - TWO COMPONENTS WITH THE PARALLEL SUM EQUALLING THE.kicad_sch](KICAD/1976%20-%20M464%20-%20DOC%20-%20464-02-001%20-%20TWO%20COMPONENTS%20WITH%20THE%20PARALLEL%20SUM%20EQUALLING%20THE.kicad_sch)
- [1976 - M464 - HP LP Filter p24.kicad_sch](KICAD/1976%20-%20M464%20-%20HP%20LP%20Filter%20p24.kicad_sch)
- [1977 - 464-01-001 - HIGH PASS - LOW PASS FILTER FUNCTIONAL DIAGRAM - DRN DNM - ISS 1.kicad_sch](KICAD/1977%20-%20464-01-001%20-%20HIGH%20PASS%20-%20LOW%20PASS%20FILTER%20FUNCTIONAL%20DIAGRAM%20-%20DRN%20DNM%20-%20ISS%201.kicad_sch)
- [1977 - M464 - 464-01-001 - HIGH PASS LOW PASS FILTER FUNCTIONAL DIAGRAM FIMISH.kicad_sch](KICAD/1977%20-%20M464%20-%20464-01-001%20-%20HIGH%20PASS%20LOW%20PASS%20FILTER%20FUNCTIONAL%20DIAGRAM%20FIMISH.kicad_sch)
- [M464 - 24 - Functional Diagram.kicad_sch](KICAD/M464%20-%2024%20-%20Functional%20Diagram.kicad_sch)
- [M464.kicad_sym](KICAD/M464.kicad_sym)

## Software

- [cpp](SOFTWARE/cpp)
- [plugin](SOFTWARE/plugin): the Rust VST3 / CLAP plugin and Audio Tap insert

---
Archive: Anthony Peter Kuzub · [WBSps.ca](https://WBSps.ca) · [WBSps.Kuzub.com](https://WBSps.Kuzub.com)
