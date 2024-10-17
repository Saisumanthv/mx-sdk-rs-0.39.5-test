#![no_std]

drt_sc::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[drt_sc::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}
}
