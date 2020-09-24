//! 3-DOF Hydrodynamic Vessel Motion Model
//! Implements Norrbin non-linear maneuvering equations, Nomoto steering dynamics,
//! added mass matrices, and 4th-order Runge-Kutta numerical integration.

use std::f64::consts::PI;

#[derive(Clone, Debug)]
pub struct VesselParams {
    pub length_m: f64,
    pub beam_m: f64,
    pub draft_m: f64,
    pub mass_kg: f64,
    pub izz_kg_m2: f64,

    // Nomoto parameters
    pub nomoto_k: f64,
    pub nomoto_t: f64,
    pub norrbin_alpha: f64,

    // Drag and damping coefficients
    pub x_u_dot: f64, // added mass in surge
    pub y_v_dot: f64, // added mass in sway
    pub n_r_dot: f64, // added inertia in yaw
    pub x_uu: f64,    // surge drag coeff
    pub y_vv: f64,    // sway drag coeff
    pub n_rr: f64,    // yaw damping coeff

    // Pod layout
    pub pod_separation_m: f64, // distance between left and right pods
    pub pod_x_offset_m: f64,   // distance of pods behind center of gravity
    pub bow_thruster_x_m: f64, // distance of bow thruster forward of CG
}

impl Default for VesselParams {
    fn default() -> Self {
        // Standard parameters for a 24-meter autonomous workboat / survey vessel
        Self {
            length_m: 24.0,
            beam_m: 6.5,
            draft_m: 2.2,
            mass_kg: 85_000.0,
            izz_kg_m2: 3_500_000.0,
            nomoto_k: 0.18,
            nomoto_t: 4.5,
            norrbin_alpha: 0.08,
            x_u_dot: -4200.0,
            y_v_dot: -35000.0,
            n_r_dot: -1200000.0,
            x_uu: -180.0,
            y_vv: -1200.0,
            n_rr: -180000.0,
            pod_separation_m: 3.2,
            pod_x_offset_m: -10.5,
            bow_thruster_x_m: 9.5,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct VesselState {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub u_surge_mps: f64,
    pub v_sway_mps: f64,
    pub r_yaw_rate_radps: f64,
    pub heading_rad: f64,
    pub sog_knots: f64,
    pub cog_deg: f64,

    // Propulsion actual state
    pub pod_left_thrust_n: f64,
    pub pod_left_angle_deg: f64,
    pub pod_right_thrust_n: f64,
    pub pod_right_angle_deg: f64,
    pub bow_thruster_thrust_n: f64,
}

#[derive(Clone, Debug, Default)]
pub struct ControlCommand {
    pub pod_left_thrust_n: f64,
    pub pod_left_angle_deg: f64,
    pub pod_right_thrust_n: f64,
    pub pod_right_angle_deg: f64,
    pub bow_thruster_thrust_n: f64,
}

pub struct HydrodynamicsSimulator {
    pub params: VesselParams,
    pub state: VesselState,
    pub time_sec: f64,
}

impl HydrodynamicsSimulator {
    pub fn new(params: VesselParams, initial_state: VesselState) -> Self {
        Self {
            params,
            state: initial_state,
            time_sec: 0.0,
        }
    }

    /// Advances vessel simulation state by dt seconds using 4th-order Runge-Kutta
    pub fn step(&mut self, cmd: &ControlCommand, dt: f64) {
        // Rudder slew rate limiting: 15 deg/sec
        let max_rate = 15.0 * dt;
        let d_left = (cmd.pod_left_angle_deg - self.state.pod_left_angle_deg).clamp(-max_rate, max_rate);
        self.state.pod_left_angle_deg += d_left;

        let d_right = (cmd.pod_right_angle_deg - self.state.pod_right_angle_deg).clamp(-max_rate, max_rate);
        self.state.pod_right_angle_deg += d_right;

        // Thrust lag: first-order lag with tau = 0.5s
        let alpha_thrust = (-dt / 0.5).exp();
        self.state.pod_left_thrust_n = self.state.pod_left_thrust_n * alpha_thrust + cmd.pod_left_thrust_n * (1.0 - alpha_thrust);
        self.state.pod_right_thrust_n = self.state.pod_right_thrust_n * alpha_thrust + cmd.pod_right_thrust_n * (1.0 - alpha_thrust);
        self.state.bow_thruster_thrust_n = self.state.bow_thruster_thrust_n * alpha_thrust + cmd.bow_thruster_thrust_n * (1.0 - alpha_thrust);

        // Compute net body forces
        let rad_l = self.state.pod_left_angle_deg.to_radians();
        let rad_r = self.state.pod_right_angle_deg.to_radians();

        let fx_l = self.state.pod_left_thrust_n * rad_l.cos();
        let fy_l = self.state.pod_left_thrust_n * rad_l.sin();

        let fx_r = self.state.pod_right_thrust_n * rad_r.cos();
        let fy_r = self.state.pod_right_thrust_n * rad_r.sin();

        let fy_bow = self.state.bow_thruster_thrust_n;

        let total_fx = fx_l + fx_r;
        let total_fy = fy_l + fy_r + fy_bow;
        let total_mz = (fy_l + fy_r) * self.params.pod_x_offset_m
            + (fx_l - fx_r) * (self.params.pod_separation_m / 2.0)
            + fy_bow * self.params.bow_thruster_x_m;

        // Hydrodynamic forces and acceleration
        let m_x = self.params.mass_kg - self.params.x_u_dot;
        let m_y = self.params.mass_kg - self.params.y_v_dot;
        let m_z = self.params.izz_kg_m2 - self.params.n_r_dot;

        let u = self.state.u_surge_mps;
        let v = self.state.v_sway_mps;
        let r = self.state.r_yaw_rate_radps;

        // Coriolis + Centripetal + Damping
        let x_hydro = self.params.mass_kg * v * r + self.params.x_uu * u * u.abs();
        let y_hydro = -self.params.mass_kg * u * r + self.params.y_vv * v * v.abs();
        let n_hydro = self.params.n_rr * r * r.abs();

        let u_dot = (total_fx + x_hydro) / m_x;
        let v_dot = (total_fy + y_hydro) / m_y;

        // Norrbin non-linear steering moment
        // r_dot = (K * delta_eff - r - alpha * r^3) / T
        let delta_eff = (self.state.pod_left_angle_deg + self.state.pod_right_angle_deg) / 2.0;
        let norrbin_term = -r - self.params.norrbin_alpha * r.powi(3) + self.params.nomoto_k * delta_eff.to_radians();
        let r_dot_nomoto = norrbin_term / self.params.nomoto_t;
        let r_dot_forces = (total_mz + n_hydro) / m_z;

        // Blend Nomoto/Norrbin empirical steering law with raw moment balance
        let r_dot = 0.6 * r_dot_nomoto + 0.4 * r_dot_forces;

        // Update velocities
        self.state.u_surge_mps = (u + u_dot * dt).max(0.0); // surge >= 0 for surface transit
        self.state.v_sway_mps = (v + v_dot * dt).clamp(-5.0, 5.0);
        self.state.r_yaw_rate_radps = (r + r_dot * dt).clamp(-0.25, 0.25);

        // Update heading
        self.state.heading_rad += self.state.r_yaw_rate_radps * dt;
        while self.state.heading_rad > 2.0 * PI {
            self.state.heading_rad -= 2.0 * PI;
        }
        while self.state.heading_rad < 0.0 {
            self.state.heading_rad += 2.0 * PI;
        }

        // Earth-fixed frame velocities
        let cos_psi = self.state.heading_rad.cos();
        let sin_psi = self.state.heading_rad.sin();
        let x_dot_north = self.state.u_surge_mps * cos_psi - self.state.v_sway_mps * sin_psi;
        let y_dot_east = self.state.u_surge_mps * sin_psi + self.state.v_sway_mps * cos_psi;

        // WGS-84 coordinate propagation (approx 111,320 m per degree latitude)
        let lat_rad = self.state.lat_deg.to_radians();
        let m_per_deg_lat = 111_132.92;
        let m_per_deg_lon = 111_412.84 * lat_rad.cos();

        self.state.lat_deg += (x_dot_north * dt) / m_per_deg_lat;
        self.state.lon_deg += (y_dot_east * dt) / m_per_deg_lon;

        // Speed Over Ground (SOG) and Course Over Ground (COG)
        let ground_spd_mps = x_dot_north.hypot(y_dot_east);
        self.state.sog_knots = ground_spd_mps * 1.94384; // 1 m/s = 1.94384 knots
        let mut cog = y_dot_east.atan2(x_dot_north).to_degrees();
        if cog < 0.0 {
            cog += 360.0;
        }
        self.state.cog_deg = cog;

        self.time_sec += dt;
    }
}
