### 17.08.2020 -> 15.09.2020
#### _c46b32d73087ed2afc6c1ac3e78535cac37e3fa9 -> b9c785f592492c16c0365e62d0b0c0945003ac0f_

[breaking][vm][move stdlib] Use Errors module everywhere. Merge epilogues. Fix bugs.

[fuzzing] adding two fuzzers to find incorrect deserialization.

[language][tools] Move CLI.

[fuzz] types/proof fuzzers.

[vm] add a comment about running the failure epilogue.

[move prover] implement async framework running multiple boogie instances.

[move-prover] Avoiding unnecessary writebacks. 

[move-prover] Copy Propagation and Transformation Pipeline.

[bug fix][bytecode verifier] Fixed canonical initial state.

[breaking][vm] Merge KeptVMStatus for Verifier/Deserialization errors.

[language] use StructTag instead of TypeTag in TransactionEffect.

refactored LibraTimestamp, LibraConfig, and LibraSystem.

[Move script] tiered_mint: fix quotes in doc comment.

[move source language] Small error message cleanup.

[vm] fix execution count for user transactions.

[vm] Fix tracking for VM_MAX_VALUE_DEPTH_REACHED.

[move-prover] Bug fixes for `old(..)` + new `update_field` builtin function.

[dependencies] Update summaries.

[dependencies] Bump the dalek libraries.

[move-prover] Small changes to ValidatorConfig, LibraSystem.

[dependency] update serde-generate to 0.13.0.

[types] update comment about transaction expiration times.

[types] fix warning.

[breaking][move] Fix network address names.

[dependency] update serde-generate to 0.12.2.

[move prover] summary-based analysis to compute possibly-packed types.

[move-prover] process functions in topological order in FunctionTargetPipeline.

[vm][data cache] Small cleanup of effects return type.

[vm] Fix native function errors.

[Move Prover] Optimizations for global invariants/removal of some timeouts.

[RFC][bytecode tools] Add module linking/layout API definition and compatibility checking.

[Move prover] Added specs to LibraConfig.
fixed specs in Errors.
More specs.

[language] reorganize testing infrastructures.

[spec] Add data structure spec.

[libra framework] make Signature::ed25519_verify non-aborting.

[libra-vm] Add simple debugging interface to the VM.

[vm] Make TYPE_RESOLUTION_FAILURE an invariant violation.

Store new cache key immediately before starting long jobs.

[libra framework] Add tests back in for account limits, get coverage up.

[libra framework] add some comments to FixedPoint32 about precision.

Set up cache file as a daily latch for code coverage and fresh circle ci build images.

[vm] Move INVALID_DATA to invariant violations.

Parameterize build, remove redundant step.

[vm] Remove unused OUT_OF_BOUNDS_INDEX code.

[management] Prevent from sending no change validator configs.

[logging] add unique ids to every log request.

[Move stdlib] fix typo in docs.

[language] capitalize Move (the language) in comments and logs.

[dead code][vm] Remove dead status codes.

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