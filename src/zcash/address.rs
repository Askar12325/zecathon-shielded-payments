use zcash_address::ZcashAddress;

/// Validates whether the given string represents a valid Zcash address (UA, Sapling, or Transparent).
pub fn is_valid_address(address_str: &str) -> bool {
    address_str.parse::<ZcashAddress>().is_ok()
}
