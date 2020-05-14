extern crate sentinel;

use sentinel::utils;

#[test]
fn test_parse_utc_time_to_rfc_rfc3339() {
    let utc = utils::time::get_utc_time();
    let parsed_time = utils::time::parse_utc_time_to_rfc_rfc3339(utc);

    assert_eq!(utc, parsed_time);
}
