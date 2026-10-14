# Q3: Blacklist form versus replacement

Finding F1: a block-form canonical blacklist copied into output is hidden by the subsequently appended canonical value during parse_text. The final check counts one coalesced parameter and returns success. A block alone returns InvalidBlacklistForm. The successful mixed result still contains two assignments. Baseline Node::insert rejected the duplicate value; preserving the narrow form check now requires checking unmerged occurrences before replacement. This does not require restoring general duplicate rejection.

Evidence: mixed.probe.json; block-only.probe.json; 09380d356886842b8987b602ed2ef7daaefec4f3:tools/bootconfig/src/lib.rs:155; 09380d356886842b8987b602ed2ef7daaefec4f3:tools/bootconfig/src/parsers/bootconfig.rs:33; 515d9ae99a8800eb0373d7f6e33a41183435c506:tools/bootconfig/src/lib.rs:68.
