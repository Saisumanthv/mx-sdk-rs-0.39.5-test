#[test]
fn call_other_shard_1_go() {
    drt_sc_scenario::run_go("scenarios/call_other_shard-1.scen.json");
}

#[test]
fn call_other_shard_2_go() {
    drt_sc_scenario::run_go("scenarios/call_other_shard-2.scen.json");
}

#[ignore]
#[test]
fn call_other_shard_insufficient_gas_go() {
    drt_sc_scenario::run_go("scenarios/call_other_shard-insufficient-gas.scen.json");
}

#[test]
fn changeboard_go() {
    drt_sc_scenario::run_go("scenarios/changeBoard.scen.json");
}

#[test]
fn changequorum_go() {
    drt_sc_scenario::run_go("scenarios/changeQuorum.scen.json");
}

#[test]
fn changequorum_toobig_go() {
    drt_sc_scenario::run_go("scenarios/changeQuorum_tooBig.scen.json");
}

#[test]
fn deployadder_err_go() {
    drt_sc_scenario::run_go("scenarios/deployAdder_err.scen.json");
}

#[test]
fn deployadder_then_call_go() {
    drt_sc_scenario::run_go("scenarios/deployAdder_then_call.scen.json");
}

#[test]
fn deployfactorial_go() {
    drt_sc_scenario::run_go("scenarios/deployFactorial.scen.json");
}

#[test]
fn deployothermultisig_go() {
    drt_sc_scenario::run_go("scenarios/deployOtherMultisig.scen.json");
}

#[test]
fn deploy_duplicate_bm_go() {
    drt_sc_scenario::run_go("scenarios/deploy_duplicate_bm.scen.json");
}

#[test]
fn remove_everyone_go() {
    drt_sc_scenario::run_go("scenarios/remove_everyone.scen.json");
}

// TODO: investigate gas issue
#[ignore]
#[test]
fn senddcdt_go() {
    drt_sc_scenario::run_go("scenarios/sendDcdt.scen.json");
}

#[test]
fn upgrade_go() {
    drt_sc_scenario::run_go("scenarios/upgrade.scen.json");
}

#[test]
fn upgrade_from_source_go() {
    drt_sc_scenario::run_go("scenarios/upgrade_from_source.scen.json");
}
