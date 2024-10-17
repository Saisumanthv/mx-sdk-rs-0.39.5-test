mod dns_proxy {
    drt_sc::imports!();

    #[drt_sc::proxy]
    pub trait Dns {
        #[payable("REWA")]
        #[endpoint]
        fn register(&self, name: &ManagedBuffer);
    }
}

drt_sc::imports!();

/// Standard smart contract module that deals with registering usernames in a DNS contract.
///
/// Numbat usernames/herotags need to be requested by the beneficiary.
/// For a contract, this means that they need an endpoint via which to request a username from the DNS.
///
#[drt_sc::module]
pub trait DnsModule {
    #[proxy]
    fn dns_proxy(&self, to: ManagedAddress) -> dns_proxy::Proxy<Self::Api>;

    #[payable("REWA")]
    #[only_owner]
    #[endpoint(dnsRegister)]
    fn dns_register(&self, dns_address: ManagedAddress, name: ManagedBuffer) {
        let payment = self.call_value().rewa_value();
        self.dns_proxy(dns_address)
            .register(&name)
            .with_rewa_transfer(payment)
            .async_call()
            .call_and_exit()
    }
}