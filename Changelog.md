### 05.08.2020 -> 17.08.2020
#### _4f212d33d95d3c9f3a09df8c032bdb0957a70e46 -> c46b32d73087ed2afc6c1ac3e78535cac37e3fa9_
[vm] Rewrite the PublishOption to become a native rust resource 

[vm] More logging 

[Libra Framework][Move Prover] Revised Error Model & Specification Cleanup

[vm] Cleanup existing resource error 

[dead code][vm] Kill Unused Entries Check

[move-prover] Fixing unsoundness in opaque aborts_if specs.
 
[language][vm] rework GlobalValue to fix the deletion bug 

[move-prover] Changes pulled out of current Errors work. 

[vm] Make unexpected verification errors invariant violations 

[vm] Fix issue in frame resolution for stack traces 

[language] Concurrent loader test

[libra framework][move prover] enable calling of Move native functions from specs 

[breaking][libra framework] decouple AccountLimits and Balance

[move-prover] Prover changes and fix resulting from work on error model. 
[breaking][vm] Add sender signer to `AdminScript`s

[vm] Bolstered prologue/epilogue VM status special case

[logging] add all log levels to structured logging



### 29.07.2020 -> 05.08.2020
#### _93c0d0e32d5e0e81bb33c200dac4d8e49931ca1b -> 4f212d33d95d3c9f3a09df8c032bdb0957a70e46_

[logging] Keep important fields from being lost.

[move prover] improvements to pure move function call feature.

[language][vm] fix inconsistency in calculation of intrinsic gas cost.
 
[move-prover] Restore global timeout flag and leverage new boogie features for per-procedure settings.

[libra framework] Derive `Arbitrary` on `ScriptCall`. Use to generate a transaction fuzzer. 

[Libra Framework][Specs] Use global invariants where appropriate. 

[Move stdlib] Make the stdlib tool work from the root of the repo.

[Move stdlib] add basic test to ensure that generated files are up to date.

[vm] Added offset information for two TODO execution errors.