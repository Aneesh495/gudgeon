//! NMEA 0183 Marine Sensor Protocol Parser and Formatter
//! Implements IEC 61162 / NMEA 0183 standard sentences (GGA, RMC, HDT, ROT, VTG, VHW)
//! with mandatory XOR checksum verification.

#[derive(Clone, Debug, PartialEq)]
pub struct NmeaRmc {
    pub utc_time: String,
    pub status: char, // 'A' = Valid, 'V' = Warning
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub sog_knots: f64,
    pub cog_deg: f64,
    pub date: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NmeaGga {
    pub utc_time: String,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub fix_quality: u8, // 1 = GPS fix, 2 = DGPS fix
    pub num_satellites: u8,
    pub hdop: f64,
    pub altitude_m: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NmeaSentence {
    Rmc(NmeaRmc),
    Gga(NmeaGga),
    Hdt { heading_deg: f64 },
    Rot { rate_of_turn_deg_min: f64 },
    Vtg { cog_true_deg: f64, sog_knots: f64 },
    Unknown(String),
}

pub struct NmeaParser;

impl NmeaParser {
    pub fn compute_checksum(sentence_content: &str) -> u8 {
        let mut sum: u8 = 0;
        let mut active = false;
        for b in sentence_content.bytes() {
            if b == b'$' || b == b'!' {
                active = true;
                continue;
            }
            if b == b'*' {
                break;
            }
            if active {
                sum ^= b;
            }
        }
        sum
    }

    pub fn format_checksum(sentence_content: &str) -> String {
        let chk = Self::compute_checksum(sentence_content);
        format!("{:02X}", chk)
    }

    pub fn verify_checksum(sentence: &str) -> bool {
        let trimmed = sentence.trim();
        if let Some(star_idx) = trimmed.rfind('*') {
            let content = &trimmed[..star_idx];
            let expected_hex = &trimmed[star_idx + 1..];
            if expected_hex.len() >= 2 {
                if let Ok(expected) = u8::from_str_radix(&expected_hex[..2], 16) {
                    return Self::compute_checksum(content) == expected;
                }
            }
        }
        false
    }

    pub fn parse(line: &str) -> Result<NmeaSentence, String> {
        let trimmed = line.trim();
        if !Self::verify_checksum(trimmed) {
            return Err("NMEA checksum verification failed".to_string());
        }

        let star_idx = trimmed.rfind('*').unwrap_or(trimmed.len());
        let payload = &trimmed[..star_idx];
        let fields: Vec<&str> = payload.split(',').collect();
        if fields.is_empty() {
            return Err("Empty NMEA sentence".to_string());
        }

        let talker_type = fields[0].trim_start_matches(['$', '!']);
        match talker_type {
            t if t.ends_with("RMC") => Self::parse_rmc(&fields),
            t if t.ends_with("GGA") => Self::parse_gga(&fields),
            t if t.ends_with("HDT") => Self::parse_hdt(&fields),
            t if t.ends_with("ROT") => Self::parse_rot(&fields),
            t if t.ends_with("VTG") => Self::parse_vtg(&fields),
            _ => Ok(NmeaSentence::Unknown(talker_type.to_string())),
        }
    }

    fn parse_coordinate(coord_str: &str, hemi: &str, is_lon: bool) -> Option<f64> {
        if coord_str.is_empty() {
            return None;
        }
        let deg_len = if is_lon { 3 } else { 2 };
        if coord_str.len() < deg_len {
            return None;
        }
        let deg: f64 = coord_str[..deg_len].parse().ok()?;
        let min: f64 = coord_str[deg_len..].parse().ok()?;
        let mut decimal = deg + (min / 60.0);
        if hemi == "S" || hemi == "W" {
            decimal = -decimal;
        }
        Some(decimal)
    }

    fn parse_rmc(fields: &[&str]) -> Result<NmeaSentence, String> {
        if fields.len() < 10 {
            return Err("Malformed RMC sentence: insufficient fields".to_string());
        }
        let utc_time = fields[1].to_string();
        let status = fields[2].chars().next().unwrap_or('V');
        let lat = Self::parse_coordinate(fields[3], fields[4], false).unwrap_or(0.0);
        let lon = Self::parse_coordinate(fields[5], fields[6], true).unwrap_or(0.0);
        let sog: f64 = fields[7].parse().unwrap_or(0.0);
        let cog: f64 = fields[8].parse().unwrap_or(0.0);
        let date = fields[9].to_string();

        Ok(NmeaSentence::Rmc(NmeaRmc {
            utc_time,
            status,
            lat_deg: lat,
            lon_deg: lon,
            sog_knots: sog,
            cog_deg: cog,
            date,
        }))
    }

    fn parse_gga(fields: &[&str]) -> Result<NmeaSentence, String> {
        if fields.len() < 10 {
            return Err("Malformed GGA sentence: insufficient fields".to_string());
        }
        let utc_time = fields[1].to_string();
        let lat = Self::parse_coordinate(fields[2], fields[3], false).unwrap_or(0.0);
        let lon = Self::parse_coordinate(fields[4], fields[5], true).unwrap_or(0.0);
        let fix_quality: u8 = fields[6].parse().unwrap_or(0);
        let num_satellites: u8 = fields[7].parse().unwrap_or(0);
        let hdop: f64 = fields[8].parse().unwrap_or(99.9);
        let altitude_m: f64 = fields[9].parse().unwrap_or(0.0);

        Ok(NmeaSentence::Gga(NmeaGga {
            utc_time,
            lat_deg: lat,
            lon_deg: lon,
            fix_quality,
            num_satellites,
            hdop,
            altitude_m,
        }))
    }

    fn parse_hdt(fields: &[&str]) -> Result<NmeaSentence, String> {
        if fields.len() < 2 {
            return Err("Malformed HDT sentence".to_string());
        }
        let heading: f64 = fields[1].parse().unwrap_or(0.0);
        Ok(NmeaSentence::Hdt { heading_deg: heading })
    }

    fn parse_rot(fields: &[&str]) -> Result<NmeaSentence, String> {
        if fields.len() < 2 {
            return Err("Malformed ROT sentence".to_string());
        }
        let rot: f64 = fields[1].parse().unwrap_or(0.0);
        Ok(NmeaSentence::Rot { rate_of_turn_deg_min: rot })
    }

    fn parse_vtg(fields: &[&str]) -> Result<NmeaSentence, String> {
        if fields.len() < 8 {
            return Err("Malformed VTG sentence".to_string());
        }
        let cog: f64 = fields[1].parse().unwrap_or(0.0);
        let sog: f64 = fields[5].parse().unwrap_or(0.0);
        Ok(NmeaSentence::Vtg {
            cog_true_deg: cog,
            sog_knots: sog,
        })
    }

    pub fn format_rmc(rmc: &NmeaRmc) -> String {
        let lat_deg = rmc.lat_deg.abs().floor();
        let lat_min = (rmc.lat_deg.abs() - lat_deg) * 60.0;
        let lat_hemi = if rmc.lat_deg >= 0.0 { "N" } else { "S" };

        let lon_deg = rmc.lon_deg.abs().floor();
        let lon_min = (rmc.lon_deg.abs() - lon_deg) * 60.0;
        let lon_hemi = if rmc.lon_deg >= 0.0 { "E" } else { "W" };

        let payload = format!(
            "GPRMC,{},{},{:02}{:07.4},{},{:03}{:07.4},{},{:.1},{:.1},{},,E",
            rmc.utc_time,
            rmc.status,
            lat_deg as u32,
            lat_min,
            lat_hemi,
            lon_deg as u32,
            lon_min,
            lon_hemi,
            rmc.sog_knots,
            rmc.cog_deg,
            rmc.date
        );
        let chk = Self::format_checksum(&format!("${}", payload));
        format!("${}*{}", payload, chk)
    }

    pub fn format_hdt(heading_deg: f64) -> String {
        let payload = format!("GPHDT,{:.1},T", heading_deg);
        let chk = Self::format_checksum(&format!("${}", payload));
        format!("${}*{}", payload, chk)
    }
}
