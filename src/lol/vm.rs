//! LOLCODE Virtual Machine and Deterministic Execution Runtime
//! Supports native AST evaluation and bytecode execution with maritime navigation builtins.

use crate::lol::ast::*;
use crate::lol::value::LolValue;
use std::collections::HashMap;

pub struct LolVM {
    pub globals: HashMap<String, LolValue>,
    pub functions: HashMap<String, (Vec<String>, Vec<Stmt>)>,
    pub output_buffer: Vec<String>,
    pub registers: [f64; 256],
    pub step_count: u64,
}

impl Default for LolVM {
    fn default() -> Self {
        Self::new()
    }
}

impl LolVM {
    pub fn new() -> Self {
        let mut vm = Self {
            globals: HashMap::new(),
            functions: HashMap::new(),
            output_buffer: Vec::new(),
            registers: [0.0; 256],
            step_count: 0,
        };
        vm.globals.insert("IT".to_string(), LolValue::Noob);
        vm
    }

    pub fn set_register(&mut self, reg: usize, val: f64) {
        if reg < 256 {
            self.registers[reg] = val;
        }
    }

    pub fn get_register(&self, reg: usize) -> f64 {
        if reg < 256 {
            self.registers[reg]
        } else {
            0.0
        }
    }

    pub fn run_program(&mut self, prog: &Program) -> Result<LolValue, String> {
        let mut last_val = LolValue::Noob;
        let mut top_locals = HashMap::new();
        for stmt in &prog.statements {
            let res = self.exec_statement(stmt, &mut top_locals)?;
            if let Some(ret) = res {
                last_val = ret;
                break;
            }
            if let Some(it) = self.globals.get("IT") {
                last_val = it.clone();
            }
        }
        Ok(last_val)
    }

    fn exec_statement(
        &mut self,
        stmt: &Stmt,
        locals: &mut HashMap<String, LolValue>,
    ) -> Result<Option<LolValue>, String> {
        self.step_count += 1;
        match stmt {
            Stmt::VarDecl { name, init } => {
                let val = if let Some(expr) = init {
                    self.eval_expr(expr, locals)?
                } else {
                    LolValue::Noob
                };
                locals.insert(name.clone(), val);
                Ok(None)
            }
            Stmt::Assignment { name, value } => {
                let val = self.eval_expr(value, locals)?;
                if locals.contains_key(name) {
                    locals.insert(name.clone(), val);
                } else {
                    self.globals.insert(name.clone(), val);
                }
                Ok(None)
            }
            Stmt::ArraySet { target, index, value } => {
                let idx = self.eval_expr(index, locals)?.to_numbr() as usize;
                let val = self.eval_expr(value, locals)?;
                if let Some(LolValue::Buket(arr)) = locals.get_mut(target) {
                    if idx < arr.len() {
                        arr[idx] = val;
                    }
                } else if let Some(LolValue::Buket(arr)) = self.globals.get_mut(target) {
                    if idx < arr.len() {
                        arr[idx] = val;
                    }
                }
                Ok(None)
            }
            Stmt::Print { args, newline } => {
                let mut line = String::new();
                for (i, arg) in args.iter().enumerate() {
                    let v = self.eval_expr(arg, locals)?;
                    if i > 0 {
                        line.push(' ');
                    }
                    line.push_str(&v.to_yarn());
                }
                if *newline {
                    self.output_buffer.push(line);
                } else if let Some(last) = self.output_buffer.last_mut() {
                    last.push_str(&line);
                } else {
                    self.output_buffer.push(line);
                }
                Ok(None)
            }
            Stmt::Input { name } => {
                let default_val = LolValue::Yarn("0".to_string());
                if locals.contains_key(name) {
                    locals.insert(name.clone(), default_val);
                } else {
                    self.globals.insert(name.clone(), default_val);
                }
                Ok(None)
            }
            Stmt::FuncDef { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
                Ok(None)
            }
            Stmt::Return { val } => {
                let ret = if let Some(expr) = val {
                    self.eval_expr(expr, locals)?
                } else {
                    LolValue::Noob
                };
                Ok(Some(ret))
            }
            Stmt::Break => Ok(Some(LolValue::Noob)),
            Stmt::Loop { op, var, cond, body, .. } => {
                let mut loop_count = 0;
                loop {
                    loop_count += 1;
                    if loop_count > 1_000_000 {
                        return Err("Infinite loop detected: exceeded 1,000,000 iterations".to_string());
                    }

                    // Check condition before body
                    if let Some(c) = cond {
                        let c_val = self.eval_expr(&c.expr, locals)?.to_troof();
                        match c.kind {
                            LoopCondKind::Til => {
                                if c_val {
                                    break;
                                }
                            }
                            LoopCondKind::Wile => {
                                if !c_val {
                                    break;
                                }
                            }
                        }
                    }

                    // Execute body
                    let mut broken = false;
                    for s in body {
                        if let Stmt::Break = s {
                            broken = true;
                            break;
                        }
                        if let Some(ret) = self.exec_statement(s, locals)? {
                            return Ok(Some(ret));
                        }
                    }
                    if broken {
                        break;
                    }

                    // Increment / decrement loop variable
                    if let (Some(operation), Some(vname)) = (op, var) {
                        let current = self.get_var(vname, locals);
                        let next = match operation {
                            LoopOp::Uppin => current.add(&LolValue::Numbr(1)),
                            LoopOp::Nerfin => current.sub(&LolValue::Numbr(1)),
                        };
                        self.set_var(vname, next, locals);
                    }
                }
                Ok(None)
            }
            Stmt::IfThen { then_branch, mebbe_branches, else_branch, .. } => {
                let it_cond = self.globals.get("IT").cloned().unwrap_or(LolValue::Noob).to_troof();
                if it_cond {
                    for s in then_branch {
                        if let Some(ret) = self.exec_statement(s, locals)? {
                            return Ok(Some(ret));
                        }
                    }
                } else {
                    let mut matched = false;
                    for (m_expr, m_body) in mebbe_branches {
                        if self.eval_expr(m_expr, locals)?.to_troof() {
                            matched = true;
                            for s in m_body {
                                if let Some(ret) = self.exec_statement(s, locals)? {
                                    return Ok(Some(ret));
                                }
                            }
                            break;
                        }
                    }
                    if !matched {
                        if let Some(eb) = else_branch {
                            for s in eb {
                                if let Some(ret) = self.exec_statement(s, locals)? {
                                    return Ok(Some(ret));
                                }
                            }
                        }
                    }
                }
                Ok(None)
            }
            Stmt::Switch { cases, default } => {
                let it_val = self.globals.get("IT").cloned().unwrap_or(LolValue::Noob);
                let mut matched = false;
                for (c_val, c_body) in cases {
                    if it_val.equal_to(c_val) || matched {
                        matched = true;
                        let mut broken = false;
                        for s in c_body {
                            if let Stmt::Break = s {
                                broken = true;
                                break;
                            }
                            if let Some(ret) = self.exec_statement(s, locals)? {
                                return Ok(Some(ret));
                            }
                        }
                        if broken {
                            break;
                        }
                    }
                }
                if !matched {
                    if let Some(def) = default {
                        for s in def {
                            if let Some(ret) = self.exec_statement(s, locals)? {
                                return Ok(Some(ret));
                            }
                        }
                    }
                }
                Ok(None)
            }
            Stmt::ExprStmt(expr) => {
                let val = self.eval_expr(expr, locals)?;
                self.globals.insert("IT".to_string(), val);
                Ok(None)
            }
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr, locals: &mut HashMap<String, LolValue>) -> Result<LolValue, String> {
        match expr {
            Expr::Lit(v) => Ok(v.clone()),
            Expr::Var(name) => Ok(self.get_var(name, locals)),
            Expr::Unary(op, e) => {
                let v = self.eval_expr(e, locals)?;
                match op {
                    UnaryOp::Not => Ok(LolValue::Troof(!v.to_troof())),
                }
            }
            Expr::Binary(op, left, right) => {
                let l = self.eval_expr(left, locals)?;
                let r = self.eval_expr(right, locals)?;
                let res = match op {
                    BinaryOp::Sum => l.add(&r),
                    BinaryOp::Diff => l.sub(&r),
                    BinaryOp::Produkt => l.mul(&r),
                    BinaryOp::Quoshunt => l.div(&r),
                    BinaryOp::Mod => l.modulo(&r),
                    BinaryOp::Biggr => l.max(&r),
                    BinaryOp::Smallr => l.min(&r),
                    BinaryOp::Both => LolValue::Troof(l.to_troof() && r.to_troof()),
                    BinaryOp::Either => LolValue::Troof(l.to_troof() || r.to_troof()),
                    BinaryOp::Won => LolValue::Troof(l.to_troof() ^ r.to_troof()),
                    BinaryOp::BothSaem => LolValue::Troof(l.equal_to(&r)),
                    BinaryOp::Diffrint => LolValue::Troof(!l.equal_to(&r)),
                };
                Ok(res)
            }
            Expr::Nary(op, exprs) => {
                match op {
                    NaryOp::All => {
                        for e in exprs {
                            if !self.eval_expr(e, locals)?.to_troof() {
                                return Ok(LolValue::Troof(false));
                            }
                        }
                        Ok(LolValue::Troof(true))
                    }
                    NaryOp::Any => {
                        for e in exprs {
                            if self.eval_expr(e, locals)?.to_troof() {
                                return Ok(LolValue::Troof(true));
                            }
                        }
                        Ok(LolValue::Troof(false))
                    }
                    NaryOp::Smoosh => {
                        let mut s = String::new();
                        for e in exprs {
                            s.push_str(&self.eval_expr(e, locals)?.to_yarn());
                        }
                        Ok(LolValue::Yarn(s))
                    }
                }
            }
            Expr::Cast { expr, target_type } => {
                let val = self.eval_expr(expr, locals)?;
                match target_type.as_str() {
                    "TROOF" => Ok(LolValue::Troof(val.to_troof())),
                    "NUMBR" => Ok(LolValue::Numbr(val.to_numbr())),
                    "NUMBAR" => Ok(LolValue::Numbar(val.to_numbar())),
                    "YARN" => Ok(LolValue::Yarn(val.to_yarn())),
                    _ => Ok(val),
                }
            }
            Expr::ArrayLit(exprs) => {
                let mut arr = Vec::with_capacity(exprs.len());
                for e in exprs {
                    arr.push(self.eval_expr(e, locals)?);
                }
                Ok(LolValue::Buket(arr))
            }
            Expr::ArrayGet { target, index } => {
                let t = self.eval_expr(target, locals)?;
                let idx = self.eval_expr(index, locals)?.to_numbr() as usize;
                if let LolValue::Buket(arr) = t {
                    if idx < arr.len() {
                        Ok(arr[idx].clone())
                    } else {
                        Ok(LolValue::Noob)
                    }
                } else {
                    Ok(LolValue::Noob)
                }
            }
            Expr::Call { name, args } => {
                let mut evaluated_args = Vec::new();
                for a in args {
                    evaluated_args.push(self.eval_expr(a, locals)?);
                }
                self.call_function(name, &evaluated_args)
            }
        }
    }

    pub fn call_function(&mut self, name: &str, args: &[LolValue]) -> Result<LolValue, String> {
        // First check native functions
        if let Some(res) = self.call_native(name, args)? {
            return Ok(res);
        }

        // Look up user defined function
        let (params, body) = match self.functions.get(name) {
            Some((p, b)) => (p.clone(), b.clone()),
            None => return Err(format!("Undefined function: {}", name)),
        };

        let mut func_locals = HashMap::new();
        for (i, p) in params.iter().enumerate() {
            let val = args.get(i).cloned().unwrap_or(LolValue::Noob);
            func_locals.insert(p.clone(), val);
        }

        for stmt in &body {
            if let Some(ret) = self.exec_statement(stmt, &mut func_locals)? {
                return Ok(ret);
            }
        }

        Ok(LolValue::Noob)
    }

    fn call_native(&mut self, name: &str, args: &[LolValue]) -> Result<Option<LolValue>, String> {
        let arg0 = args.first().map(|v| v.to_numbar()).unwrap_or(0.0);
        let arg1 = args.get(1).map(|v| v.to_numbar()).unwrap_or(0.0);
        let arg2 = args.get(2).map(|v| v.to_numbar()).unwrap_or(0.0);

        match name {
            // Trigonometry and Math
            "MATH_SIN" => Ok(Some(LolValue::Numbar(arg0.sin()))),
            "MATH_COS" => Ok(Some(LolValue::Numbar(arg0.cos()))),
            "MATH_TAN" => Ok(Some(LolValue::Numbar(arg0.tan()))),
            "MATH_ASIN" => Ok(Some(LolValue::Numbar(arg0.asin()))),
            "MATH_ACOS" => Ok(Some(LolValue::Numbar(arg0.acos()))),
            "MATH_ATAN2" => Ok(Some(LolValue::Numbar(arg0.atan2(arg1)))),
            "MATH_SQRT" => Ok(Some(LolValue::Numbar(arg0.max(0.0).sqrt()))),
            "MATH_ABS" => Ok(Some(LolValue::Numbar(arg0.abs()))),
            "MATH_HYPOT" => Ok(Some(LolValue::Numbar(arg0.hypot(arg1)))),
            "MATH_CLAMP" => {
                let min = arg1;
                let max = arg2;
                Ok(Some(LolValue::Numbar(arg0.max(min).min(max))))
            }
            "MATH_DEG2RAD" => Ok(Some(LolValue::Numbar(arg0.to_radians()))),
            "MATH_RAD2DEG" => Ok(Some(LolValue::Numbar(arg0.to_degrees()))),
            "MATH_NORM_ANGLE" => {
                let mut a = arg0;
                while a > std::f64::consts::PI {
                    a -= 2.0 * std::f64::consts::PI;
                }
                while a < -std::f64::consts::PI {
                    a += 2.0 * std::f64::consts::PI;
                }
                Ok(Some(LolValue::Numbar(a)))
            }
            "MATH_NORM_HEADING" => {
                let mut h = arg0 % 360.0;
                if h < 0.0 {
                    h += 360.0;
                }
                Ok(Some(LolValue::Numbar(h)))
            }

            // Great Circle & Navigation
            "NAV_HAVERSINE" => {
                let lat1 = arg0.to_radians();
                let lon1 = arg1.to_radians();
                let lat2 = args.get(2).map(|v| v.to_numbar()).unwrap_or(0.0).to_radians();
                let lon2 = args.get(3).map(|v| v.to_numbar()).unwrap_or(0.0).to_radians();
                let dlat = lat2 - lat1;
                let dlon = lon2 - lon1;
                let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
                let c = 2.0 * a.sqrt().atan2((1.0 - a).max(0.0).sqrt());
                // Nautical miles: 1 radian = 3440.065 nautical miles
                let nm = 3440.065 * c;
                Ok(Some(LolValue::Numbar(nm)))
            }
            "NAV_BEARING" => {
                let lat1 = arg0.to_radians();
                let lon1 = arg1.to_radians();
                let lat2 = args.get(2).map(|v| v.to_numbar()).unwrap_or(0.0).to_radians();
                let lon2 = args.get(3).map(|v| v.to_numbar()).unwrap_or(0.0).to_radians();
                let dlon = lon2 - lon1;
                let y = dlon.sin() * lat2.cos();
                let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
                let mut bearing = y.atan2(x).to_degrees();
                if bearing < 0.0 {
                    bearing += 360.0;
                }
                Ok(Some(LolValue::Numbar(bearing)))
            }
            "NAV_CROSS_TRACK_ERR" => {
                // Cross track error in nautical miles
                let d_cur = arg0; // distance from start
                let b_cur = arg1.to_radians(); // bearing to cur
                let b_path = arg2.to_radians(); // path bearing
                let xte = d_cur * (b_cur - b_path).sin();
                Ok(Some(LolValue::Numbar(xte)))
            }
            "NAV_LOS_HEADING" => {
                let xte = arg0;
                let delta = arg1.max(0.1); // lookahead distance
                let path_course = arg2;
                let crab = (-xte / delta).atan().to_degrees();
                let mut chi = (path_course + crab) % 360.0;
                if chi < 0.0 {
                    chi += 360.0;
                }
                Ok(Some(LolValue::Numbar(chi)))
            }

            // COLREGS Encounter Classification
            "COLREGS_CLASSIFY" => {
                // Returns 13 (overtaking), 14 (head-on), 15 (crossing give-way), 16 (stand-on), or 0 (safe)
                let rel_bearing = arg0; // relative bearing of target [-180, 180] deg
                let target_aspect = arg1; // aspect angle of target [-180, 180] deg
                let range_nm = arg2;

                if range_nm > 6.0 {
                    return Ok(Some(LolValue::Numbr(0))); // Outside encounter range
                }

                if rel_bearing.abs() > 112.5 {
                    // Target is behind us -> We are being overtaken or target is in stern sector
                    Ok(Some(LolValue::Numbr(13)))
                } else if rel_bearing.abs() <= 15.0 && target_aspect.abs() >= 165.0 {
                    // Head-on situation (Rule 14)
                    Ok(Some(LolValue::Numbr(14)))
                } else if rel_bearing > 5.0 && rel_bearing <= 112.5 {
                    // Target on starboard side -> We are Give-Way (Rule 15)
                    Ok(Some(LolValue::Numbr(15)))
                } else if rel_bearing < -5.0 && rel_bearing >= -112.5 {
                    // Target on port side -> We are Stand-On (Rule 17)
                    Ok(Some(LolValue::Numbr(16)))
                } else {
                    Ok(Some(LolValue::Numbr(0)))
                }
            }

            // CPA and TCPA
            "CPA_COMPUTE" => {
                let dx = arg0; // x_tgt - x_own
                let dy = arg1; // y_tgt - y_own
                let dvx = arg2; // vx_tgt - vx_own
                let dvy = args.get(3).map(|v| v.to_numbar()).unwrap_or(0.0); // vy_tgt - vy_own
                let dv_sq = dvx * dvx + dvy * dvy;
                if dv_sq < 1e-6 {
                    let dist = (dx * dx + dy * dy).sqrt();
                    return Ok(Some(LolValue::Numbar(dist)));
                }
                let tcpa = -(dx * dvx + dy * dvy) / dv_sq;
                let cpa_x = dx + dvx * tcpa.max(0.0);
                let cpa_y = dy + dvy * tcpa.max(0.0);
                let cpa = (cpa_x * cpa_x + cpa_y * cpa_y).sqrt();
                Ok(Some(LolValue::Numbar(cpa)))
            }
            "TCPA_COMPUTE" => {
                let dx = arg0;
                let dy = arg1;
                let dvx = arg2;
                let dvy = args.get(3).map(|v| v.to_numbar()).unwrap_or(0.0);
                let dv_sq = dvx * dvx + dvy * dvy;
                if dv_sq < 1e-6 {
                    return Ok(Some(LolValue::Numbar(9999.0)));
                }
                let tcpa = -(dx * dvx + dy * dvy) / dv_sq;
                Ok(Some(LolValue::Numbar(tcpa)))
            }

            // Hardware Telemetry Registers
            "READ_REG" => {
                let reg_id = arg0 as usize;
                Ok(Some(LolValue::Numbar(self.get_register(reg_id))))
            }
            "WRITE_REG" => {
                let reg_id = arg0 as usize;
                let val = arg1;
                self.set_register(reg_id, val);
                Ok(Some(LolValue::Troof(true)))
            }

            // NMEA 0183 XOR Checksum
            "NMEA_XOR_CHECKSUM" => {
                let s = args.first().map(|v| v.to_yarn()).unwrap_or_default();
                let mut sum: u8 = 0;
                let mut counting = false;
                for b in s.bytes() {
                    if b == b'$' || b == b'!' {
                        counting = true;
                        continue;
                    }
                    if b == b'*' {
                        break;
                    }
                    if counting {
                        sum ^= b;
                    }
                }
                Ok(Some(LolValue::Yarn(format!("{:02X}", sum))))
            }

            _ => Ok(None),
        }
    }

    fn get_var(&self, name: &str, locals: &HashMap<String, LolValue>) -> LolValue {
        if let Some(v) = locals.get(name) {
            v.clone()
        } else if let Some(v) = self.globals.get(name) {
            v.clone()
        } else {
            LolValue::Noob
        }
    }

    fn set_var(&mut self, name: &str, val: LolValue, locals: &mut HashMap<String, LolValue>) {
        if locals.contains_key(name) {
            locals.insert(name.to_string(), val);
        } else {
            self.globals.insert(name.to_string(), val);
        }
    }
}
