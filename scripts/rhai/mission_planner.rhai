// Gudgeon Autonomous Mission Planner in Rhai
// Coordinates waypoint sequencing, route optimization, and fuel estimates

fn create_route() {
    [
        #{ "name": "WP01_BERTH",      "lat": 37.7749, "lon": -122.4194, "speed_kts": 5.0,  "radius_m": 30.0 },
        #{ "name": "WP02_CHANNEL",    "lat": 37.7850, "lon": -122.4050, "speed_kts": 10.0, "radius_m": 50.0 },
        #{ "name": "WP03_PILOT",      "lat": 37.8000, "lon": -122.3800, "speed_kts": 12.0, "radius_m": 60.0 },
        #{ "name": "WP04_OPEN_WATER", "lat": 37.8200, "lon": -122.3500, "speed_kts": 16.0, "radius_m": 80.0 },
        #{ "name": "WP05_SURVEY_A",   "lat": 37.8500, "lon": -122.3100, "speed_kts": 14.0, "radius_m": 50.0 }
    ]
}

fn compute_total_route_distance(route) {
    let total_dist = 0.0;
    for i in 0..(route.len() - 1) {
        let p1 = route[i];
        let p2 = route[i + 1];
        let leg_dist = haversine_nm(p1.lat, p1.lon, p2.lat, p2.lon);
        total_dist += leg_dist;
    }
    total_dist
}

fn estimate_transit_time_hours(route) {
    let total_hours = 0.0;
    for i in 0..(route.len() - 1) {
        let p1 = route[i];
        let p2 = route[i + 1];
        let leg_dist = haversine_nm(p1.lat, p1.lon, p2.lat, p2.lon);
        let leg_spd = (p1.speed_kts + p2.speed_kts) / 2.0;
        total_hours += leg_dist / leg_spd;
    }
    total_hours
}

fn verify_mission_planner() {
    let route = create_route();
    if route.len() != 5 { return false; }
    let dist = compute_total_route_distance(route);
    if dist < 5.0 || dist > 20.0 { return false; }
    let time_h = estimate_transit_time_hours(route);
    if time_h < 0.2 || time_h > 3.0 { return false; }
    true
}

verify_mission_planner()
