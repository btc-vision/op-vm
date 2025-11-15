use crate::domain::runner::MLDSAPublicKeyMetadata;

pub fn verify_mldsa_internal(
    metadata: &MLDSAPublicKeyMetadata,
    public_key_bytes: &[u8],
    signature_bytes: &[u8],
    message_bytes: &[u8],
) -> bool {
    match metadata {
        MLDSAPublicKeyMetadata::MLDSA44 => {
            #[cfg(feature = "ml-dsa-44")]
            {
                use fips204::ml_dsa_44;
                use fips204::traits::{SerDes, Verifier};

                // Convert public key bytes to array
                let Ok(pk_array): Result<[u8; 1312], _> = public_key_bytes.try_into() else {
                    return false;
                };

                // Convert signature bytes to array
                let Ok(sig_array): Result<[u8; 2420], _> = signature_bytes.try_into() else {
                    return false;
                };

                // Deserialize public key
                let Ok(public_key) = ml_dsa_44::PublicKey::try_from_bytes(pk_array) else {
                    return false;
                };

                // Verify signature with empty context
                public_key.verify(message_bytes, &sig_array, &[])
            }
            #[cfg(not(feature = "ml-dsa-44"))]
            {
                false
            }
        }
        MLDSAPublicKeyMetadata::MLDSA65 => {
            #[cfg(feature = "ml-dsa-65")]
            {
                use fips204::ml_dsa_65;
                use fips204::traits::{SerDes, Verifier};

                // Convert public key bytes to array
                let Ok(pk_array): Result<[u8; 1952], _> = public_key_bytes.try_into() else {
                    return false;
                };

                // Convert signature bytes to array
                let Ok(sig_array): Result<[u8; 3309], _> = signature_bytes.try_into() else {
                    return false;
                };

                // Deserialize public key
                let Ok(public_key) = ml_dsa_65::PublicKey::try_from_bytes(pk_array) else {
                    return false;
                };

                // Verify signature with empty context
                public_key.verify(message_bytes, &sig_array, &[])
            }
            #[cfg(not(feature = "ml-dsa-65"))]
            {
                false
            }
        }
        MLDSAPublicKeyMetadata::MLDSA87 => {
            #[cfg(feature = "ml-dsa-87")]
            {
                use fips204::ml_dsa_87;
                use fips204::traits::{SerDes, Verifier};

                // Convert public key bytes to array
                let Ok(pk_array): Result<[u8; 2592], _> = public_key_bytes.try_into() else {
                    return false;
                };

                // Convert signature bytes to array
                let Ok(sig_array): Result<[u8; 4627], _> = signature_bytes.try_into() else {
                    return false;
                };

                // Deserialize public key
                let Ok(public_key) = ml_dsa_87::PublicKey::try_from_bytes(pk_array) else {
                    return false;
                };

                // Verify signature with empty context
                public_key.verify(message_bytes, &sig_array, &[])
            }
            #[cfg(not(feature = "ml-dsa-87"))]
            {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test data generation helpers
    fn generate_test_data(seed: u8, len: usize) -> Vec<u8> {
        (0..len)
            .map(|i| ((i as u8).wrapping_add(seed)) ^ 0xAA)
            .collect()
    }

    fn corrupt_bytes(data: &mut [u8], positions: &[usize]) {
        for &pos in positions {
            if pos < data.len() {
                data[pos] ^= 0xFF;
            }
        }
    }

    // Basic panic safety tests
    #[test]
    fn test_panic_safety_empty_inputs() {
        let result = std::panic::catch_unwind(|| {
            let metadata = MLDSAPublicKeyMetadata::MLDSA44;
            verify_mldsa_internal(&metadata, &[], &[], &[])
        });
        assert!(result.is_ok());

        let result = std::panic::catch_unwind(|| {
            let metadata = MLDSAPublicKeyMetadata::MLDSA65;
            verify_mldsa_internal(&metadata, &[], &[], &[])
        });
        assert!(result.is_ok());

        let result = std::panic::catch_unwind(|| {
            let metadata = MLDSAPublicKeyMetadata::MLDSA87;
            verify_mldsa_internal(&metadata, &[], &[], &[])
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_panic_safety_wrong_sizes() {
        let small_data = vec![0u8; 10];
        let large_data = vec![0xFFu8; 10000];

        for metadata in [
            MLDSAPublicKeyMetadata::MLDSA44,
            MLDSAPublicKeyMetadata::MLDSA65,
            MLDSAPublicKeyMetadata::MLDSA87,
        ] {
            let result = std::panic::catch_unwind(|| {
                verify_mldsa_internal(&metadata, &small_data, &small_data, b"test")
            });
            assert!(result.is_ok());
            assert!(!result.unwrap());

            let result = std::panic::catch_unwind(|| {
                verify_mldsa_internal(&metadata, &large_data, &large_data, &large_data)
            });
            assert!(result.is_ok());
            assert!(!result.unwrap());
        }
    }

    #[test]
    fn test_invalid_public_key_sizes() {
        let metadata = MLDSAPublicKeyMetadata::MLDSA44;
        let wrong_pk = vec![0u8; 1311]; // One byte short
        let sig = vec![0u8; 2420];
        assert!(!verify_mldsa_internal(
            &metadata, &wrong_pk, &sig, b"message"
        ));

        let wrong_pk = vec![0u8; 1313]; // One byte too many
        assert!(!verify_mldsa_internal(
            &metadata, &wrong_pk, &sig, b"message"
        ));
    }

    #[test]
    fn test_invalid_signature_sizes() {
        let metadata = MLDSAPublicKeyMetadata::MLDSA44;
        let pk = vec![0u8; 1312];
        let wrong_sig = vec![0u8; 2419]; // One byte short
        assert!(!verify_mldsa_internal(
            &metadata, &pk, &wrong_sig, b"message"
        ));

        let wrong_sig = vec![0u8; 2421]; // One byte too many
        assert!(!verify_mldsa_internal(
            &metadata, &pk, &wrong_sig, b"message"
        ));
    }

    #[test]
    fn test_malformed_public_keys() {
        // Test with random data that won't deserialize properly
        let metadata = MLDSAPublicKeyMetadata::MLDSA44;
        let garbage_pk = generate_test_data(0x13, 1312);
        let sig = vec![0u8; 2420];
        assert!(!verify_mldsa_internal(
            &metadata,
            &garbage_pk,
            &sig,
            b"message"
        ));

        // Test with all zeros (likely invalid)
        let zero_pk = vec![0u8; 1312];
        assert!(!verify_mldsa_internal(
            &metadata, &zero_pk, &sig, b"message"
        ));

        // Test with all ones (likely invalid)
        let ones_pk = vec![0xFFu8; 1312];
        assert!(!verify_mldsa_internal(
            &metadata, &ones_pk, &sig, b"message"
        ));
    }

    #[test]
    fn test_empty_message() {
        let metadata = MLDSAPublicKeyMetadata::MLDSA44;
        let pk = vec![0u8; 1312];
        let sig = vec![0u8; 2420];

        // Should handle empty message without panic
        let result = std::panic::catch_unwind(|| verify_mldsa_internal(&metadata, &pk, &sig, &[]));
        assert!(result.is_ok());
    }

    #[test]
    fn test_large_message() {
        let metadata = MLDSAPublicKeyMetadata::MLDSA44;
        let pk = vec![0u8; 1312];
        let sig = vec![0u8; 2420];

        // Test with various large message sizes
        for size in [1024, 10_000, 100_000, 1_000_000] {
            let large_msg = vec![0xABu8; size];
            let result = std::panic::catch_unwind(|| {
                verify_mldsa_internal(&metadata, &pk, &sig, &large_msg)
            });
            assert!(result.is_ok());
        }
    }

    // Feature-gated comprehensive tests
    #[cfg(feature = "ml-dsa-44")]
    mod ml_dsa_44_tests {
        use super::*;
        use fips204::ml_dsa_44;
        use fips204::traits::{KeyGen, SerDes, Signer};

        #[test]
        fn test_valid_signature() {
            let (pk, sk) = ml_dsa_44::KG::keygen_from_seed(&[0x42u8; 32]);
            let message = b"Valid test message for ML-DSA-44";
            let sig = sk.try_sign_with_seed(&[0x77u8; 32], message, &[]).unwrap();

            let pk_bytes = pk.into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA44;
            assert!(verify_mldsa_internal(&metadata, &pk_bytes, &sig, message));
        }

        #[test]
        fn test_invalid_signature() {
            let (pk, sk) = ml_dsa_44::KG::keygen_from_seed(&[0x42u8; 32]);
            let message = b"Test message";
            let mut sig = sk.try_sign_with_seed(&[0x77u8; 32], message, &[]).unwrap();

            // Corrupt the signature
            corrupt_bytes(&mut sig, &[0, 100, 1000, 2419]);

            let pk_bytes = pk.into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA44;
            assert!(!verify_mldsa_internal(&metadata, &pk_bytes, &sig, message));
        }

        #[test]
        fn test_wrong_message() {
            let (pk, sk) = ml_dsa_44::KG::keygen_from_seed(&[0x42u8; 32]);
            let original_message = b"Original message";
            let wrong_message = b"Different message";
            let sig = sk
                .try_sign_with_seed(&[0x77u8; 32], original_message, &[])
                .unwrap();

            let pk_bytes = pk.into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA44;
            assert!(!verify_mldsa_internal(
                &metadata,
                &pk_bytes,
                &sig,
                wrong_message
            ));
        }

        #[test]
        fn test_wrong_public_key() {
            let (_pk1, sk1) = ml_dsa_44::KG::keygen_from_seed(&[0x42u8; 32]);
            let (pk2, _sk2) = ml_dsa_44::KG::keygen_from_seed(&[0x43u8; 32]);
            let message = b"Test message";
            let sig = sk1.try_sign_with_seed(&[0x77u8; 32], message, &[]).unwrap();

            let pk2_bytes = pk2.into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA44;
            assert!(!verify_mldsa_internal(&metadata, &pk2_bytes, &sig, message));
        }

        #[test]
        fn test_deterministic_signatures() {
            let (pk, sk) = ml_dsa_44::KG::keygen_from_seed(&[0x42u8; 32]);
            let message = b"Deterministic test";

            let sig1 = sk.try_sign_with_seed(&[0x77u8; 32], message, &[]).unwrap();
            let sig2 = sk.try_sign_with_seed(&[0x77u8; 32], message, &[]).unwrap();

            // Same seed should produce same signature
            assert_eq!(sig1, sig2);

            let pk_bytes = pk.into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA44;
            assert!(verify_mldsa_internal(&metadata, &pk_bytes, &sig1, message));
        }

        #[test]
        fn test_different_seeds_different_signatures() {
            let (pk, sk) = ml_dsa_44::KG::keygen_from_seed(&[0x42u8; 32]);
            let message = b"Test with different seeds";

            let sig1 = sk.try_sign_with_seed(&[0x77u8; 32], message, &[]).unwrap();
            let sig2 = sk.try_sign_with_seed(&[0x88u8; 32], message, &[]).unwrap();

            // Different seeds should produce different signatures
            assert_ne!(sig1, sig2);

            let pk_bytes = pk.into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA44;
            // Both should verify
            assert!(verify_mldsa_internal(&metadata, &pk_bytes, &sig1, message));
            assert!(verify_mldsa_internal(&metadata, &pk_bytes, &sig2, message));
        }

        #[test]
        fn test_corrupted_public_key() {
            let (pk, sk) = ml_dsa_44::KG::keygen_from_seed(&[0x42u8; 32]);
            let message = b"Test message";
            let sig = sk.try_sign_with_seed(&[0x77u8; 32], message, &[]).unwrap();

            let mut pk_bytes = pk.into_bytes();
            // Corrupt various parts of the public key
            corrupt_bytes(&mut pk_bytes, &[0, 656, 1311]);

            let metadata = MLDSAPublicKeyMetadata::MLDSA44;
            assert!(!verify_mldsa_internal(&metadata, &pk_bytes, &sig, message));
        }
    }

    #[cfg(feature = "ml-dsa-65")]
    mod ml_dsa_65_tests {
        use super::*;
        use fips204::ml_dsa_65;
        use fips204::traits::{KeyGen, SerDes, Signer};

        #[test]
        fn test_valid_signature() {
            let (pk, sk) = ml_dsa_65::KG::keygen_from_seed(&[0x65u8; 32]);
            let message = b"Valid test message for ML-DSA-65 with longer content";
            let sig = sk.try_sign_with_seed(&[0x11u8; 32], message, &[]).unwrap();

            let pk_bytes = pk.into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA65;
            assert!(verify_mldsa_internal(&metadata, &pk_bytes, &sig, message));
        }

        #[test]
        fn test_message_variations() {
            let (pk, sk) = ml_dsa_65::KG::keygen_from_seed(&[0x65u8; 32]);
            let pk_bytes = pk.clone().into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA65;

            // Test various message sizes
            for size in [0, 1, 32, 64, 128, 256, 512, 1024, 4096] {
                let message = generate_test_data(size as u8, size);
                let sig = sk
                    .try_sign_with_seed(&[size as u8; 32], &message, &[])
                    .unwrap();
                assert!(verify_mldsa_internal(&metadata, &pk_bytes, &sig, &message));
            }
        }

        #[test]
        fn test_signature_uniqueness() {
            let (pk, sk) = ml_dsa_65::KG::keygen_from_seed(&[0x65u8; 32]);
            let pk_bytes = pk.into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA65;

            let messages = [b"Message A", b"Message B", b"Message C"];

            let mut signatures = Vec::new();
            for msg in &messages {
                let sig = sk.try_sign_with_seed(&[0x22u8; 32], *msg, &[]).unwrap();
                signatures.push(sig);
            }

            // Each signature should be unique
            for i in 0..signatures.len() {
                for j in i + 1..signatures.len() {
                    assert_ne!(signatures[i], signatures[j]);
                }
            }

            // Each signature should only verify its corresponding message
            for (i, sig) in signatures.iter().enumerate() {
                for (j, msg) in messages.iter().enumerate() {
                    let should_verify = i == j;
                    assert_eq!(
                        verify_mldsa_internal(&metadata, &pk_bytes, sig, msg.as_ref()),
                        should_verify
                    );
                }
            }
        }
    }

    #[cfg(feature = "ml-dsa-87")]
    mod ml_dsa_87_tests {
        use super::*;
        use fips204::ml_dsa_87;
        use fips204::traits::{KeyGen, SerDes, Signer};

        #[test]
        fn test_valid_signature() {
            let (pk, sk) = ml_dsa_87::KG::keygen_from_seed(&[0x87u8; 32]);
            let message = b"Valid test message for ML-DSA-87 - highest security level";
            let sig = sk.try_sign_with_seed(&[0x99u8; 32], message, &[]).unwrap();

            let pk_bytes = pk.into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA87;
            assert!(verify_mldsa_internal(&metadata, &pk_bytes, &sig, message));
        }

        #[test]
        fn test_boundary_conditions() {
            let (pk, sk) = ml_dsa_87::KG::keygen_from_seed(&[0x87u8; 32]);
            let pk_bytes = pk.into_bytes();
            let metadata = MLDSAPublicKeyMetadata::MLDSA87;

            // Test with messages at boundary sizes
            let test_cases = [
                vec![],                       // Empty
                vec![0x00],                   // Single byte
                vec![0xFF; 1],                // Single byte max value
                generate_test_data(0, 255),   // Max single byte length
                generate_test_data(1, 256),   // Min two byte length
                generate_test_data(2, 65535), // Max two byte length
                generate_test_data(3, 65536), // Min three byte length
            ];

            for message in test_cases {
                let sig = sk
                    .try_sign_with_seed(&[message.len() as u8; 32], &message, &[])
                    .unwrap();
                assert!(verify_mldsa_internal(&metadata, &pk_bytes, &sig, &message));
            }
        }

        #[test]
        fn test_cross_keypair_rejection() {
            let (pk1, sk1) = ml_dsa_87::KG::keygen_from_seed(&[0x01u8; 32]);
            let (pk2, sk2) = ml_dsa_87::KG::keygen_from_seed(&[0x02u8; 32]);
            let (pk3, sk3) = ml_dsa_87::KG::keygen_from_seed(&[0x03u8; 32]);

            let message = b"Cross keypair test";
            let metadata = MLDSAPublicKeyMetadata::MLDSA87;

            // Sign with each key
            let sig1 = sk1.try_sign_with_seed(&[0x11u8; 32], message, &[]).unwrap();
            let sig2 = sk2.try_sign_with_seed(&[0x22u8; 32], message, &[]).unwrap();
            let sig3 = sk3.try_sign_with_seed(&[0x33u8; 32], message, &[]).unwrap();

            let pk1_bytes = pk1.into_bytes();
            let pk2_bytes = pk2.into_bytes();
            let pk3_bytes = pk3.into_bytes();

            // Only matching key pairs should verify
            assert!(verify_mldsa_internal(&metadata, &pk1_bytes, &sig1, message));
            assert!(verify_mldsa_internal(&metadata, &pk2_bytes, &sig2, message));
            assert!(verify_mldsa_internal(&metadata, &pk3_bytes, &sig3, message));

            // Cross verification should fail
            assert!(!verify_mldsa_internal(
                &metadata, &pk1_bytes, &sig2, message
            ));
            assert!(!verify_mldsa_internal(
                &metadata, &pk1_bytes, &sig3, message
            ));
            assert!(!verify_mldsa_internal(
                &metadata, &pk2_bytes, &sig1, message
            ));
            assert!(!verify_mldsa_internal(
                &metadata, &pk2_bytes, &sig3, message
            ));
            assert!(!verify_mldsa_internal(
                &metadata, &pk3_bytes, &sig1, message
            ));
            assert!(!verify_mldsa_internal(
                &metadata, &pk3_bytes, &sig2, message
            ));
        }
    }

    #[cfg(all(feature = "ml-dsa-44", feature = "ml-dsa-65", feature = "ml-dsa-87"))]
    mod cross_level_tests {
        use super::*;
        use fips204::traits::{KeyGen, SerDes, Signer};

        #[test]
        fn test_wrong_metadata_level() {
            // Generate keys and signatures for each level
            let (pk44, sk44) = fips204::ml_dsa_44::KG::keygen_from_seed(&[0x44u8; 32]);
            let (pk65, sk65) = fips204::ml_dsa_65::KG::keygen_from_seed(&[0x65u8; 32]);
            let (pk87, sk87) = fips204::ml_dsa_87::KG::keygen_from_seed(&[0x87u8; 32]);

            let message = b"Cross-level test message";

            let sig44 = sk44
                .try_sign_with_seed(&[0x11u8; 32], message, &[])
                .unwrap();
            let sig65 = sk65
                .try_sign_with_seed(&[0x22u8; 32], message, &[])
                .unwrap();
            let sig87 = sk87
                .try_sign_with_seed(&[0x33u8; 32], message, &[])
                .unwrap();

            let pk44_bytes = pk44.into_bytes();
            let pk65_bytes = pk65.into_bytes();
            let pk87_bytes = pk87.into_bytes();

            // Test using wrong metadata level (should all fail due to size mismatch)
            assert!(!verify_mldsa_internal(
                &MLDSAPublicKeyMetadata::MLDSA44,
                &pk65_bytes,
                &sig65,
                message
            ));
            assert!(!verify_mldsa_internal(
                &MLDSAPublicKeyMetadata::MLDSA44,
                &pk87_bytes,
                &sig87,
                message
            ));
            assert!(!verify_mldsa_internal(
                &MLDSAPublicKeyMetadata::MLDSA65,
                &pk44_bytes,
                &sig44,
                message
            ));
            assert!(!verify_mldsa_internal(
                &MLDSAPublicKeyMetadata::MLDSA65,
                &pk87_bytes,
                &sig87,
                message
            ));
            assert!(!verify_mldsa_internal(
                &MLDSAPublicKeyMetadata::MLDSA87,
                &pk44_bytes,
                &sig44,
                message
            ));
            assert!(!verify_mldsa_internal(
                &MLDSAPublicKeyMetadata::MLDSA87,
                &pk65_bytes,
                &sig65,
                message
            ));
        }

        #[test]
        fn test_all_levels_with_same_seed() {
            let seed = [0xAAu8; 32];
            let message = b"Same seed, different levels";

            // Generate keys with same seed for all levels
            let (pk44, sk44) = fips204::ml_dsa_44::KG::keygen_from_seed(&seed);
            let (pk65, sk65) = fips204::ml_dsa_65::KG::keygen_from_seed(&seed);
            let (pk87, sk87) = fips204::ml_dsa_87::KG::keygen_from_seed(&seed);

            // Sign with same seed
            let sign_seed = [0xBBu8; 32];
            let sig44 = sk44.try_sign_with_seed(&sign_seed, message, &[]).unwrap();
            let sig65 = sk65.try_sign_with_seed(&sign_seed, message, &[]).unwrap();
            let sig87 = sk87.try_sign_with_seed(&sign_seed, message, &[]).unwrap();

            // Verify each level independently
            let pk44_bytes = pk44.into_bytes();
            let pk65_bytes = pk65.into_bytes();
            let pk87_bytes = pk87.into_bytes();

            assert!(verify_mldsa_internal(
                &MLDSAPublicKeyMetadata::MLDSA44,
                &pk44_bytes,
                &sig44,
                message
            ));
            assert!(verify_mldsa_internal(
                &MLDSAPublicKeyMetadata::MLDSA65,
                &pk65_bytes,
                &sig65,
                message
            ));
            assert!(verify_mldsa_internal(
                &MLDSAPublicKeyMetadata::MLDSA87,
                &pk87_bytes,
                &sig87,
                message
            ));

            // Signatures should be different even with same seeds (different algorithms)
            assert_ne!(&sig44[..100], &sig65[..100]);
            assert_ne!(&sig65[..100], &sig87[..100]);
            assert_ne!(&sig44[..100], &sig87[..100]);
        }

        #[test]
        fn test_performance_characteristics() {
            use std::time::Instant;

            let message = b"Performance test message";

            // Generate keys for each level
            let (pk44, sk44) = fips204::ml_dsa_44::KG::keygen_from_seed(&[0x44u8; 32]);
            let (pk65, sk65) = fips204::ml_dsa_65::KG::keygen_from_seed(&[0x65u8; 32]);
            let (pk87, sk87) = fips204::ml_dsa_87::KG::keygen_from_seed(&[0x87u8; 32]);

            let sig44 = sk44
                .try_sign_with_seed(&[0x11u8; 32], message, &[])
                .unwrap();
            let sig65 = sk65
                .try_sign_with_seed(&[0x22u8; 32], message, &[])
                .unwrap();
            let sig87 = sk87
                .try_sign_with_seed(&[0x33u8; 32], message, &[])
                .unwrap();

            let pk44_bytes = pk44.into_bytes();
            let pk65_bytes = pk65.into_bytes();
            let pk87_bytes = pk87.into_bytes();

            // Measure verification times (crude but shows relative performance)
            let iterations = 10;

            let start = Instant::now();
            for _ in 0..iterations {
                verify_mldsa_internal(
                    &MLDSAPublicKeyMetadata::MLDSA44,
                    &pk44_bytes,
                    &sig44,
                    message,
                );
            }
            let time44 = start.elapsed();

            let start = Instant::now();
            for _ in 0..iterations {
                verify_mldsa_internal(
                    &MLDSAPublicKeyMetadata::MLDSA65,
                    &pk65_bytes,
                    &sig65,
                    message,
                );
            }
            let time65 = start.elapsed();

            let start = Instant::now();
            for _ in 0..iterations {
                verify_mldsa_internal(
                    &MLDSAPublicKeyMetadata::MLDSA87,
                    &pk87_bytes,
                    &sig87,
                    message,
                );
            }
            let time87 = start.elapsed();

            // Just ensure all completed without panic
            assert!(time44.as_nanos() > 0);
            assert!(time65.as_nanos() > 0);
            assert!(time87.as_nanos() > 0);

            // Typically ML-DSA-87 should take longest due to larger parameters
            if cfg!(not(debug_assertions)) {
                // In release mode, we might see performance differences
                println!("ML-DSA-44: {:?}", time44);
                println!("ML-DSA-65: {:?}", time65);
                println!("ML-DSA-87: {:?}", time87);
            }
        }
    }

    #[test]
    fn test_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let metadata = MLDSAPublicKeyMetadata::MLDSA44;
        let pk = Arc::new(vec![0u8; 1312]);
        let sig = Arc::new(vec![0u8; 2420]);
        let message = Arc::new(b"Thread safety test".to_vec());

        let mut handles = vec![];

        for _ in 0..10 {
            let metadata = metadata.clone();
            let pk = Arc::clone(&pk);
            let sig = Arc::clone(&sig);
            let message = Arc::clone(&message);

            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let _ = verify_mldsa_internal(&metadata, &pk, &sig, &message);
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread should not panic");
        }
    }
}
