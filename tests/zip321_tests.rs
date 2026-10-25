use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use zecpay::zcash::zip321::{Zip321PaymentRequest, DEFAULT_TESTNET_UA};

#[test]
fn test_zip321_uri_generation() {
    let req = Zip321PaymentRequest::new(
        DEFAULT_TESTNET_UA,
        0.05,
        Some("Order #1042".to_string()),
    )
    .expect("Valid payment request");

    let uri = req.to_uri();
    println!("Generated ZIP-321 URI: {}", uri);

    assert!(uri.starts_with(&format!("zcash:{}?amount=0.05&memo=", DEFAULT_TESTNET_UA)));

    // Extract memo query parameter and decode base64url
    let memo_part = uri.split("&memo=").nth(1).expect("URI contains memo parameter");
    let decoded_bytes = URL_SAFE_NO_PAD.decode(memo_part).expect("Valid base64url");
    let decoded_memo = String::from_utf8(decoded_bytes).expect("Valid UTF-8 memo");

    assert_eq!(decoded_memo, "Order #1042");
}

#[test]
fn test_zip321_amount_precision() {
    let req = Zip321PaymentRequest::new(DEFAULT_TESTNET_UA, 0.00000001, None).unwrap();
    assert_eq!(
        req.to_uri(),
        format!("zcash:{}?amount=0.00000001", DEFAULT_TESTNET_UA)
    );

    let req_int = Zip321PaymentRequest::new(DEFAULT_TESTNET_UA, 1.0, None).unwrap();
    assert_eq!(req_int.to_uri(), format!("zcash:{}?amount=1", DEFAULT_TESTNET_UA));
}

#[test]
fn test_zip321_validation_errors() {
    // 0 or negative amount
    assert!(Zip321PaymentRequest::new(DEFAULT_TESTNET_UA, 0.0, None).is_err());
    assert!(Zip321PaymentRequest::new(DEFAULT_TESTNET_UA, -1.5, None).is_err());

    // Memo exceeding 512 bytes
    let huge_memo = "A".repeat(513);
    assert!(Zip321PaymentRequest::new(DEFAULT_TESTNET_UA, 0.1, Some(huge_memo)).is_err());
}
