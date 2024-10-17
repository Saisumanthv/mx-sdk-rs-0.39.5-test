// Code generated by the drt-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Async Callback:                       1
// Total number of exported functions:   8

#![no_std]
#![feature(alloc_error_handler, lang_items)]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    proxy_test_first
    (
        deploySecondContract
        upgradeSecondContract
        forwardToOtherContract
        forwardToOtherContractWithCallback
        messageOtherContract
        messageOtherContractWithCallback
        callBack
    )
}
