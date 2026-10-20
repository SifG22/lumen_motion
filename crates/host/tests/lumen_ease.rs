//! Lumen ▸ Easing's expressions (`lumen.ease`), evaluated by the real expression engine: the
//! elastic overshoot swings past the last keyframe and settles; the bounce rebounds from it and
//! never passes it.

use serde_json::json;

fn session_with_move() -> (lumen_motion_engine::Session, u64) {
    let mut s = lumen_motion_host::session();
    s.execute("comp.new", json!({"name": "Main", "width": 1920, "height": 1080, "frameRate": 30, "duration": 6})).unwrap();
    let l = s.execute("layer.newSolid", json!({"name": "Ball"})).unwrap()["layer"].as_u64().unwrap();
    s.execute("prop.addKey", json!({"layer": l, "path": "transform/position", "value": [100.0, 540.0, 0.0], "time": 0.0})).unwrap();
    s.execute("prop.addKey", json!({"layer": l, "path": "transform/position", "value": [1800.0, 540.0, 0.0], "time": 1.0})).unwrap();
    s.execute("layer.select", json!({"layers": [l]})).unwrap();
    (s, l)
}

fn x_at(s: &mut lumen_motion_engine::Session, l: u64, t: f64) -> f64 {
    let r = s.execute("prop.get", json!({"layer": l, "path": "transform/position", "time": t})).unwrap();
    assert!(r.get("expressionError").is_none(), "{r}");
    r["evaluated"][0].as_f64().unwrap_or_else(|| panic!("no evaluated value: {r}"))
}

#[test]
fn elastic_overshoot_swings_past_and_settles() {
    let (mut s, l) = session_with_move();
    s.execute("lumen.ease", json!({"curve": "overshoot"})).unwrap();
    let peak = (1..30).map(|f| x_at(&mut s, l, 1.0 + f as f64 / 30.0)).fold(f64::MIN, f64::max);
    assert!(peak > 1810.0, "it swings past 1800: {peak}");
    assert!((x_at(&mut s, l, 4.5) - 1800.0).abs() < 1.0, "and settles");
    let r = s.execute("prop.get", json!({"layer": l, "path": "transform/position", "time": 0.5})).unwrap();
    assert_eq!(r["evaluated"], r["value"], "before the last keyframe the move is the keyframes' own");
}

#[test]
fn bounce_rebounds_without_passing_the_keyframe() {
    let (mut s, l) = session_with_move();
    s.execute("lumen.ease", json!({"curve": "bounce"})).unwrap();
    let xs: Vec<f64> = (1..60).map(|f| x_at(&mut s, l, 1.0 + f as f64 / 30.0)).collect();
    assert!(xs.iter().all(|x| *x <= 1800.0 + 1e-6), "never past the keyframe");
    assert!(xs.iter().any(|x| *x < 1790.0), "but it rebounds");
    assert!((x_at(&mut s, l, 4.5) - 1800.0).abs() < 1.0, "and rests");
}
