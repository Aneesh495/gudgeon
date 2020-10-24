//! Tests for NMEA 0183 and AIS Sensor Decoders

use gudgeon::sensors::{AisDecoder, NmeaParser, NmeaRmc, NmeaSentence};

#[test]
fn test_nmea_rmc_parsing() {
    let raw = "$GPRMC,123519,A,4807.0380,N,01131.0000,E,022.4,084.4,230324,003.1,W*61";
    let parsed = NmeaParser::parse(raw).expect("Failed to parse valid RMC");
    if let NmeaSentence::Rmc(rmc) = parsed {
        assert_eq!(rmc.utc_time, "123519");
        assert_eq!(rmc.status, 'A');
        assert!((rmc.lat_deg - 48.1173).abs() < 1e-4);
        assert!((rmc.lon_deg - 11.5166).abs() < 1e-4);
        assert!((rmc.sog_knots - 22.4).abs() < 1e-4);
        assert!((rmc.cog_deg - 84.4).abs() < 1e-4);
    } else {
        panic!("Expected RMC sentence");
    }
}

#[test]
fn test_nmea_checksum_rejection() {
    // Corrupted checksum: *6A changed to *99
    let raw = "$GPRMC,123519,A,4807.0380,N,01131.0000,E,022.4,084.4,230324,003.1,W*99";
    assert!(NmeaParser::parse(raw).is_err(), "Parser must reject corrupted checksum");
}

#[test]
fn test_nmea_formatting_roundtrip() {
    let rmc = NmeaRmc {
        utc_time: "153000".to_string(),
        status: 'A',
        lat_deg: 37.7749,
        lon_deg: -122.4194,
        sog_knots: 12.5,
        cog_deg: 180.0,
        date: "241126".to_string(),
    };
    let formatted = NmeaParser::format_rmc(&rmc);
    assert!(NmeaParser::verify_checksum(&formatted), "Formatted RMC must have valid checksum");
    let re_parsed = NmeaParser::parse(&formatted).expect("Failed to re-parse formatted RMC");
    if let NmeaSentence::Rmc(r) = re_parsed {
        assert!((r.lat_deg - rmc.lat_deg).abs() < 1e-3);
        assert!((r.lon_deg - rmc.lon_deg).abs() < 1e-3);
        assert!((r.sog_knots - rmc.sog_knots).abs() < 1e-3);
    } else {
        panic!("Expected RMC sentence");
    }
}

#[test]
fn test_ais_6bit_unpacking() {
    // Basic 6-bit char test
    assert_eq!(AisDecoder::decode_6bit_char(b'0').unwrap(), 0);
    assert_eq!(AisDecoder::decode_6bit_char(b'9').unwrap(), 9);
    assert_eq!(AisDecoder::decode_6bit_char(b':').unwrap(), 10);
    assert_eq!(AisDecoder::decode_6bit_char(b'w').unwrap(), 63);
}
