#[test]
fn managed_error_message_go() {
    drt_sc_scenario::run_go("scenarios/managed_error_message.scen.json");
}

#[test]
fn sc_format_go() {
    drt_sc_scenario::run_go("scenarios/sc_format.scen.json");
}
