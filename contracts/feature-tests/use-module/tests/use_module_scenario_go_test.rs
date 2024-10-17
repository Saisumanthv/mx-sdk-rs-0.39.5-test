#[test]
fn use_module_claim_developer_rewards_go() {
    drt_sc_scenario::run_go("scenarios/use_module_claim_developer_rewards.scen.json");
}

#[test]
fn use_module_dns_register_go() {
    drt_sc_scenario::run_go("scenarios/use_module_dns_register.scen.json");
}

#[test]
fn use_module_features_go() {
    drt_sc_scenario::run_go("scenarios/use_module_features.scen.json");
}

#[test]
fn use_module_internal_go() {
    drt_sc_scenario::run_go("scenarios/use_module_internal.scen.json");
}

#[test]
fn use_module_only_owner_go() {
    drt_sc_scenario::run_go("scenarios/use_module_only_owner.scen.json");
}

#[test]
fn use_module_only_admin_go() {
    drt_sc_scenario::run_go("scenarios/use_module_only_admin.scen.json");
}

#[test]
fn use_module_no_endpoint_go() {
    drt_sc_scenario::run_go("scenarios/use_module_no_endpoint.scen.json");
}

#[test]
fn use_module_pause_go() {
    drt_sc_scenario::run_go("scenarios/use_module_pause.scen.json");
}

#[test]
fn use_module_ongoing_operation_go() {
    drt_sc_scenario::run_go("scenarios/use_module_ongoing_operation_example.scen.json");
}
