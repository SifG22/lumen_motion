//! The Lumen menu's tools (`commands/lumen.rs`) and Lumen Motion's own project files
//! (`.lumeneffects`: format and version, older formats, damaged files, backups).

use lumen_motion_keyframe::Interp;
use lumen_motion_project::render_queue::{Channels, OutputFormat};
use lumen_motion_project::{LayerId, LayerSource, Project, Resolution, Value as KV};
use serde_json::{Value, json};

use crate::Session;

fn session_with_comp() -> Session {
    let mut s = Session::default();
    s.execute("comp.new", json!({"name": "Main", "width": 1920, "height": 1080, "frameRate": 30, "duration": 10})).unwrap();
    s
}

fn layers(s: &Session) -> Vec<LayerId> {
    s.active_comp().map(|c| c.layers.iter().map(|l| l.id).collect()).unwrap_or_default()
}

fn prop_keys(s: &Session, l: LayerId, path: &str) -> usize {
    s.active_comp().and_then(|c| c.layer(l)).and_then(|l| l.props.prop(path)).map(|p| p.keys.len()).unwrap_or(0)
}

fn tmp(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("lm-lumen-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn animate_text_from_an_empty_project_is_one_undo_step() {
    let mut s = Session::default();
    let r = s.execute("lumen.animateText", json!({"style": "typewriter", "text": "Hola"})).unwrap();
    assert_eq!(r["preset"], "typewriter");
    let comp = s.active_comp().expect("a composition was made");
    assert_eq!((comp.width, comp.height), (1920, 1080));
    let text = comp.layers.iter().find(|l| matches!(l.source, LayerSource::Text)).expect("a text layer");
    assert!(text.props.sub("text").and_then(|t| t.sub("animators")).is_some_and(|a| !a.children.is_empty()), "the preset added an animator");
    assert_eq!(s.state.selected_layers, vec![text.id]);
    s.execute("edit.undo", json!({})).unwrap();
    assert!(s.project.comps().next().is_none(), "one undo removes the text and the composition");
}

#[test]
fn animate_text_uses_the_selected_text_layers() {
    let mut s = session_with_comp();
    let a = LayerId(s.execute("layer.newText", json!({"text": "A"})).unwrap()["layer"].as_u64().unwrap());
    let solid = LayerId(s.execute("layer.newSolid", json!({"name": "Solid"})).unwrap()["layer"].as_u64().unwrap());
    s.execute("layer.select", json!({"layers": [a.0, solid.0]})).unwrap();
    let r = s.execute("lumen.animateText", json!({"style": "bounce"})).unwrap();
    assert_eq!(r["layers"], json!([a.0]), "only the text layer, and no new one");
    assert_eq!(layers(&s).len(), 2);
    assert!(s.execute("lumen.animateText", json!({"style": "nope"})).is_err());
}

#[test]
fn title_and_lower_third_animate_in_and_out() {
    let mut s = session_with_comp();
    s.execute("time.goTo", json!({"time": 1.0})).ok();
    let r = s.execute("lumen.addTitle", json!({"title": "Hola", "subtitle": "Mundo"})).unwrap();
    let ids: Vec<LayerId> = r["layers"].as_array().unwrap().iter().map(|v| LayerId(v.as_u64().unwrap())).collect();
    assert_eq!(ids.len(), 2);
    assert_eq!(prop_keys(&s, ids[0], "transform/opacity"), 4, "in and out");
    assert!(prop_keys(&s, ids[0], "transform/scale") >= 2);
    let r = s.execute("lumen.addLowerThird", json!({"name": "Ana", "role": "Directora"})).unwrap();
    let lt: Vec<LayerId> = r["layers"].as_array().unwrap().iter().map(|v| LayerId(v.as_u64().unwrap())).collect();
    assert_eq!(lt.len(), 3);
    assert_eq!(prop_keys(&s, lt[0], "transform/position"), 4, "the bar slides in and out");
    let before = layers(&s).len();
    s.execute("edit.undo", json!({})).unwrap();
    assert_eq!(layers(&s).len(), before - 3, "the lower third is one undo step");
}

#[test]
fn quick_transitions_key_the_in_and_out_points() {
    let mut s = session_with_comp();
    let l = LayerId(s.execute("layer.newSolid", json!({"name": "S"})).unwrap()["layer"].as_u64().unwrap());
    s.execute("layer.select", json!({"layers": [l.0]})).unwrap();
    s.execute("lumen.transition", json!({"style": "fade"})).unwrap();
    assert_eq!(prop_keys(&s, l, "transform/opacity"), 4);
    let comp = s.active_comp().unwrap();
    let opacity = comp.layer(l).unwrap().props.prop("transform/opacity").unwrap();
    assert_eq!(opacity.value_at(lumen_motion_time::Tick::ZERO), KV::Scalar(0.0), "transparent at the In point");
    assert_eq!(opacity.value_at(lumen_motion_time::Tick::from_seconds_f64(5.0)), KV::Scalar(100.0), "opaque in the middle");
    s.execute("lumen.transition", json!({"style": "slideLeft", "edge": "in"})).unwrap();
    assert_eq!(prop_keys(&s, l, "transform/position"), 2, "only the In point");
    s.execute("lumen.transition", json!({"style": "pop"})).unwrap();
    assert_eq!(prop_keys(&s, l, "transform/scale"), 6);
    assert!(s.execute("lumen.transition", json!({"style": "spin"})).is_err());
    assert!(s.execute("lumen.transition", json!({"edge": "middle"})).is_err());
    s.execute("layer.select", json!({"layers": []})).unwrap();
    assert!(s.execute("lumen.transition", json!({})).is_err(), "nothing selected: a clear error");
}

#[test]
fn easing_presets_change_the_keyframes_and_replace_each_other() {
    let mut s = session_with_comp();
    let l = LayerId(s.execute("layer.newSolid", json!({"name": "S"})).unwrap()["layer"].as_u64().unwrap());
    s.execute("prop.addKey", json!({"layer": l.0, "path": "transform/position", "value": [100.0, 540.0, 0.0], "time": 0.0})).unwrap();
    s.execute("prop.addKey", json!({"layer": l.0, "path": "transform/position", "value": [1800.0, 540.0, 0.0], "time": 1.0})).unwrap();
    s.execute("layer.select", json!({"layers": [l.0]})).unwrap();
    s.state.selected_keys.clear();
    let key = |s: &Session, i: usize| s.active_comp().unwrap().layer(l).unwrap().props.prop("transform/position").unwrap().keys[i].clone();
    let r = s.execute("lumen.ease", json!({"curve": "smooth"})).unwrap();
    assert_eq!(r["keyframes"], 2);
    assert_eq!((key(&s, 0).out_interp, key(&s, 1).in_interp), (Interp::Bezier, Interp::Bezier));
    assert!((key(&s, 1).in_ease[0].influence - 1.0 / 3.0).abs() < 1e-9);
    s.execute("lumen.ease", json!({"curve": "linear"})).unwrap();
    assert_eq!((key(&s, 0).out_interp, key(&s, 1).in_interp), (Interp::Linear, Interp::Linear));
    // Overshoot: the value swings past the last keyframe after it.
    s.execute("lumen.ease", json!({"curve": "overshoot"})).unwrap();
    let expr = || s.active_comp().unwrap().layer(l).unwrap().props.prop("transform/position").unwrap().expr.clone();
    assert!(expr().is_some_and(|e| e.text.starts_with(crate::commands::lumen::EXPRESSION_TAG) && e.enabled));
    // (What the expression does is checked where expressions run: crates/host/tests/lumen_ease.rs.)
    assert_eq!(key(&s, 1).in_interp, Interp::Linear, "it arrives with speed, so it can swing past");
    // Another preset replaces the tool's expression; someone's own expression is kept.
    s.execute("lumen.ease", json!({"curve": "smooth"})).unwrap();
    assert!(s.active_comp().unwrap().layer(l).unwrap().props.prop("transform/position").unwrap().expr.is_none());
    s.execute("prop.setExpression", json!({"layer": l.0, "path": "transform/position", "expression": "value"})).unwrap();
    let r = s.execute("lumen.ease", json!({"curve": "bounce"})).unwrap();
    assert_eq!(r["keptExpressions"], 1);
    assert!(s.execute("lumen.ease", json!({"curve": "wobbly"})).is_err());
}

#[test]
fn reframe_makes_a_vertical_copy_over_a_blurred_fill() {
    let mut s = session_with_comp();
    let src = s.active_comp_id().unwrap();
    s.execute("layer.newSolid", json!({"name": "S"})).unwrap();
    let r = s.execute("lumen.reframe", json!({"format": "9x16"})).unwrap();
    assert_eq!((r["width"].as_u64(), r["height"].as_u64()), (Some(1080), Some(1920)));
    assert_eq!(r["name"], "Main (Vertical 9:16)");
    let comp = s.active_comp().unwrap();
    assert_eq!(comp.layers.len(), 2, "the blurred fill and the whole picture");
    assert!(comp.layers.iter().all(|l| matches!(l.source, LayerSource::Comp { item } if item == src)));
    let front = comp.layers.iter().find(|l| l.props.sub("effects").is_none_or(|e| e.children.is_empty())).unwrap();
    let scale = front.props.prop("transform/scale").unwrap().value.clone();
    assert_eq!(scale, KV::Vec3([56.25, 56.25, 100.0]), "fitted: 1080 / 1920");
    s.execute("edit.undo", json!({})).unwrap();
    assert!(s.project.items.values().all(|i| i.name != "Main (Vertical 9:16)"), "one undo step");
    s.execute("comp.open", json!({"comp": src.0})).unwrap();
    let r = s.execute("lumen.reframe", json!({"format": "1x1", "mode": "fill"})).unwrap();
    assert_eq!(r["layers"].as_array().map(Vec::len), Some(1));
    assert!(s.execute("lumen.reframe", json!({"format": "3x2"})).is_err());
}

#[test]
fn export_for_social_queues_every_format_once() {
    let mut s = session_with_comp();
    let src = s.active_comp_id().unwrap();
    let r = s.execute("lumen.exportSocial", json!({"start": false})).unwrap();
    assert_eq!(r["comps"].as_array().map(Vec::len), Some(4), "{r}");
    assert_eq!(s.project.render_queue.len(), 4);
    assert!(s.project.render_queue.iter().all(|i| i.output.format == OutputFormat::H264));
    assert_eq!(s.active_comp_id(), Some(src), "back on the original");
    let comps = s.project.comps().count();
    s.execute("lumen.exportSocial", json!({"start": false, "formats": ["9x16"]})).unwrap();
    assert_eq!(s.project.comps().count(), comps, "the vertical copy is reused");
    assert!(s.execute("lumen.exportSocial", json!({"start": false, "formats": ["8x3"]})).is_err());
    s.execute("lumen.exportAlpha", json!({"start": false})).unwrap();
    let last = s.project.render_queue.last().unwrap();
    assert_eq!((last.output.format, last.output.channels), (OutputFormat::WebM, Channels::Rgba));
}

#[test]
fn fast_preview_toggles_half_resolution() {
    let mut s = session_with_comp();
    let cid = s.active_comp_id().unwrap();
    assert_eq!(s.execute("lumen.fastPreview", json!({})).unwrap()["on"], true);
    assert_eq!(s.project.comp(cid).unwrap().resolution, Resolution::Half);
    assert_eq!(s.execute("lumen.fastPreview", json!({})).unwrap()["on"], false);
    assert_eq!(s.project.comp(cid).unwrap().resolution, Resolution::Auto);
}

#[test]
fn every_lumen_tool_is_in_the_lumen_menu() {
    let ids: Vec<&str> = crate::commands::command_specs().iter().map(|c| c.id).filter(|id| id.starts_with("lumen.")).collect();
    assert_eq!(ids.len(), 9);
    let tree = crate::menus::TREE;
    for id in ids {
        assert!(tree.contains(&format!("| {id}")), "{id} has no Lumen menu entry");
    }
    assert!(crate::menus::top_level().contains(&"Lumen"));
}

// ------------------------------------------------------------------------- project files

fn project() -> Session {
    let mut s = session_with_comp();
    s.execute("layer.newText", json!({"text": "Round trip"})).unwrap();
    s.execute("lumen.addTitle", json!({})).unwrap();
    s
}

#[test]
fn save_then_open_gives_the_same_project() {
    let d = tmp("roundtrip");
    let mut s = project();
    let path = d.join("Intro.lumeneffects");
    s.execute("file.saveAs", json!({"path": path.to_string_lossy()})).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.contains("\"format\": \"lumeneffects\"") && text.contains("\"formatVersion\": 1"), "the file names its format and version");
    let saved = (*s.project).clone();
    let mut t = Session::default();
    t.execute("file.open", json!({"path": path.to_string_lossy()})).unwrap();
    assert_eq!(*t.project, saved);
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn a_second_save_keeps_the_previous_file_as_bak() {
    let d = tmp("bak");
    let mut s = project();
    let path = d.join("P.lumeneffects");
    s.execute("file.saveAs", json!({"path": path.to_string_lossy()})).unwrap();
    let first = std::fs::read(&path).unwrap();
    assert!(!d.join("P.lumeneffects.bak").exists(), "nothing to back up the first time");
    s.execute("layer.newSolid", json!({"name": "More"})).unwrap();
    s.execute("file.save", json!({})).unwrap();
    assert_eq!(std::fs::read(d.join("P.lumeneffects.bak")).unwrap(), first);
    assert_ne!(std::fs::read(&path).unwrap(), first);
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn older_projects_open_and_save_with_the_new_extension() {
    let d = tmp("legacy");
    let s = project();
    // A project of the older format: no `format` header, the old extension.
    let old = d.join("Old.ecproj");
    std::fs::write(&old, s.project.to_json()).unwrap();
    let original = std::fs::read(&old).unwrap();
    let mut t = Session::default();
    let r = t.execute("file.open", json!({"path": old.to_string_lossy()})).unwrap();
    assert_eq!(r["savedBy"], Value::Null, "an older program's version is not compared with ours");
    assert_eq!(*t.project, *s.project);
    t.execute("file.save", json!({})).unwrap();
    assert_eq!(std::fs::read(&old).unwrap(), original, "the original is never overwritten");
    assert!(d.join("Old.lumeneffects").is_file());
    assert_eq!(t.path.as_deref(), Some(d.join("Old.lumeneffects").to_string_lossy().as_ref()));
    // Its XML copy too.
    let xml = crate::xml_project::to_xml(&s.project.to_file_json().unwrap()).unwrap();
    std::fs::write(d.join("Old.ecprojx"), xml).unwrap();
    Session::default().execute("file.open", json!({"path": d.join("Old.ecprojx").to_string_lossy()})).unwrap();
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn damaged_files_give_a_clear_error_and_change_nothing() {
    let d = tmp("damaged");
    let s = project();
    let full = s.project.to_file_json().unwrap();
    let cases: Vec<(&str, Vec<u8>, &str)> = vec![
        ("truncated.lumeneffects", full.as_bytes()[..full.len() / 2].to_vec(), "incomplete"),
        ("garbage.lumeneffects", b"{\"format\": \"lumeneffects\", \"items\": [1, 2,, }".to_vec(), "damaged"),
        ("empty.lumeneffects", Vec::new(), "empty"),
        ("binary.lumeneffects", vec![0xff, 0xfe, 0x00, 0x13, 0x88], "text"),
        ("newer.lumeneffects", full.replacen("\"formatVersion\": 1", "\"formatVersion\": 99", 1).into_bytes(), "newer version"),
    ];
    for (name, bytes, says) in cases {
        let path = d.join(name);
        std::fs::write(&path, &bytes).unwrap();
        let mut t = session_with_comp();
        let before = (*t.project).clone();
        let e = t.execute("file.open", json!({"path": path.to_string_lossy()})).unwrap_err().to_string();
        assert!(e.contains(says), "{name}: {e}");
        assert_eq!(*t.project, before, "{name}: the open project is untouched");
        assert_eq!(std::fs::read(&path).unwrap(), bytes, "{name}: the file is untouched");
    }
    let _ = std::fs::remove_dir_all(d);
}

#[test]
fn autosaves_go_to_the_app_data_folder_every_few_minutes() {
    let prefs = crate::prefs::Prefs::default();
    assert_eq!(prefs.auto_save.location, "appData");
    assert!(prefs.auto_save.interval_minutes <= 5 && prefs.auto_save.enabled);
    let data = std::path::Path::new("/data");
    let dir = crate::autosave::folder(&prefs, Some("/work/Intro.lumeneffects"), Some(data));
    assert_eq!(dir, Some(data.join(crate::autosave::AUTOSAVE_FOLDER)));
    let mut next = prefs.clone();
    next.auto_save.location = "nextToProject".into();
    assert_eq!(
        crate::autosave::folder(&next, Some("/work/Intro.lumeneffects"), Some(data)),
        Some(std::path::Path::new("/work").join(crate::autosave::AUTOSAVE_FOLDER))
    );
}

#[test]
fn project_text_of_a_saved_file_reads_back() {
    let s = project();
    let text = s.project.to_file_json().unwrap();
    assert_eq!(Project::from_json(&text).unwrap(), *s.project);
    assert_eq!(lumen_motion_project::saved_by(&text).as_deref(), Some(lumen_motion_project::APP_VERSION));
}
