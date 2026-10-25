use zecpay::zcash::{
    address::is_valid_address,
    memo::{decode_memo_base64url, encode_memo_base64url},
    zip321::DEFAULT_TESTNET_UA,
};

#[test]
fn test_memo_encode_decode_roundtrip() {
    let original = "inv_84f910ab: ZECATHON Shielded Invoicing Tool #42";
    let encoded = encode_memo_base64url(original);

    // ZIP-321 memos must not have '=' padding characters
    assert!(!encoded.contains('='));

    let decoded = decode_memo_base64url(&encoded).expect("Valid decode");
    assert_eq!(decoded, original);
}

#[test]
fn test_address_validation() {
    // Valid testnet Unified Address
    assert!(is_valid_address(DEFAULT_TESTNET_UA));

    // Valid mainnet Unified Address from test vectors
    let valid_mainnet_ua = "u1ddnjsdcpm36r6aq79n3s68shjweksnmwtdltrh046s8m6xcws9ygyawalxx8n6hg6vegk0wh8zjnafxgh6msppjsljvyt0ynece3lvm0";
    assert!(is_valid_address(valid_mainnet_ua));

    // Invalid address strings
    assert!(!is_valid_address("invalid_address_12345"));
    assert!(!is_valid_address(""));
}
