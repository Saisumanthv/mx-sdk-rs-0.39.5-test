drt_sc::imports!();

#[drt_sc::module]
pub trait OnlyOwnerDerivedTestModule {
    #[view]
    fn call_derived_not_owner_only(&self) {}
}