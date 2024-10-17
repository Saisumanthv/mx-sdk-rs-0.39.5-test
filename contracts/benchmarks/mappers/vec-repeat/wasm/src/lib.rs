// Code generated by the drt-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]
#![feature(alloc_error_handler, lang_items)]

drt_sc_wasm_adapter::allocator!();
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    vec_repeat
    (
        add
        count
        remove
        bench
        add_struct
        count_struct
        remove_struct
        bench_struct
    )
}

drt_sc_wasm_adapter::empty_callback! {}