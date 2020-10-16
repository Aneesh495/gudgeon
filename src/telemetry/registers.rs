//! Telemetry Register Map and Hardware State Memory
//! 256-channel memory-mapped register bank for real-time telemetry,
//! inter-process control, and hardware-in-the-loop (HIL) qualification.

pub const REG_SYS_STATUS: usize = 0;
pub const REG_SYS_MODE: usize = 1;
pub const REG_WATCHDOG_COUNT: usize = 2;
pub const REG_FAULT_FLAGS: usize = 3;
pub const REG_TIME_SEC_LOW: usize = 4;
pub const REG_TIME_SEC_HIGH: usize = 5;

pub const REG_LAT_DEG: usize = 16;
pub const REG_LON_DEG: usize = 17;
pub const REG_ALT_M: usize = 18;
pub const REG_SOG_KNOTS: usize = 19;
pub const REG_COG_DEG: usize = 20;
pub const REG_HEADING_DEG: usize = 21;
pub const REG_YAW_RATE_DEG_S: usize = 22;
pub const REG_SURGE_VEL_MPS: usize = 23;
pub const REG_SWAY_VEL_MPS: usize = 24;

pub const REG_TARGET_WP_IDX: usize = 48;
pub const REG_CROSS_TRACK_ERR_M: usize = 49;
pub const REG_ALONG_TRACK_DIST_M: usize = 50;
pub const REG_DESIRED_HEADING_DEG: usize = 51;
pub const REG_DESIRED_SPEED_KTS: usize = 52;

pub const REG_COLREGS_SITUATION: usize = 64;
pub const REG_COLREGS_MIN_CPA_NM: usize = 65;
pub const REG_COLREGS_MIN_TCPA_S: usize = 66;
pub const REG_COLREGS_ACTIVE_TARGETS: usize = 67;

pub const REG_CMD_THRUST_LEFT_N: usize = 80;
pub const REG_CMD_ANGLE_LEFT_DEG: usize = 81;
pub const REG_CMD_THRUST_RIGHT_N: usize = 82;
pub const REG_CMD_ANGLE_RIGHT_DEG: usize = 83;
pub const REG_CMD_THRUST_BOW_N: usize = 84;

#[derive(Clone, Debug)]
pub struct RegisterBank {
    pub words: [f64; 256],
}

impl Default for RegisterBank {
    fn default() -> Self {
        Self { words: [0.0; 256] }
    }
}

impl RegisterBank {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&self, offset: usize) -> f64 {
        if offset < 256 {
            self.words[offset]
        } else {
            0.0
        }
    }

    pub fn write(&mut self, offset: usize, value: f64) {
        if offset < 256 {
            self.words[offset] = value;
        }
    }

    pub fn snapshot(&self) -> [f64; 256] {
        self.words
    }

    pub fn reset_faults(&mut self) {
        self.words[REG_FAULT_FLAGS] = 0.0;
    }

    pub fn set_fault(&mut self, bit: u32) {
        let current = self.words[REG_FAULT_FLAGS] as u32;
        self.words[REG_FAULT_FLAGS] = (current | (1 << bit)) as f64;
    }
}
