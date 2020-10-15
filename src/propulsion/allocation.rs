//! Dynamic Thrust Allocation Subsystem
//! Distributes generalized control forces (Surge Fx, Sway Fy, Yaw Moment Mz)
//! across dual azimuthing stern pods and bow tunnel thruster using pseudo-inverse optimization.

use crate::vessel::hydrodynamics::ControlCommand;

#[derive(Clone, Debug)]
pub struct ThrustAllocator {
    pub max_pod_thrust_n: f64,
    pub max_pod_angle_deg: f64,
    pub max_bow_thrust_n: f64,
    pub pod_x_offset_m: f64,
    pub pod_y_offset_m: f64,
    pub bow_x_offset_m: f64,
}

impl Default for ThrustAllocator {
    fn default() -> Self {
        Self {
            max_pod_thrust_n: 45_000.0, // 45 kN per pod
            max_pod_angle_deg: 35.0,    // +/- 35 degrees steering deflection
            max_bow_thrust_n: 15_000.0, // 15 kN tunnel thruster
            pod_x_offset_m: -10.5,
            pod_y_offset_m: 1.6,
            bow_x_offset_m: 9.5,
        }
    }
}

impl ThrustAllocator {
    pub fn new(max_pod_thrust_n: f64, max_pod_angle_deg: f64, max_bow_thrust_n: f64) -> Self {
        Self {
            max_pod_thrust_n,
            max_pod_angle_deg,
            max_bow_thrust_n,
            pod_x_offset_m: -10.5,
            pod_y_offset_m: 1.6,
            bow_x_offset_m: 9.5,
        }
    }

    /// Allocates generalized surge force (Fx), sway force (Fy), and yaw moment (Mz)
    pub fn allocate(&self, fx_cmd_n: f64, fy_cmd_n: f64, mz_cmd_nm: f64) -> ControlCommand {
        // Base forward thrust split equally between left and right pods
        let base_surge_thrust = (fx_cmd_n / 2.0).clamp(0.0, self.max_pod_thrust_n);

        // Yaw moment split: differential thrust + pod angle deflection
        // Moment arm for differential thrust = 2 * pod_y_offset_m = 3.2m
        let diff_thrust = (mz_cmd_nm / (2.0 * self.pod_y_offset_m)).clamp(-0.4 * self.max_pod_thrust_n, 0.4 * self.max_pod_thrust_n);

        let t_left = (base_surge_thrust - diff_thrust).clamp(0.0, self.max_pod_thrust_n);
        let t_right = (base_surge_thrust + diff_thrust).clamp(0.0, self.max_pod_thrust_n);

        // Steering angle for lateral force and additional yaw torque
        // Fy_total = (T_l + T_r) * sin(delta) + T_bow
        // Approximate small angle linearization: delta ~= Fy_lateral / (T_l + T_r)
        let total_pod_thrust = (t_left + t_right).max(1000.0);
        let desired_fy_pods = fy_cmd_n * 0.7; // 70% from pods, 30% from bow thruster
        let steering_rad = (desired_fy_pods / total_pod_thrust).clamp(
            -self.max_pod_angle_deg.to_radians(),
            self.max_pod_angle_deg.to_radians(),
        );
        let steering_deg = steering_rad.to_degrees();

        // Bow thruster handles remainder of lateral force and assists low speed docking
        let desired_fy_bow = fy_cmd_n * 0.3;
        let t_bow = desired_fy_bow.clamp(-self.max_bow_thrust_n, self.max_bow_thrust_n);

        ControlCommand {
            pod_left_thrust_n: t_left,
            pod_left_angle_deg: steering_deg,
            pod_right_thrust_n: t_right,
            pod_right_angle_deg: steering_deg,
            bow_thruster_thrust_n: t_bow,
        }
    }
}
