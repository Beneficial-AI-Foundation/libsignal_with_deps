//
// Copyright 2020-2022 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

//! Interfaces in [traits] and reference implementations in [inmem] for various mutable stores.

#![warn(missing_docs)]

mod inmem;
mod traits;

pub use inmem::{
    InMemIdentityKeyStore, InMemKyberPreKeyStore, InMemPreKeyStore, InMemSenderKeyStore,
    InMemSessionStore, InMemSignalProtocolStore, InMemSignedPreKeyStore,
};
pub use traits::{
    Direction, IdentityChange, IdentityKeyStore, KyberPreKeyStore, PreKeyStore, ProtocolStore,
    SenderKeyStore, SessionStore, SignedPreKeyStore,
};

/*
#[cfg(test)]
mod tests {
    //! Validation tests for storage implementations
    //!
    //! These tests can be used to validate any implementation of the storage traits
    //! to ensure they satisfy the required properties for Signal Protocol.

    use super::*;
    use crate::{DeviceId, ProtocolAddress};

    /// Test suite that any SessionStore implementation should pass
    pub async fn test_session_store_properties<S>(mut store: S)
    where
        S: SessionStore + Send,
    {
        use crate::SessionRecord;

        let address1 = ProtocolAddress::new("alice".to_string(), DeviceId::from(1));
        let address2 = ProtocolAddress::new("bob".to_string(), DeviceId::from(1));

        // Test 1: Load non-existent session should return None
        let result = store.load_session(&address1).await.unwrap();
        assert_eq!(None, result);

        // Test 2: Store and load consistency
        let session = SessionRecord::new_fresh();
        store.store_session(&address1, &session).await.unwrap();

        let loaded = store.load_session(&address1).await.unwrap();
        assert!(loaded.is_some());

        // Test 3: Different addresses are isolated
        let result = store.load_session(&address2).await.unwrap();
        assert_eq!(None, result);

        // Test 4: Overwrite behavior
        let session2 = SessionRecord::new_fresh();
        store.store_session(&address1, &session2).await.unwrap();

        let loaded2 = store.load_session(&address1).await.unwrap();
        assert!(loaded2.is_some());
    }

    /// Test suite that any IdentityKeyStore implementation should pass
    pub async fn test_identity_store_properties<S>(mut store: S)
    where
        S: IdentityKeyStore + Send,
    {
        use crate::{IdentityKey, IdentityKeyPair};
        use rand::rngs::OsRng;

        let address = ProtocolAddress::new("alice".to_string(), DeviceId::from(1));
        let key1 = IdentityKeyPair::generate(&mut OsRng).identity_key();
        let key2 = IdentityKeyPair::generate(&mut OsRng).identity_key();

        // Test 1: Get non-existent identity should return None
        let result = store.get_identity(&address).await.unwrap();
        assert_eq!(None, result);

        // Test 2: First identity should be trusted (TOFU)
        let trusted = store.is_trusted_identity(&address, &key1, Direction::Sending).await.unwrap();
        assert!(trusted);

        // Test 3: Save new identity
        let change = store.save_identity(&address, &key1).await.unwrap();
        assert_eq!(IdentityChange::NewOrUnchanged, change);

        // Test 4: Saved identity should be retrievable
        let retrieved = store.get_identity(&address).await.unwrap();
        assert_eq!(Some(key1), retrieved);

        // Test 5: Same identity should not be marked as changed
        let change = store.save_identity(&address, &key1).await.unwrap();
        assert_eq!(IdentityChange::NewOrUnchanged, change);

        // Test 6: Different identity should be detected
        let change = store.save_identity(&address, &key2).await.unwrap();
        assert_eq!(IdentityChange::ReplacedExisting, change);
    }

    // Individual test implementations using the validation functions
    #[cfg(test)]
    mod validation_tests {
        use super::*;
        use crate::{IdentityKeyPair, InMemIdentityKeyStore, InMemSessionStore};
        use rand::rngs::OsRng;

        #[tokio::test]
        async fn test_inmem_session_store_validation() {
            let store = InMemSessionStore::new();
            test_session_store_properties(store).await;
        }

        #[tokio::test]
        async fn test_inmem_identity_store_validation() {
            let identity_pair = IdentityKeyPair::generate(&mut OsRng);
            let store = InMemIdentityKeyStore::new(identity_pair, 12345);
            test_identity_store_properties(store).await;
        }
    }
}

*/
