//
// Copyright 2020-2022 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

//! Implementations for stores defined in [super::traits].
//!
//! These implementations are purely in-memory, and therefore most likely useful for testing.

use std::borrow::Cow;
use std::collections::HashMap;

use async_trait::async_trait;
use uuid::Uuid;

use crate::storage::traits::{self, IdentityChange};
use crate::{
    IdentityKey, IdentityKeyPair, KyberPreKeyId, KyberPreKeyRecord, PreKeyId, PreKeyRecord,
    ProtocolAddress, Result, SenderKeyRecord, SessionRecord, SignalProtocolError, SignedPreKeyId,
    SignedPreKeyRecord,
};

/// Reference implementation of [traits::IdentityKeyStore].
#[derive(Clone)]
pub struct InMemIdentityKeyStore {
    key_pair: IdentityKeyPair,
    registration_id: u32,
    known_keys: HashMap<ProtocolAddress, IdentityKey>,
}

impl InMemIdentityKeyStore {
    /// Create a new instance.
    ///
    /// `key_pair` corresponds to [traits::IdentityKeyStore::get_identity_key_pair], and
    /// `registration_id` corresponds to [traits::IdentityKeyStore::get_local_registration_id].
    pub fn new(key_pair: IdentityKeyPair, registration_id: u32) -> Self {
        Self {
            key_pair,
            registration_id,
            known_keys: HashMap::new(),
        }
    }

    /// Clear the mapping of known keys.
    pub fn reset(&mut self) {
        self.known_keys.clear();
    }
}

#[async_trait(?Send)]
impl traits::IdentityKeyStore for InMemIdentityKeyStore {
    async fn get_identity_key_pair(&self) -> Result<IdentityKeyPair> {
        Ok(self.key_pair)
    }

    async fn get_local_registration_id(&self) -> Result<u32> {
        Ok(self.registration_id)
    }

    async fn save_identity(
        &mut self,
        address: &ProtocolAddress,
        identity: &IdentityKey,
    ) -> Result<IdentityChange> {
        match self.known_keys.get(address) {
            None => {
                self.known_keys.insert(address.clone(), *identity);
                Ok(IdentityChange::NewOrUnchanged)
            }
            Some(k) if k == identity => Ok(IdentityChange::NewOrUnchanged),
            Some(_k) => {
                self.known_keys.insert(address.clone(), *identity);
                Ok(IdentityChange::ReplacedExisting)
            }
        }
    }

    async fn is_trusted_identity(
        &self,
        address: &ProtocolAddress,
        identity: &IdentityKey,
        _direction: traits::Direction,
    ) -> Result<bool> {
        match self.known_keys.get(address) {
            None => {
                Ok(true) // first use
            }
            Some(k) => Ok(k == identity),
        }
    }

    async fn get_identity(&self, address: &ProtocolAddress) -> Result<Option<IdentityKey>> {
        match self.known_keys.get(address) {
            None => Ok(None),
            Some(k) => Ok(Some(k.to_owned())),
        }
    }
}

/// Reference implementation of [traits::PreKeyStore].
#[derive(Clone)]
pub struct InMemPreKeyStore {
    pre_keys: HashMap<PreKeyId, PreKeyRecord>,
}

impl InMemPreKeyStore {
    /// Create an empty pre-key store.
    pub fn new() -> Self {
        Self {
            pre_keys: HashMap::new(),
        }
    }

    /// Returns all registered pre-key ids
    pub fn all_pre_key_ids(&self) -> impl Iterator<Item = &PreKeyId> {
        self.pre_keys.keys()
    }
}

impl Default for InMemPreKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl traits::PreKeyStore for InMemPreKeyStore {
    async fn get_pre_key(&self, id: PreKeyId) -> Result<PreKeyRecord> {
        Ok(self
            .pre_keys
            .get(&id)
            .ok_or(SignalProtocolError::InvalidPreKeyId)?
            .clone())
    }

    async fn save_pre_key(&mut self, id: PreKeyId, record: &PreKeyRecord) -> Result<()> {
        // This overwrites old values, which matches Java behavior, but is it correct?
        self.pre_keys.insert(id, record.to_owned());
        Ok(())
    }

    async fn remove_pre_key(&mut self, id: PreKeyId) -> Result<()> {
        // If id does not exist this silently does nothing
        self.pre_keys.remove(&id);
        Ok(())
    }
}

/// Reference implementation of [traits::SignedPreKeyStore].
#[derive(Clone)]
pub struct InMemSignedPreKeyStore {
    signed_pre_keys: HashMap<SignedPreKeyId, SignedPreKeyRecord>,
}

impl InMemSignedPreKeyStore {
    /// Create an empty signed pre-key store.
    pub fn new() -> Self {
        Self {
            signed_pre_keys: HashMap::new(),
        }
    }

    /// Returns all registered signed pre-key ids
    pub fn all_signed_pre_key_ids(&self) -> impl Iterator<Item = &SignedPreKeyId> {
        self.signed_pre_keys.keys()
    }
}

impl Default for InMemSignedPreKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl traits::SignedPreKeyStore for InMemSignedPreKeyStore {
    async fn get_signed_pre_key(&self, id: SignedPreKeyId) -> Result<SignedPreKeyRecord> {
        Ok(self
            .signed_pre_keys
            .get(&id)
            .ok_or(SignalProtocolError::InvalidSignedPreKeyId)?
            .clone())
    }

    async fn save_signed_pre_key(
        &mut self,
        id: SignedPreKeyId,
        record: &SignedPreKeyRecord,
    ) -> Result<()> {
        // This overwrites old values, which matches Java behavior, but is it correct?
        self.signed_pre_keys.insert(id, record.to_owned());
        Ok(())
    }
}

/// Reference implementation of [traits::KyberPreKeyStore].
#[derive(Clone)]
pub struct InMemKyberPreKeyStore {
    kyber_pre_keys: HashMap<KyberPreKeyId, KyberPreKeyRecord>,
}

impl InMemKyberPreKeyStore {
    /// Create an empty kyber pre-key store.
    pub fn new() -> Self {
        Self {
            kyber_pre_keys: HashMap::new(),
        }
    }

    /// Returns all registered Kyber pre-key ids
    pub fn all_kyber_pre_key_ids(&self) -> impl Iterator<Item = &KyberPreKeyId> {
        self.kyber_pre_keys.keys()
    }
}

impl Default for InMemKyberPreKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl traits::KyberPreKeyStore for InMemKyberPreKeyStore {
    async fn get_kyber_pre_key(&self, kyber_prekey_id: KyberPreKeyId) -> Result<KyberPreKeyRecord> {
        Ok(self
            .kyber_pre_keys
            .get(&kyber_prekey_id)
            .ok_or(SignalProtocolError::InvalidKyberPreKeyId)?
            .clone())
    }

    async fn save_kyber_pre_key(
        &mut self,
        kyber_prekey_id: KyberPreKeyId,
        record: &KyberPreKeyRecord,
    ) -> Result<()> {
        self.kyber_pre_keys
            .insert(kyber_prekey_id, record.to_owned());
        Ok(())
    }

    async fn mark_kyber_pre_key_used(&mut self, _kyber_prekey_id: KyberPreKeyId) -> Result<()> {
        Ok(())
    }
}

/// Reference implementation of [traits::SessionStore].
#[derive(Clone)]
pub struct InMemSessionStore {
    sessions: HashMap<ProtocolAddress, SessionRecord>,
}

impl InMemSessionStore {
    /// Create an empty session store.
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    /// Bulk version of [`SessionStore::load_session`].
    ///
    /// Useful for [crate::sealed_sender_multi_recipient_encrypt].
    ///
    /// [`SessionStore::load_session`]: crate::SessionStore::load_session
    pub fn load_existing_sessions(
        &self,
        addresses: &[&ProtocolAddress],
    ) -> Result<Vec<&SessionRecord>> {
        addresses
            .iter()
            .map(|&address| {
                self.sessions
                    .get(address)
                    .ok_or_else(|| SignalProtocolError::SessionNotFound(address.clone()))
            })
            .collect()
    }
}

impl Default for InMemSessionStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl traits::SessionStore for InMemSessionStore {
    async fn load_session(&self, address: &ProtocolAddress) -> Result<Option<SessionRecord>> {
        match self.sessions.get(address) {
            None => Ok(None),
            Some(s) => Ok(Some(s.clone())),
        }
    }

    async fn store_session(
        &mut self,
        address: &ProtocolAddress,
        record: &SessionRecord,
    ) -> Result<()> {
        self.sessions.insert(address.clone(), record.clone());
        Ok(())
    }
}

/// Reference implementation of [traits::SenderKeyStore].
#[derive(Clone)]
pub struct InMemSenderKeyStore {
    // We use Cow keys in order to store owned values but compare to referenced ones.
    // See https://users.rust-lang.org/t/hashmap-with-tuple-keys/12711/6.
    keys: HashMap<(Cow<'static, ProtocolAddress>, Uuid), SenderKeyRecord>,
}

impl InMemSenderKeyStore {
    /// Create an empty sender key store.
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }
}

impl Default for InMemSenderKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl traits::SenderKeyStore for InMemSenderKeyStore {
    async fn store_sender_key(
        &mut self,
        sender: &ProtocolAddress,
        distribution_id: Uuid,
        record: &SenderKeyRecord,
    ) -> Result<()> {
        self.keys.insert(
            (Cow::Owned(sender.clone()), distribution_id),
            record.clone(),
        );
        Ok(())
    }

    async fn load_sender_key(
        &mut self,
        sender: &ProtocolAddress,
        distribution_id: Uuid,
    ) -> Result<Option<SenderKeyRecord>> {
        Ok(self
            .keys
            .get(&(Cow::Borrowed(sender), distribution_id))
            .cloned())
    }
}

/// Reference implementation of [traits::ProtocolStore].
#[allow(missing_docs)]
#[derive(Clone)]
pub struct InMemSignalProtocolStore {
    pub session_store: InMemSessionStore,
    pub pre_key_store: InMemPreKeyStore,
    pub signed_pre_key_store: InMemSignedPreKeyStore,
    pub kyber_pre_key_store: InMemKyberPreKeyStore,
    pub identity_store: InMemIdentityKeyStore,
    pub sender_key_store: InMemSenderKeyStore,
}

impl InMemSignalProtocolStore {
    /// Create an object with the minimal implementation of [traits::ProtocolStore], representing
    /// the given identity `key_pair` along with the separate randomly chosen `registration_id`.
    pub fn new(key_pair: IdentityKeyPair, registration_id: u32) -> Result<Self> {
        Ok(Self {
            session_store: InMemSessionStore::new(),
            pre_key_store: InMemPreKeyStore::new(),
            signed_pre_key_store: InMemSignedPreKeyStore::new(),
            kyber_pre_key_store: InMemKyberPreKeyStore::new(),
            identity_store: InMemIdentityKeyStore::new(key_pair, registration_id),
            sender_key_store: InMemSenderKeyStore::new(),
        })
    }

    /// Returns all registered pre-key ids
    pub fn all_pre_key_ids(&self) -> impl Iterator<Item = &PreKeyId> {
        self.pre_key_store.all_pre_key_ids()
    }

    /// Returns all registered signed pre-key ids
    pub fn all_signed_pre_key_ids(&self) -> impl Iterator<Item = &SignedPreKeyId> {
        self.signed_pre_key_store.all_signed_pre_key_ids()
    }

    /// Returns all registered Kyber pre-key ids
    pub fn all_kyber_pre_key_ids(&self) -> impl Iterator<Item = &KyberPreKeyId> {
        self.kyber_pre_key_store.all_kyber_pre_key_ids()
    }
}

#[async_trait(?Send)]
impl traits::IdentityKeyStore for InMemSignalProtocolStore {
    async fn get_identity_key_pair(&self) -> Result<IdentityKeyPair> {
        self.identity_store.get_identity_key_pair().await
    }

    async fn get_local_registration_id(&self) -> Result<u32> {
        self.identity_store.get_local_registration_id().await
    }

    async fn save_identity(
        &mut self,
        address: &ProtocolAddress,
        identity: &IdentityKey,
    ) -> Result<IdentityChange> {
        self.identity_store.save_identity(address, identity).await
    }

    async fn is_trusted_identity(
        &self,
        address: &ProtocolAddress,
        identity: &IdentityKey,
        direction: traits::Direction,
    ) -> Result<bool> {
        self.identity_store
            .is_trusted_identity(address, identity, direction)
            .await
    }

    async fn get_identity(&self, address: &ProtocolAddress) -> Result<Option<IdentityKey>> {
        self.identity_store.get_identity(address).await
    }
}

#[async_trait(?Send)]
impl traits::PreKeyStore for InMemSignalProtocolStore {
    async fn get_pre_key(&self, id: PreKeyId) -> Result<PreKeyRecord> {
        self.pre_key_store.get_pre_key(id).await
    }

    async fn save_pre_key(&mut self, id: PreKeyId, record: &PreKeyRecord) -> Result<()> {
        self.pre_key_store.save_pre_key(id, record).await
    }

    async fn remove_pre_key(&mut self, id: PreKeyId) -> Result<()> {
        self.pre_key_store.remove_pre_key(id).await
    }
}

#[async_trait(?Send)]
impl traits::SignedPreKeyStore for InMemSignalProtocolStore {
    async fn get_signed_pre_key(&self, id: SignedPreKeyId) -> Result<SignedPreKeyRecord> {
        self.signed_pre_key_store.get_signed_pre_key(id).await
    }

    async fn save_signed_pre_key(
        &mut self,
        id: SignedPreKeyId,
        record: &SignedPreKeyRecord,
    ) -> Result<()> {
        self.signed_pre_key_store
            .save_signed_pre_key(id, record)
            .await
    }
}

#[async_trait(?Send)]
impl traits::KyberPreKeyStore for InMemSignalProtocolStore {
    async fn get_kyber_pre_key(&self, kyber_prekey_id: KyberPreKeyId) -> Result<KyberPreKeyRecord> {
        self.kyber_pre_key_store
            .get_kyber_pre_key(kyber_prekey_id)
            .await
    }

    async fn save_kyber_pre_key(
        &mut self,
        kyber_prekey_id: KyberPreKeyId,
        record: &KyberPreKeyRecord,
    ) -> Result<()> {
        self.kyber_pre_key_store
            .save_kyber_pre_key(kyber_prekey_id, record)
            .await
    }

    async fn mark_kyber_pre_key_used(&mut self, kyber_prekey_id: KyberPreKeyId) -> Result<()> {
        self.kyber_pre_key_store
            .mark_kyber_pre_key_used(kyber_prekey_id)
            .await
    }
}

#[async_trait(?Send)]
impl traits::SessionStore for InMemSignalProtocolStore {
    async fn load_session(&self, address: &ProtocolAddress) -> Result<Option<SessionRecord>> {
        self.session_store.load_session(address).await
    }

    async fn store_session(
        &mut self,
        address: &ProtocolAddress,
        record: &SessionRecord,
    ) -> Result<()> {
        self.session_store.store_session(address, record).await
    }
}

#[async_trait(?Send)]
impl traits::SenderKeyStore for InMemSignalProtocolStore {
    async fn store_sender_key(
        &mut self,
        sender: &ProtocolAddress,
        distribution_id: Uuid,
        record: &SenderKeyRecord,
    ) -> Result<()> {
        self.sender_key_store
            .store_sender_key(sender, distribution_id, record)
            .await
    }

    async fn load_sender_key(
        &mut self,
        sender: &ProtocolAddress,
        distribution_id: Uuid,
    ) -> Result<Option<SenderKeyRecord>> {
        self.sender_key_store
            .load_sender_key(sender, distribution_id)
            .await
    }
}

impl traits::ProtocolStore for InMemSignalProtocolStore {}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ProtocolAddress, DeviceId};

    #[tokio::test]
    async fn test_session_store_load_consistency() {
        let mut store = InMemSessionStore::new();
        let address = ProtocolAddress::new("alice".to_string(), DeviceId::from(1));

        // Initially should return None
        let result = store.load_session(&address).await.unwrap();
        assert_eq!(None, result);

        // After storing should return the same record
        let session = SessionRecord::new_fresh();
        store.store_session(&address, &session).await.unwrap();

        let loaded = store.load_session(&address).await.unwrap();
        assert!(loaded.is_some());
        // Note: Can't directly compare SessionRecords without implementing PartialEq
    }

    #[tokio::test]
    async fn test_identity_change_detection() {
        let identity_pair = IdentityKeyPair::generate(&mut rand::rngs::OsRng);
        let mut store = InMemIdentityKeyStore::new(identity_pair, 12345);
        let address = ProtocolAddress::new("bob".to_string(), DeviceId::from(1));

        let key1 = IdentityKeyPair::generate(&mut rand::rngs::OsRng).identity_key();
        let key2 = IdentityKeyPair::generate(&mut rand::rngs::OsRng).identity_key();

        // First save should be new
        let result1 = store.save_identity(&address, &key1).await.unwrap();
        assert_eq!(IdentityChange::NewOrUnchanged, result1);

        // Same key should be unchanged
        let result2 = store.save_identity(&address, &key1).await.unwrap();
        assert_eq!(IdentityChange::NewOrUnchanged, result2);

        // Different key should be detected as replacement
        let result3 = store.save_identity(&address, &key2).await.unwrap();
        assert_eq!(IdentityChange::ReplacedExisting, result3);
    }
}
*/
