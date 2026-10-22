use assert_cmd::Command;

/// TODO(week 2): full round-trip test —
/// 1. compile fixture hello.c through `ccbolt wrap gcc`
/// 2. delete the .o, compile again
/// 3. assert second run prints "cached" and output is byte-identical
#[test]
fn stats_runs_on_empty_cache() {
    let mut cmd = Command::cargo_bin("ccbolt").unwrap();
    cmd.arg("stats").assert().success();
}
