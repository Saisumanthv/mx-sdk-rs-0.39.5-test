#[test]
fn generate_kitty_genes_go() {
    drt_sc_scenario::run_go("scenarios/generate-kitty-genes.scen.json");
}

#[test]
fn init_go() {
    drt_sc_scenario::run_go("scenarios/init.scen.json");
}
