use super::IDENTITY_PALLET_NAME;
use crate::{
    primitives::{AccountId, AesOutput},
    ApiClient,
};
use codec::Decode;
use sp_core::{Pair, H256};
use sp_runtime::{MultiSignature, MultiSigner};
use std::sync::mpsc::channel;
use substrate_api_client::{ApiResult, StaticEvent};

pub trait IdentityManagementEventApi {
    fn wait_event_user_shielding_key_set(&self) -> ApiResult<SetUserShieldingKeyEvent>;
    fn wait_event_set_user_shielding_key_handle_failed(
        &self,
    ) -> ApiResult<SetUserShieldingKeyHandlingFailedEvent>;
    fn wait_event_delegatee_added(&self) -> ApiResult<DelegateeAddedEvent>;
    fn wait_event_identity_created(&self) -> ApiResult<IdentityCreatedEvent>;
    fn wait_event_identity_removed(&self) -> ApiResult<IdentityRemovedEvent>;
    fn wait_event_identity_verified(&self) -> ApiResult<IdentityVerifiedEvent>;
    fn wait_event_unexpected_message(&self) -> ApiResult<UnexpectedMessageEvent>;
}

pub trait IdentityManagementErrorApi {
    fn wait_error(&self) -> ApiResult<IdentityManagementError>;
}

impl<P> IdentityManagementEventApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn wait_event_user_shielding_key_set(&self) -> ApiResult<SetUserShieldingKeyEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let event: ApiResult<SetUserShieldingKeyEvent> = self.api.wait_for_event(&events_out);
        event
    }

    fn wait_event_set_user_shielding_key_handle_failed(
        &self,
    ) -> ApiResult<SetUserShieldingKeyHandlingFailedEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let event: ApiResult<SetUserShieldingKeyHandlingFailedEvent> =
            self.api.wait_for_event(&events_out);
        event
    }

    fn wait_event_delegatee_added(&self) -> ApiResult<DelegateeAddedEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();
        let event: ApiResult<DelegateeAddedEvent> = self.api.wait_for_event(&events_out);
        event
    }

    fn wait_event_identity_created(&self) -> ApiResult<IdentityCreatedEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();
        let event: ApiResult<IdentityCreatedEvent> = self.api.wait_for_event(&events_out);
        event
    }

    fn wait_event_identity_removed(&self) -> ApiResult<IdentityRemovedEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();
        let event: ApiResult<IdentityRemovedEvent> = self.api.wait_for_event(&events_out);
        event
    }

    fn wait_event_identity_verified(&self) -> ApiResult<IdentityVerifiedEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();
        let event: ApiResult<IdentityVerifiedEvent> = self.api.wait_for_event(&events_out);
        event
    }

    fn wait_event_unexpected_message(&self) -> ApiResult<UnexpectedMessageEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();
        let event: ApiResult<UnexpectedMessageEvent> = self.api.wait_for_event(&events_out);
        event
    }
}

impl<P> IdentityManagementErrorApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn wait_error(&self) -> ApiResult<IdentityManagementError> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let vc_disabled_event: ApiResult<IdentityManagementError> =
            self.api.wait_for_event(&events_out);
        vc_disabled_event
    }
}

/// UserShieldingKeySet
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct SetUserShieldingKeyEvent {
    pub account: AccountId,
    pub req_ext_hash: H256,
}

impl StaticEvent for SetUserShieldingKeyEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "UserShieldingKeySet";
}

/// SetUserShieldingKeyHandlingFailed
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct SetUserShieldingKeyHandlingFailedEvent;
impl StaticEvent for SetUserShieldingKeyHandlingFailedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "SetUserShieldingKeyHandlingFailed";
}

/// IdentityCreated
#[derive(Decode, Debug)]
pub struct IdentityCreatedEvent {
    pub who: AccountId,
    pub identity: AesOutput,
    pub code: AesOutput,
    pub req_ext_hash: H256,
}

impl StaticEvent for IdentityCreatedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "IdentityCreated";
}

/// IdentityRemoved
#[derive(Decode, Debug)]
pub struct IdentityRemovedEvent {
    pub who: AccountId,
    pub identity: AesOutput,
    pub req_ext_hash: H256,
}

impl StaticEvent for IdentityRemovedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "IdentityRemoved";
}

/// IdentityVerified
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct IdentityVerifiedEvent {
    pub account: AccountId,
    pub identity: AesOutput,
    pub id_graph: AesOutput,
    pub req_ext_hash: H256,
}

impl StaticEvent for IdentityVerifiedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "IdentityVerified";
}

/// DelegateeAdded
#[derive(Decode, Debug)]
pub struct DelegateeAddedEvent {
    pub account: AccountId,
}

impl StaticEvent for DelegateeAddedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "DelegateeAdded";
}

/// UnexpectedMessage
#[derive(Decode, Debug)]
pub struct UnexpectedMessageEvent;
impl StaticEvent for UnexpectedMessageEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "UnexpectedMessage";
}

/// Error
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct IdentityManagementError;
impl StaticEvent for IdentityManagementError {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "()";
}
