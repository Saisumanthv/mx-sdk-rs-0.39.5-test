// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

#![allow(unused)]

use drt_sc::{
    contract_base::ProxyObjBase,
    types::{BigInt, ManagedAddress},
};

use crate::module_1::VersionModule;

mod module_1 {
    drt_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait VersionModule: drt_sc::contract_base::ContractBase + Sized {
        fn version(&self) -> BigInt<Self::Api>;

        fn some_async(&self);

        fn callback(&self);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: drt_sc::contract_base::ContractBase {}

    impl<C> VersionModule for C
    where
        C: AutoImpl,
    {
        fn version(&self) -> BigInt<Self::Api> {
            BigInt::from(100)
        }

        fn some_async(&self) {
            panic!("wooo")
        }

        fn callback(&self) {}
    }

    impl<A> AutoImpl for drt_sc::contract_base::UniversalContractObj<A> where
        A: drt_sc::api::VMApi
    {
    }

    pub trait EndpointWrappers: VersionModule + drt_sc::contract_base::ContractBase {
        #[inline]
        fn call_version(&self) {
            drt_sc::api::CallValueApiImpl::check_not_payable(
                &Self::Api::call_value_api_impl(),
            );
            let result = self.version();
            drt_sc::io::finish_multi::<Self::Api, _>(&result)
        }

        fn call_some_async(&self) {
            self.some_async();
            drt_sc::io::finish_multi::<Self::Api, _>(&())
        }

        fn call(&self, fn_name: &str) -> bool {
            if match fn_name {
                "callBack" => {
                    self.callback();
                    return true;
                },
                "version" => {
                    self.call_version();
                    true
                },
                _other => false,
            } {
                return true;
            }
            false
        }
    }

    impl<A> EndpointWrappers for drt_sc::contract_base::UniversalContractObj<A> where
        A: drt_sc::api::VMApi
    {
    }

    pub struct AbiProvider {}

    impl drt_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = drt_sc::api::uncallable::UncallableApi;

        fn abi() -> drt_sc::abi::ContractAbi {
            drt_sc::abi::ContractAbi::default()
        }
    }

    pub trait ProxyTrait: drt_sc::contract_base::ProxyObjBase + Sized {
        fn version(
            &mut self,
        ) -> drt_sc::types::ContractCallNoPayment<Self::Api, BigInt<Self::Api>> {
            let ___address___ = self.extract_address();
            drt_sc::types::ContractCallNoPayment::new(___address___, "version")
        }
    }
}

mod sample_adder {
    drt_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait Adder:
        super::module_1::VersionModule + drt_sc::contract_base::ContractBase + Sized
    {
        fn init(&self, initial_value: &BigInt<Self::Api>) {
            self.set_sum(initial_value);
        }
        fn add(&self, value: BigInt<Self::Api>) -> SCResult<()> {
            let mut sum = self.get_sum();
            sum.add_assign(value);
            self.set_sum(&sum);
            Ok(())
        }
        fn get_sum(&self) -> BigInt<Self::Api>;
        fn set_sum(&self, sum: &BigInt<Self::Api>);
        fn add_version(&self) -> SCResult<()> {
            self.add(self.version())
        }
        fn callback(&self);
        fn callbacks(&self) -> self::CallbackProxyObj<Self::Api>;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: drt_sc::contract_base::ContractBase {}

    // impl<C> super::module_1::AutoImpl for C where C: AutoImpl {}

    impl<C> Adder for C
    where
        C: AutoImpl + super::module_1::AutoImpl,
    {
        fn get_sum(&self) -> BigInt<Self::Api> {
            let mut ___key___ = drt_sc::storage::StorageKey::<Self::Api>::new(&b"sum"[..]);
            drt_sc::storage_get(drt_sc::types::ManagedRef::new(&___key___))
        }
        fn set_sum(&self, sum: &BigInt<Self::Api>) {
            let mut ___key___ = drt_sc::storage::StorageKey::<Self::Api>::new(&b"sum"[..]);
            drt_sc::storage_set(drt_sc::types::ManagedRef::new(&___key___), &sum);
        }
        fn callback(&self) {}
        fn callbacks(&self) -> self::CallbackProxyObj<Self::Api> {
            <self::CallbackProxyObj::<Self::Api> as drt_sc::contract_base::CallbackProxyObjBase>::new_cb_proxy_obj()
        }
    }

    impl<A> AutoImpl for drt_sc::contract_base::UniversalContractObj<A> where
        A: drt_sc::api::VMApi
    {
    }

    pub trait EndpointWrappers:
        Adder + drt_sc::contract_base::ContractBase + super::module_1::EndpointWrappers
    {
        #[inline]
        fn call_get_sum(&self) {
            <Self::Api as drt_sc::api::VMApi>::init_static();
            drt_sc::api::CallValueApiImpl::check_not_payable(
                &Self::Api::call_value_api_impl(),
            );
            let () = drt_sc::io::load_endpoint_args::<Self::Api, ()>(());
            let result = self.get_sum();
            drt_sc::io::finish_multi::<Self::Api, _>(&result);
        }
        #[inline]
        fn call_init(&self) {
            <Self::Api as drt_sc::api::VMApi>::init_static();
            drt_sc::api::CallValueApiImpl::check_not_payable(
                &Self::Api::call_value_api_impl(),
            );
            let (initial_value, ()) = drt_sc::io::load_endpoint_args::<
                Self::Api,
                (drt_sc::types::BigInt<Self::Api>, ()),
            >(("initial_value", ()));
            self.init(&initial_value);
        }
        #[inline]
        fn call_add(&self) {
            <Self::Api as drt_sc::api::VMApi>::init_static();
            drt_sc::api::CallValueApiImpl::check_not_payable(
                &Self::Api::call_value_api_impl(),
            );
            let (value, ()) = drt_sc::io::load_endpoint_args::<
                Self::Api,
                (drt_sc::types::BigInt<Self::Api>, ()),
            >(("value", ()));
            let result = self.add(value);
            drt_sc::io::finish_multi::<Self::Api, _>(&result);
        }

        fn call(&self, fn_name: &str) -> bool {
            if match fn_name {
                "callBack" => {
                    Adder::callback(self);
                    return true;
                },
                "getSum" => {
                    self.call_get_sum();
                    true
                },
                "init" => {
                    self.call_init();
                    true
                },
                "add" => {
                    self.call_add();
                    true
                },
                _other => false,
            } {
                return true;
            }
            if super::module_1::EndpointWrappers::call(self, fn_name) {
                return true;
            }
            false
        }
    }

    impl<A> EndpointWrappers for drt_sc::contract_base::UniversalContractObj<A> where
        A: drt_sc::api::VMApi
    {
    }

    pub trait ProxyTrait:
        drt_sc::contract_base::ProxyObjBase + super::module_1::ProxyTrait
    {
        fn get_sum(
            &mut self,
        ) -> drt_sc::types::ContractCallNoPayment<Self::Api, BigInt<Self::Api>> {
            let ___address___ = self.extract_address();
            drt_sc::types::ContractCallNoPayment::new(___address___, "get_sum")
        }
        fn add(
            &mut self,
            amount: &BigInt<Self::Api>,
        ) -> drt_sc::types::ContractCallNoPayment<Self::Api, ()> {
            let ___address___ = self.extract_address();
            let mut ___contract_call___ =
                drt_sc::types::ContractCallNoPayment::new(___address___, "add");
            drt_sc::types::ContractCall::proxy_arg(&mut ___contract_call___, amount);
            ___contract_call___
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT ////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub struct ContractObj<A>
    where
        A: drt_sc::api::VMApi,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT as CONTRACT BASE ///////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    impl<A> drt_sc::contract_base::ContractBase for ContractObj<A>
    where
        A: drt_sc::api::VMApi,
    {
        type Api = A;
    }

    impl<A> super::module_1::AutoImpl for ContractObj<A> where A: drt_sc::api::VMApi {}

    impl<A> AutoImpl for ContractObj<A> where A: drt_sc::api::VMApi {}

    impl<A> super::module_1::EndpointWrappers for ContractObj<A> where A: drt_sc::api::VMApi {}

    impl<A> EndpointWrappers for ContractObj<A> where A: drt_sc::api::VMApi {}

    impl<A> drt_sc::contract_base::CallableContract for ContractObj<A>
    where
        A: drt_sc::api::VMApi,
    {
        fn call(&self, fn_name: &str) -> bool {
            EndpointWrappers::call(
                &drt_sc::contract_base::UniversalContractObj::<A>::new(),
                fn_name,
            )
        }
    }

    pub struct ContractBuilder;

    impl drt_sc::contract_base::CallableContractBuilder for ContractBuilder {
        fn new_contract_obj<A: drt_sc::api::VMApi>(
            &self,
        ) -> drt_sc::types::heap::Box<dyn drt_sc::contract_base::CallableContract>
        {
            drt_sc::types::heap::Box::new(ContractObj::<A> {
                _phantom: core::marker::PhantomData,
            })
        }
    }

    pub struct AbiProvider {}

    impl drt_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = drt_sc::api::uncallable::UncallableApi;

        fn abi() -> drt_sc::abi::ContractAbi {
            drt_sc::abi::ContractAbi::default()
        }
    }

    pub fn contract_obj<A>() -> ContractObj<A>
    where
        A: drt_sc::api::VMApi,
    {
        ContractObj {
            _phantom: core::marker::PhantomData,
        }
    }

    pub struct Proxy<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        pub address:
            drt_sc::types::ManagedOption<A, drt_sc::types::ManagedAddress<A>>,
    }

    impl<A> drt_sc::contract_base::ProxyObjBase for Proxy<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        type Api = A;

        fn new_proxy_obj() -> Self {
            Proxy {
                address: drt_sc::types::ManagedOption::none(),
            }
        }

        fn contract(mut self, address: drt_sc::types::ManagedAddress<Self::Api>) -> Self {
            self.address = drt_sc::types::ManagedOption::some(address);
            self
        }

        fn extract_opt_address(
            &mut self,
        ) -> drt_sc::types::ManagedOption<
            Self::Api,
            drt_sc::types::ManagedAddress<Self::Api>,
        > {
            core::mem::replace(
                &mut self.address,
                drt_sc::types::ManagedOption::none(),
            )
        }

        fn extract_address(&mut self) -> drt_sc::types::ManagedAddress<Self::Api> {
            let address = core::mem::replace(
                &mut self.address,
                drt_sc::types::ManagedOption::none(),
            );
            address.unwrap_or_sc_panic(drt_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET)
        }
    }

    impl<A> super::module_1::ProxyTrait for Proxy<A> where A: drt_sc::api::VMApi {}

    impl<A> ProxyTrait for Proxy<A> where A: drt_sc::api::VMApi {}

    pub struct CallbackProxyObj<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    impl<A> drt_sc::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        type Api = A;

        fn new_cb_proxy_obj() -> Self {
            CallbackProxyObj {
                _phantom: core::marker::PhantomData,
            }
        }
    }

    pub trait CallbackProxy: drt_sc::contract_base::CallbackProxyObjBase + Sized {
        fn my_callback(self, caller: &Address) -> drt_sc::types::CallbackClosure<Self::Api> {
            let mut ___callback_call___ =
                drt_sc::types::new_callback_call::<Self::Api>("my_callback");
            ___callback_call___.push_endpoint_arg(caller);
            ___callback_call___
        }
    }
    impl<A> self::CallbackProxy for CallbackProxyObj<A> where A: drt_sc::api::VMApi + 'static {}
}

#[test]
fn test_add() {
    use drt_chain_vm::DebugApi;
    use sample_adder::{Adder, EndpointWrappers, ProxyTrait};

    let _ = DebugApi::dummy();

    let adder = sample_adder::contract_obj::<DebugApi>();

    adder.init(&BigInt::from(5));
    assert_eq!(BigInt::from(5), adder.get_sum());

    let _ = adder.add(BigInt::from(7));
    assert_eq!(BigInt::from(12), adder.get_sum());

    let _ = adder.add(BigInt::from(-1));
    assert_eq!(BigInt::from(11), adder.get_sum());

    assert_eq!(BigInt::from(100), adder.version());

    let _ = adder.add_version();
    assert_eq!(BigInt::from(111), adder.get_sum());

    assert!(!adder.call("invalid_endpoint"));

    assert!(adder.call("version"));

    let mut own_proxy =
        sample_adder::Proxy::<DebugApi>::new_proxy_obj().contract(ManagedAddress::zero());
    let _ = own_proxy.get_sum();

    let _ = drt_sc_meta::abi_json::contract_abi::<sample_adder::AbiProvider>();
}

// TODO: re-enable after reorganizing project
//
// fn world() -> drt_sc_scenario::ScenarioWorld {
//     let mut blockchain = drt_sc_scenario::ScenarioWorld::new();
//     blockchain.register_contract(
//         "file:../contracts/examples/adder/output/adder.wasm",
//         sample_adder::ContractBuilder,
//     );
//     blockchain
// }

// #[test]
// fn test_denali() {
//     drt_sc_scenario::run_rs(
//         "../contracts/examples/adder/scenarios/adder.scen.json",
//         world(),
//     );
// }
