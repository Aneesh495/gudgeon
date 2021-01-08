# Mathematical and Hydrodynamic Foundations

This document details the mathematical, physical, and hydrodynamic formulations implemented across the Gudgeon autonomous navigation engine.

## 1. 3-DOF Vessel Hydrodynamics and Maneuvering Equations

The horizontal maneuvering motion of a surface vessel is modeled in three degrees of freedom: surge (longitudinal translation along the body x-axis), sway (lateral translation along the body y-axis), and yaw (rotation about the vertical z-axis).

### 1.1 Non-Linear Norrbin Steering Model
The steering dynamics relate rudder/pod deflection angle \(\delta\) to vessel yaw rate \(r = \dot{\psi}\):

\[ T \dot{r} + r + \alpha r^3 = K \delta \]

Where:
- \(K\) is the Nomoto turning gain (rad/s per rad of rudder deflection)
- \(T\) is the steering time constant (seconds), characterizing vessel inertia
- \(\alpha\) is the Norrbin non-linear steering coefficient (\(s^3 / rad^2\)), modeling the non-linear dampening and flow separation at high turning rates
- \(\delta\) is the effective steering angle produced by the azimuthing stern pods

For small yaw rates (\(r \ll 1\)), the equation reduces to the classic 1st-order linear Nomoto equation:

\[ T \dot{r} + r = K \delta \]

### 1.2 Added Mass and Hydrodynamic Inertia Tensor
Water displaced by the moving hull contributes substantial hydrodynamic inertia. The total generalized mass matrix \(M\) is defined as:

\[ M = \begin{bmatrix} m - X_{\dot{u}} & 0 & 0 \\ 0 & m - Y_{\dot{v}} & 0 \\ 0 & 0 & I_{zz} - N_{\dot{r}} \end{bmatrix} \]

Where:
- \(m\) is the rigid-body vessel mass (85,000 kg for a nominal 24-meter survey hull)
- \(I_{zz}\) is the mass moment of inertia about the yaw axis (3,500,000 \(kg \cdot m^2\))
- \(X_{\dot{u}}\) is the added mass in surge (\(-4,200\) kg)
- \(Y_{\dot{v}}\) is the added mass in sway (\(-35,000\) kg)
- \(N_{\dot{r}}\) is the added yaw moment of inertia (\(-1,200,000\) \(kg \cdot m^2\))

### 1.3 Hydrodynamic Damping and Coriolis Terms
Non-linear hydrodynamic damping forces are formulated using quadratic cross-flow drag:

\[ X_{hydro} = m \cdot v \cdot r + X_{u|u|} u |u| \]
\[ Y_{hydro} = -m \cdot u \cdot r + Y_{v|v|} v |v| \]
\[ N_{hydro} = N_{r|r|} r |r| \]

Where:
- \(X_{u|u|}\) is the longitudinal hull drag coefficient (\(-180.0\) \(kg/m\))
- \(Y_{v|v|}\) is the lateral hull drag coefficient (\(-1200.0\) \(kg/m\))
- \(N_{r|r|}\) is the non-linear yaw damping coefficient (\(-180,000\) \(kg \cdot m^2\))

---

## 2. Geodesic Navigation and Line-of-Sight Guidance

### 2.1 Haversine Distance on WGS-84 Sphere
The Great Circle distance \(d\) between points \((\phi_1, \lambda_1)\) and \((\phi_2, \lambda_2)\) is computed via:

\[ a = \sin^2\left(\frac{\Delta \phi}{2}\right) + \cos(\phi_1)\cos(\phi_2)\sin^2\left(\frac{\Delta \lambda}{2}\right) \]
\[ c = 2 \cdot \arctan2\left(\sqrt{a}, \sqrt{1 - a}\right) \]
\[ d = R_{earth} \cdot c \]

Where \(R_{earth} = 3440.065\) nautical miles (\(6,371,000\) meters).

### 2.2 Initial Geodetic True Bearing
The true initial compass bearing \(\theta\) from point 1 to point 2 is given by:

\[ \theta = \arctan2\left(\sin(\Delta \lambda)\cos(\phi_2), \cos(\phi_1)\sin(\phi_2) - \sin(\phi_1)\cos(\phi_2)\cos(\Delta \lambda)\right) \]

### 2.3 Line-of-Sight (LOS) Path Tracking with Crab Angle
To follow a straight-line path segment between waypoints \(W_k\) and \(W_{k+1}\) with path course \(\chi_p\), the cross-track error \(e_{xt}\) is calculated as:

\[ e_{xt} = d_{cur} \cdot \sin(\theta_{cur} - \chi_p) \]

The desired vessel heading \(\psi_{des}\) incorporates an integral wind/current drift compensation term:

\[ \beta_{crab} = \arctan\left(\frac{-e_{xt} - K_i \int e_{xt} \, dt}{\Delta}\right) \]
\[ \psi_{des} = \chi_p + \beta_{crab} \]

Where \(\Delta\) is the lookahead distance (nominally 150 meters), and \(K_i\) is the integral gain.

---

## 3. IMO COLREGS Collision Avoidance Mathematics

### 3.1 Closest Point of Approach (CPA) and Time to CPA (TCPA)
Given own ship position \(\mathbf{r}_o\) and velocity \(\mathbf{v}_o\), and target position \(\mathbf{r}_t\) and velocity \(\mathbf{v}_t\), the relative position and velocity are:

\[ \Delta \mathbf{r} = \mathbf{r}_t - \mathbf{r}_o \]
\[ \Delta \mathbf{v} = \mathbf{v}_t - \mathbf{v}_o \]

The Time to Closest Point of Approach (\(TCPA\)) is found by minimizing the square of the distance:

\[ TCPA = -\frac{\Delta \mathbf{r} \cdot \Delta \mathbf{v}}{\|\Delta \mathbf{v}\|^2} \]

When \(TCPA > 0\), the minimum separation distance (\(CPA\)) is:

\[ CPA = \|\Delta \mathbf{r} + \Delta \mathbf{v} \cdot TCPA\| \]

### 3.2 Velocity Obstacle (VO) Geometric Formulation
The Velocity Obstacle \(VO_{t}\) is the set of all own ship velocities \(\mathbf{v}_o\) that will result in a collision with the target within time horizon \(\tau\):

\[ VO_t = \{ \mathbf{v}_o \mid \exists t \in [0, \tau], \, (\mathbf{v}_o - \mathbf{v}_t) \cdot t \in D(\mathbf{r}_t - \mathbf{r}_o, R_{safe}) \} \]

Where \(D(\mathbf{p}, R_{safe})\) is a circular disk of radius \(R_{safe}\) (nominally 0.8 nautical miles) centered at \(\mathbf{p}\).

---

## 4. Thrust Allocation and Actuator Distribution

The propulsion subsystem decomposes high-level generalized force demands \(\boldsymbol{\tau} = [F_x, F_y, M_z]^T\) across two azimuthing stern pods and a bow tunnel thruster.

### 4.1 Actuator Geometry
- Left Pod: longitudinal position \(x_p = -10.5\) m, lateral position \(y_{p1} = -1.6\) m
- Right Pod: longitudinal position \(x_p = -10.5\) m, lateral position \(y_{p2} = +1.6\) m
- Bow Tunnel Thruster: longitudinal position \(x_b = +9.5\) m, lateral position \(y_b = 0.0\) m

### 4.2 Force Decomposition
- Base surge thrust: \(T_{base} = \frac{1}{2} F_x\)
- Differential thrust for yaw moment: \(\Delta T = \frac{M_z}{2 \cdot y_{p2}}\)
- Left Pod Thrust: \(T_1 = T_{base} - \Delta T\)
- Right Pod Thrust: \(T_2 = T_{base} + \Delta T\)
- Azimuth Pod Angle: \(\delta_{pod} = \arcsin\left(\frac{0.7 \cdot F_y}{T_1 + T_2}\right)\)
- Bow Tunnel Thruster: \(T_{bow} = 0.3 \cdot F_y\)
