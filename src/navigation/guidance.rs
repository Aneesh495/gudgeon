//! Autonomous Line-of-Sight (LOS) Navigation and Path Tracking
//! Implements adaptive cross-track integral action, crab angle wind compensation,
//! and waypoint switching corridors.

#[derive(Clone, Debug, PartialEq)]
pub struct Waypoint {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub radius_m: f64,
    pub target_speed_knots: f64,
}

#[derive(Clone, Debug)]
pub struct LosGuidance {
    pub waypoints: Vec<Waypoint>,
    pub current_wp_idx: usize,
    pub lookahead_dist_m: f64,
    pub cross_track_integral_gain: f64,
    pub accumulated_xte_m: f64,
    pub max_crab_angle_deg: f64,
}

impl LosGuidance {
    pub fn new(waypoints: Vec<Waypoint>, lookahead_dist_m: f64) -> Self {
        Self {
            waypoints,
            current_wp_idx: 0,
            lookahead_dist_m,
            cross_track_integral_gain: 0.002,
            accumulated_xte_m: 0.0,
            max_crab_angle_deg: 35.0,
        }
    }

    /// Computes Great Circle distance between two points in meters
    pub fn distance_meters(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r_earth = 6_371_000.0;
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).max(0.0).sqrt());
        r_earth * c
    }

    /// Computes True Bearing from point 1 to point 2 in degrees [0, 360)
    pub fn bearing_deg(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let l1 = lat1.to_radians();
        let l2 = lat2.to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let y = dlon.sin() * l2.cos();
        let x = l1.cos() * l2.sin() - l1.sin() * l2.cos() * dlon.cos();
        let mut b = y.atan2(x).to_degrees();
        if b < 0.0 {
            b += 360.0;
        }
        b
    }

    /// Computes cross-track error (meters) relative to segment between W1 and W2.
    /// Positive value means vessel is to starboard (right) of path.
    pub fn cross_track_error_m(lat_cur: f64, lon_cur: f64, lat_w1: f64, lon_w1: f64, lat_w2: f64, lon_w2: f64) -> f64 {
        let dist_to_w1 = Self::distance_meters(lat_w1, lon_w1, lat_cur, lon_cur);
        let bearing_to_cur = Self::bearing_deg(lat_w1, lon_w1, lat_cur, lon_cur).to_radians();
        let bearing_path = Self::bearing_deg(lat_w1, lon_w1, lat_w2, lon_w2).to_radians();
        dist_to_w1 * (bearing_to_cur - bearing_path).sin()
    }

    /// Updates guidance and computes desired heading in degrees [0, 360)
    pub fn update(&mut self, lat_cur: f64, lon_cur: f64, dt: f64) -> (f64, f64, bool) {
        if self.waypoints.is_empty() {
            return (0.0, 0.0, true);
        }

        let target_wp = &self.waypoints[self.current_wp_idx];
        let dist_to_target = Self::distance_meters(lat_cur, lon_cur, target_wp.lat_deg, target_wp.lon_deg);

        // Check if inside acceptance radius
        if dist_to_target <= target_wp.radius_m {
            if self.current_wp_idx + 1 < self.waypoints.len() {
                self.current_wp_idx += 1;
                self.accumulated_xte_m = 0.0;
            } else {
                // Route complete
                return (0.0, 0.0, true);
            }
        }

        let target_wp = &self.waypoints[self.current_wp_idx];
        let prev_wp = if self.current_wp_idx == 0 {
            target_wp
        } else {
            &self.waypoints[self.current_wp_idx - 1]
        };

        let path_course = Self::bearing_deg(prev_wp.lat_deg, prev_wp.lon_deg, target_wp.lat_deg, target_wp.lon_deg);
        let xte = Self::cross_track_error_m(
            lat_cur,
            lon_cur,
            prev_wp.lat_deg,
            prev_wp.lon_deg,
            target_wp.lat_deg,
            target_wp.lon_deg,
        );

        // Integrate cross track error for drift compensation
        self.accumulated_xte_m += xte * dt;
        let integral_correction = self.cross_track_integral_gain * self.accumulated_xte_m;

        // Line-of-sight steering angle: chi = path_course - arctan((xte + int) / delta)
        let delta = self.lookahead_dist_m.max(10.0);
        let effective_error = xte + integral_correction;
        let crab_angle = (-effective_error / delta)
            .atan()
            .to_degrees()
            .clamp(-self.max_crab_angle_deg, self.max_crab_angle_deg);

        let mut desired_heading = path_course + crab_angle;
        while desired_heading >= 360.0 {
            desired_heading -= 360.0;
        }
        while desired_heading < 0.0 {
            desired_heading += 360.0;
        }

        (desired_heading, target_wp.target_speed_knots, false)
    }
}
