#[test]
fn bid_first_go() {
    drt_sc_scenario::run_go("scenarios/bid_first.scen.json");
}

#[test]
fn bid_second_max_go() {
    drt_sc_scenario::run_go("scenarios/bid_second_max.scen.json");
}

#[test]
fn bid_second_ok_go() {
    drt_sc_scenario::run_go("scenarios/bid_second_ok.scen.json");
}

#[test]
fn bid_second_too_low_go() {
    drt_sc_scenario::run_go("scenarios/bid_second_too_low.scen.json");
}

#[test]
fn bid_siring_auction_go() {
    drt_sc_scenario::run_go("scenarios/bid_siring_auction.scen.json");
}

#[test]
fn create_and_auction_gen_zero_kitty_go() {
    drt_sc_scenario::run_go("scenarios/create_and_auction_gen_zero_kitty.scen.json");
}

#[test]
fn create_sale_auction_not_owner_go() {
    drt_sc_scenario::run_go("scenarios/create_sale_auction_not_owner.scen.json");
}

#[test]
fn create_sale_auction_ok_go() {
    drt_sc_scenario::run_go("scenarios/create_sale_auction_ok.scen.json");
}

#[test]
fn create_siring_auction_not_owner_go() {
    drt_sc_scenario::run_go("scenarios/create_siring_auction_not_owner.scen.json");
}

#[test]
fn create_siring_auction_ok_go() {
    drt_sc_scenario::run_go("scenarios/create_siring_auction_ok.scen.json");
}

#[test]
fn end_auction_no_bids_go() {
    drt_sc_scenario::run_go("scenarios/end_auction_no_bids.scen.json");
}

#[test]
fn end_auction_second_bid_max_early_go() {
    drt_sc_scenario::run_go("scenarios/end_auction_second_bid_max_early.scen.json");
}

#[test]
fn end_auction_second_bid_ok_early_go() {
    drt_sc_scenario::run_go("scenarios/end_auction_second_bid_ok_early.scen.json");
}

#[test]
fn end_auction_second_bid_ok_late_go() {
    drt_sc_scenario::run_go("scenarios/end_auction_second_bid_ok_late.scen.json");
}

#[test]
fn end_siring_auction_go() {
    drt_sc_scenario::run_go("scenarios/end_siring_auction.scen.json");
}

#[test]
fn init_go() {
    drt_sc_scenario::run_go("scenarios/init.scen.json");
}