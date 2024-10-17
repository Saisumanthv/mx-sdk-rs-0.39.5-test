#![no_std]

drt_sc::imports!();

#[drt_sc::contract]
pub trait RewaDcdtSwap: drt_sc_modules::pause::PauseModule {
    #[init]
    fn init(&self, wrapped_rewa_token_id: TokenIdentifier) {
        self.wrapped_rewa_token_id().set(&wrapped_rewa_token_id);
    }

    // endpoints

    #[payable("REWA")]
    #[endpoint(wrapRewa)]
    fn wrap_rewa(&self) -> DcdtTokenPayment<Self::Api> {
        self.require_not_paused();

        let payment_amount = self.call_value().rewa_value();
        require!(payment_amount > 0u32, "Payment must be more than 0");

        let wrapped_rewa_token_id = self.wrapped_rewa_token_id().get();
        self.send()
            .dcdt_local_mint(&wrapped_rewa_token_id, 0, &payment_amount);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_dcdt(&caller, &wrapped_rewa_token_id, 0, &payment_amount);

        DcdtTokenPayment::new(wrapped_rewa_token_id, 0, payment_amount)
    }

    #[payable("*")]
    #[endpoint(unwrapRewa)]
    fn unwrap_rewa(&self) {
        self.require_not_paused();

        let (payment_token, payment_amount) = self.call_value().single_fungible_dcdt();
        let wrapped_rewa_token_id = self.wrapped_rewa_token_id().get();

        require!(payment_token == wrapped_rewa_token_id, "Wrong dcdt token");
        require!(payment_amount > 0u32, "Must pay more than 0 tokens!");
        require!(
            payment_amount <= self.get_locked_rewa_balance(),
            "Contract does not have enough funds"
        );

        self.send()
            .dcdt_local_burn(&wrapped_rewa_token_id, 0, &payment_amount);

        // 1 wrapped rEWA = 1 rEWA, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send().direct_rewa(&caller, &payment_amount);
    }

    #[view(getLockedRewaBalance)]
    fn get_locked_rewa_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&RewaOrDcdtTokenIdentifier::rewa(), 0)
    }

    #[view(getWrappedRewaTokenId)]
    #[storage_mapper("wrappedRewaTokenId")]
    fn wrapped_rewa_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}