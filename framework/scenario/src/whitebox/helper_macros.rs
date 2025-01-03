#[macro_export]
macro_rules! rust_biguint {
    ($value:expr) => {{
        drt_sc_scenario::num_bigint::BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_biguint {
    ($value:expr) => {{
        drt_sc::types::BigUint::from($value as u64)
    }};
}

#[macro_export]
macro_rules! managed_buffer {
    ($value:expr) => {{
        drt_sc::types::ManagedBuffer::new_from_bytes($value)
    }};
}

#[macro_export]
macro_rules! managed_address {
    ($address:expr) => {{
        drt_sc::types::ManagedAddress::from_address($address)
    }};
}

#[macro_export]
macro_rules! managed_token_id {
    ($bytes:expr) => {{
        drt_sc::types::TokenIdentifier::from_dcdt_bytes($bytes)
    }};
}

#[macro_export]
macro_rules! managed_token_id_wrapped {
    ($bytes:expr) => {{
        let ___dcdt_token_id___ = drt_sc::types::TokenIdentifier::from_dcdt_bytes($bytes);
        drt_sc::types::RewaOrDcdtTokenIdentifier::dcdt(___dcdt_token_id___)
    }};
}

#[macro_export]
macro_rules! managed_rewa_token_id {
    () => {{
        drt_sc::types::RewaOrDcdtTokenIdentifier::rewa()
    }};
}

#[macro_export]
macro_rules! assert_sc_error {
    ($sc_result:expr, $expected_string:expr) => {{
        match $sc_result {
            drt_sc::types::SCResult::Ok(t) => {
                panic!("Expected SCError, but got SCResult::Ok: {:?}", t)
            },
            drt_sc::types::SCResult::Err(err) => {
                let as_str = String::from_utf8(err.as_bytes().to_vec()).unwrap();
                assert_eq!(as_str, $expected_string);
            },
        }
    }};
}

#[macro_export]
macro_rules! assert_values_eq {
    ($left:expr, $right:expr) => {{
        assert!(
            $left == $right,
            "Assert mismatch: \n Left: {:?} \n Right: {:?}",
            $left,
            $right
        )
    }};
}

#[macro_export]
macro_rules! unwrap_or_panic {
    ($sc_result:expr) => {{
        match $sc_result {
            drt_sc::types::SCResult::Ok(t) => t,
            drt_sc::types::SCResult::Err(err) => {
                let as_str = String::from_utf8(err.as_bytes().to_vec()).unwrap();
                panic!("{}", as_str);
            },
        }
    }};
}
