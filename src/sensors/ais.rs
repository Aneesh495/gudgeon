//! AIS (Automatic Identification System) Decoder and Target Tracking Engine
//! Implements ITU-R M.1371 6-bit armored ASCII unpacking for Class A/B position reports.

#[derive(Clone, Debug, PartialEq)]
pub struct AisTarget {
    pub mmsi: u32,
    pub nav_status: u8,
    pub rate_of_turn: f64,
    pub speed_over_ground_knots: f64,
    pub lon_deg: f64,
    pub lat_deg: f64,
    pub course_over_ground_deg: f64,
    pub true_heading_deg: f64,
    pub timestamp_utc: u64,
}

pub struct AisDecoder;

impl AisDecoder {
    /// Converts a single armored ASCII character into its 6-bit integer representation
    pub fn decode_6bit_char(c: u8) -> Result<u8, String> {
        if c < 48 || (c > 87 && c < 96) || c > 119 {
            return Err(format!("Invalid AIS ASCII armored character: {}", c as char));
        }
        let mut v = c - 48;
        if v > 40 {
            v -= 8;
        }
        Ok(v)
    }

    /// Unpacks an armored AIS payload string into a bit vector
    pub fn unpack_payload(payload: &str) -> Result<Vec<bool>, String> {
        let mut bits = Vec::with_capacity(payload.len() * 6);
        for byte in payload.bytes() {
            let val = Self::decode_6bit_char(byte)?;
            for i in (0..6).rev() {
                bits.push((val & (1 << i)) != 0);
            }
        }
        Ok(bits)
    }

    fn read_bits_u32(bits: &[bool], start: usize, len: usize) -> u32 {
        let mut result = 0;
        for i in 0..len {
            if let Some(&b) = bits.get(start + i) {
                if b {
                    result |= 1 << (len - 1 - i);
                }
            }
        }
        result
    }

    fn read_bits_i32(bits: &[bool], start: usize, len: usize) -> i32 {
        let unsigned = Self::read_bits_u32(bits, start, len);
        let sign_bit = 1 << (len - 1);
        if (unsigned & sign_bit) != 0 {
            (unsigned as i32) - (1 << len)
        } else {
            unsigned as i32
        }
    }

    /// Decodes an ITU-R M.1371 Message Type 1, 2, or 3 Position Report
    pub fn decode_position_report(payload: &str) -> Result<AisTarget, String> {
        let bits = Self::unpack_payload(payload)?;
        if bits.len() < 168 {
            return Err(format!("AIS payload too short for position report: {} bits", bits.len()));
        }

        let msg_type = Self::read_bits_u32(&bits, 0, 6);
        if msg_type != 1 && msg_type != 2 && msg_type != 3 {
            return Err(format!("Unsupported AIS message type: {}", msg_type));
        }

        let mmsi = Self::read_bits_u32(&bits, 8, 30);
        let nav_status = Self::read_bits_u32(&bits, 38, 4) as u8;
        let rot_raw = Self::read_bits_i32(&bits, 42, 8);
        let sog_raw = Self::read_bits_u32(&bits, 50, 10);
        let lon_raw = Self::read_bits_i32(&bits, 61, 28);
        let lat_raw = Self::read_bits_i32(&bits, 89, 27);
        let cog_raw = Self::read_bits_u32(&bits, 116, 12);
        let hdg_raw = Self::read_bits_u32(&bits, 128, 9);

        // Scaling conversions according to ITU-R M.1371
        let sog_knots = (sog_raw as f64) / 10.0;
        let lon_deg = (lon_raw as f64) / 600_000.0;
        let lat_deg = (lat_raw as f64) / 600_000.0;
        let cog_deg = (cog_raw as f64) / 10.0;
        let true_heading_deg = if hdg_raw <= 359 { hdg_raw as f64 } else { cog_deg };
        let rate_of_turn = (rot_raw as f64) / 4.733;

        Ok(AisTarget {
            mmsi,
            nav_status,
            rate_of_turn,
            speed_over_ground_knots: sog_knots,
            lon_deg,
            lat_deg,
            course_over_ground_deg: cog_deg,
            true_heading_deg,
            timestamp_utc: 0,
        })
    }
}
