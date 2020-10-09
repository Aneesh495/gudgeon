//! IMO COLREGS 1972 Autonomous Collision Avoidance Engine
//! Implements Rule 8 (Action to Avoid Collision), Rule 13 (Overtaking),
//! Rule 14 (Head-on), Rule 15 (Crossing / Give-way), and Rule 17 (Stand-on).

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColregsSituation {
    Clear,
    OvertakingStandOn, // Rule 13: We are being overtaken
    OvertakingGiveWay, // Rule 13: We are overtaking
    HeadOn,            // Rule 14: Head-on encounter
    CrossingGiveWay,   // Rule 15: Target to starboard
    CrossingStandOn,   // Rule 17: Target to port
    EmergencyInExtremis, // Rule 17(b): Stand-on vessel must take action
}

#[derive(Clone, Debug)]
pub struct ObstacleContact {
    pub id: u32,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub speed_knots: f64,
    pub course_deg: f64,
    pub range_nm: f64,
    pub bearing_deg: f64,
    pub cpa_nm: f64,
    pub tcpa_sec: f64,
    pub situation: ColregsSituation,
}

pub struct ColregsEngine {
    pub safe_cpa_nm: f64,
    pub safe_tcpa_sec: f64,
    pub max_eval_range_nm: f64,
}

impl Default for ColregsEngine {
    fn default() -> Self {
        Self {
            safe_cpa_nm: 0.8,      // 0.8 nautical miles safe passing distance
            safe_tcpa_sec: 900.0,   // 15 minutes lookahead window
            max_eval_range_nm: 6.0, // Evaluate targets within 6 nm
        }
    }
}

impl ColregsEngine {
    pub fn new(safe_cpa_nm: f64, safe_tcpa_sec: f64) -> Self {
        Self {
            safe_cpa_nm,
            safe_tcpa_sec,
            max_eval_range_nm: 6.0,
        }
    }

    /// Evaluates CPA and TCPA between own vessel and target contact
    pub fn evaluate_cpa_tcpa(
        own_lat: f64,
        own_lon: f64,
        own_spd_kts: f64,
        own_cog_deg: f64,
        tgt_lat: f64,
        tgt_lon: f64,
        tgt_spd_kts: f64,
        tgt_cog_deg: f64,
    ) -> (f64, f64, f64, f64) {
        // Distance in nautical miles
        let dlat = (tgt_lat - own_lat) * 60.0;
        let dlon = (tgt_lon - own_lon) * 60.0 * (own_lat.to_radians().cos());
        let range_nm = (dlat * dlat + dlon * dlon).sqrt();

        let mut bearing_deg = dlon.atan2(dlat).to_degrees();
        if bearing_deg < 0.0 {
            bearing_deg += 360.0;
        }

        // Relative velocity components in knots
        let own_vx = own_spd_kts * own_cog_deg.to_radians().sin();
        let own_vy = own_spd_kts * own_cog_deg.to_radians().cos();

        let tgt_vx = tgt_spd_kts * tgt_cog_deg.to_radians().sin();
        let tgt_vy = tgt_spd_kts * tgt_cog_deg.to_radians().cos();

        let dvx = tgt_vx - own_vx;
        let dvy = tgt_vy - own_vy;
        let dv_sq = dvx * dvx + dvy * dvy;

        if dv_sq < 1e-4 {
            return (range_nm, 9999.0, range_nm, bearing_deg);
        }

        // Relative position vector: target relative to own vessel
        let rx = dlon;
        let ry = dlat;

        // TCPA in hours, then convert to seconds
        let tcpa_hours = -(rx * dvx + ry * dvy) / dv_sq;
        let tcpa_sec = tcpa_hours * 3600.0;

        let cpa_x = rx + dvx * tcpa_hours.max(0.0);
        let cpa_y = ry + dvy * tcpa_hours.max(0.0);
        let cpa_nm = (cpa_x * cpa_x + cpa_y * cpa_y).sqrt();

        (cpa_nm, tcpa_sec, range_nm, bearing_deg)
    }

    /// Classifies the encounter situation according to COLREGS definitions
    pub fn classify_encounter(
        own_heading_deg: f64,
        tgt_bearing_deg: f64,
        tgt_heading_deg: f64,
        range_nm: f64,
        cpa_nm: f64,
        tcpa_sec: f64,
        safe_cpa_nm: f64,
        safe_tcpa_sec: f64,
    ) -> ColregsSituation {
        if range_nm > 6.0 || tcpa_sec < 0.0 || (tcpa_sec > safe_tcpa_sec && cpa_nm > safe_cpa_nm) {
            return ColregsSituation::Clear;
        }

        // Relative bearing [-180, 180] from own ship heading
        let mut rel_brg = tgt_bearing_deg - own_heading_deg;
        while rel_brg > 180.0 {
            rel_brg -= 360.0;
        }
        while rel_brg < -180.0 {
            rel_brg += 360.0;
        }

        // Reciprocal heading difference: head-on courses differ by approximately 180 degrees
        let mut course_diff = (tgt_heading_deg - own_heading_deg).abs() % 360.0;
        if course_diff > 180.0 {
            course_diff = 360.0 - course_diff;
        }

        // Rule 17(b): In extremis condition if CPA < 0.2 nm and TCPA < 180s
        if cpa_nm < 0.25 && tcpa_sec < 180.0 {
            return ColregsSituation::EmergencyInExtremis;
        }

        // Rule 13: Overtaking (Target is more than 22.5 degrees abaft beam -> |rel_brg| > 112.5)
        if rel_brg.abs() > 112.5 {
            return ColregsSituation::OvertakingStandOn;
        }

        // Rule 14: Head-on (reciprocal or nearly reciprocal courses within +/- 15 deg)
        if rel_brg.abs() <= 15.0 && course_diff >= 165.0 {
            return ColregsSituation::HeadOn;
        }

        // Rule 15: Crossing. If target is on our starboard side (5 to 112.5 deg) -> We are Give-way
        if rel_brg > 5.0 && rel_brg <= 112.5 {
            return ColregsSituation::CrossingGiveWay;
        }

        // Rule 17: Crossing. If target is on our port side (-112.5 to -5 deg) -> We are Stand-on
        if rel_brg < -5.0 && rel_brg >= -112.5 {
            return ColregsSituation::CrossingStandOn;
        }

        ColregsSituation::Clear
    }

    /// Computes safe evasive heading recommendation based on COLREGS
    pub fn compute_evasion_heading(
        own_heading_deg: f64,
        situation: ColregsSituation,
        cpa_nm: f64,
        safe_cpa_nm: f64,
    ) -> f64 {
        match situation {
            ColregsSituation::Clear | ColregsSituation::CrossingStandOn | ColregsSituation::OvertakingStandOn => {
                own_heading_deg // Maintain course and speed
            }
            ColregsSituation::HeadOn => {
                // Rule 14: Alter course to starboard (right) by at least 30 to 45 degrees
                let mut h = own_heading_deg + 35.0;
                if h >= 360.0 {
                    h -= 360.0;
                }
                h
            }
            ColregsSituation::CrossingGiveWay => {
                // Rule 15 & 16: Substantial alteration to starboard to pass astern of target
                let offset = if cpa_nm < 0.4 * safe_cpa_nm { 45.0 } else { 30.0 };
                let mut h = own_heading_deg + offset;
                if h >= 360.0 {
                    h -= 360.0;
                }
                h
            }
            ColregsSituation::EmergencyInExtremis => {
                // Rule 17(b): Aggressive hard starboard turn
                let mut h = own_heading_deg + 60.0;
                if h >= 360.0 {
                    h -= 360.0;
                }
                h
            }
            ColregsSituation::OvertakingGiveWay => {
                // Pass on safe side with at least 25 deg clearance
                let mut h = own_heading_deg + 25.0;
                if h >= 360.0 {
                    h -= 360.0;
                }
                h
            }
        }
    }
}
