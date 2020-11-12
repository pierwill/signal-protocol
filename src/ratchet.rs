use pyo3::prelude::*;
use pyo3::pyclass::PyClassAlloc;
use pyo3::exceptions;
use pyo3::wrap_pyfunction;

use rand::rngs::OsRng;

use crate::curve::{KeyPair,PublicKey};
use crate::error::SignalProtocolError;
use crate::identity_key::{IdentityKey,IdentityKeyPair};
use crate::state::SessionRecord;

use libsignal_protocol_rust;

#[pyclass]
pub struct AliceSignalProtocolParameters {
    inner: libsignal_protocol_rust::AliceSignalProtocolParameters,
}

#[pymethods]
impl AliceSignalProtocolParameters {
    #[new]
    pub fn new(
        our_identity_key_pair: IdentityKeyPair,
        our_base_key_pair: KeyPair,
        their_identity_key: IdentityKey,
        their_signed_pre_key: PublicKey,
        their_one_time_pre_key: Option<PublicKey>,
        their_ratchet_key: PublicKey,
    ) -> Self {
        let upstream_their_one_time_pre_key = match their_one_time_pre_key {
            None => None,
            Some(x) => Some(x.key)
        };

        Self { inner: libsignal_protocol_rust::AliceSignalProtocolParameters::new(
            our_identity_key_pair.key,
            our_base_key_pair.key,
            their_identity_key.key,
            their_signed_pre_key.key,
            upstream_their_one_time_pre_key,
            their_ratchet_key.key)}
    }
}

#[pyfunction]
pub fn initialize_alice_session(parameters: &AliceSignalProtocolParameters) -> PyResult<SessionRecord>{
    let mut csprng = OsRng;
    let session_state = libsignal_protocol_rust::initialize_alice_session(&parameters.inner, &mut csprng);
    match session_state {
        Ok(state) => Ok(SessionRecord::new(state)),
        Err(_e) => return Err(SignalProtocolError::new_err("could not create session"))
    }
}

#[pyclass]
pub struct BobSignalProtocolParameters {
    inner: libsignal_protocol_rust::BobSignalProtocolParameters,
}

#[pymethods]
impl BobSignalProtocolParameters {
    #[new]
    pub fn new(
        our_identity_key_pair: IdentityKeyPair,
        our_signed_pre_key_pair: KeyPair,
        our_one_time_pre_key_pair: Option<KeyPair>,
        our_ratchet_key_pair: KeyPair,
        their_identity_key: IdentityKey,
        their_base_key: PublicKey,
    ) -> Self {
        let upstream_our_one_time_pre_key_pair = match our_one_time_pre_key_pair {
            None => None,
            Some(x) => Some(x.key)
        };

        Self { inner: libsignal_protocol_rust::BobSignalProtocolParameters::new(
            our_identity_key_pair.key,
            our_signed_pre_key_pair.key,
            upstream_our_one_time_pre_key_pair,
            our_ratchet_key_pair.key,
            their_identity_key.key,
            their_base_key.key)}
    }
}

#[pyfunction]
pub fn initialize_bob_session(parameters: &BobSignalProtocolParameters) -> PyResult<SessionRecord>{
    let session_state = libsignal_protocol_rust::initialize_bob_session(&parameters.inner);
    match session_state {
        Ok(state) => Ok(SessionRecord::new(state)),
        Err(_e) => return Err(SignalProtocolError::new_err("could not create session"))
    }
}

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<AliceSignalProtocolParameters>()?;
    module.add_wrapped(wrap_pyfunction!(initialize_alice_session))?;
    module.add_class::<BobSignalProtocolParameters>()?;
    module.add_wrapped(wrap_pyfunction!(initialize_bob_session))?;
    Ok(())
}