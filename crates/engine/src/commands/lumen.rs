//! The Lumen menu: one-click tools for what takes many steps elsewhere. Each is an ordinary
//! command built from the engine's own commands, so it is one undo step, works from the menu, the
//! command palette, scripts, the CLI, the control channel and MCP, and needs no new file format.
//!
//! - `lumen.animateText`: animate the selected text layers with a text preset, or create a text
//!   layer and animate it (Typewriter, Fade Up, Bounce In, Tracking In, Blur In, Drop In).
//! - `lumen.addTitle` / `lumen.addLowerThird`: an animated title, or a name and role on a bar,
//!   sized for the open composition and starting at the current time.
//! - `lumen.transition`: fade, slide, zoom or pop the selected layers in at their In point and
//!   out at their Out point.
//! - `lumen.ease`: easing presets with plain names on the selected keyframes (or every keyframe
//!   of the selected layers), including an elastic overshoot and a bounce.
//! - `lumen.reframe`: a vertical 9:16, square 1:1, portrait 4:5 or landscape 16:9 copy of the
//!   composition, the original nested and fitted over a blurred fill.
//! - `lumen.exportSocial` / `lumen.exportAlpha`: queue MP4 renders for social media in every
//!   format at once, or a WebM with transparency, and start rendering.
//! - `lumen.fastPreview`: half resolution and adaptive fast previews in one click, and back.

use lumen_motion_keyframe::{Ease, Interp, Keyframe};
use lumen_motion_project::{Expression, ItemId, LayerId, LayerSource, Resolution, Uid, Value as KV};
use lumen_motion_time::Tick;
use serde_json::{Value, json};

use super::app_more::grouped;
use super::viewer_cmds::FastPreviews;
use super::{CommandSpec, always, bad, comp_id, has_comp, layers_p, str_p};
use crate::{EngineError, Result, Session, cmd};

/// The first line of every expression a Lumen tool writes, so the tools replace their own and
/// never someone's hand-written expression.
pub const EXPRESSION_TAG: &str = "// Lumen Motion:";

/// Text animation styles: (style, built-in text preset).
pub const TEXT_STYLES: &[(&str, &str)] = &[
    ("typewriter", "typewriter"),
    ("fadeUp", "fadeUpCharacters"),
    ("bounce", "bounceInWords"),
    ("tracking", "trackingIn"),
    ("blur", "blurIn"),
    ("drop", "dropInLines"),
];

/// Social formats: (id, width, height, name suffix).
pub const FORMATS: &[(&str, u32, u32, &str)] =
    &[("9x16", 1080, 1920, "Vertical 9:16"), ("1x1", 1080, 1080, "Square 1:1"), ("4x5", 1080, 1350, "Portrait 4:5"), ("16x9", 1920, 1080, "Landscape 16:9")];

fn id_of(v: &Value, key: &str) -> Result<u64> {
    v.get(key).and_then(Value::as_u64).ok_or_else(|| EngineError::Other(format!("expected `{key}` in {v}")))
}

/// The interface is in Spanish: default texts (titles, names) are written in Spanish.
fn spanish(s: &Session) -> bool {
    s.prefs.general.language == "es"
}

/// The active composition, or a new 1920×1080 one when none is open (the tools work from an
/// empty project too).
fn ensure_comp(s: &mut Session) -> Result<ItemId> {
    if let Some(c) = s.active_comp_id() {
        return Ok(c);
    }
    let name = if spanish(s) { "Animación" } else { "Animation" };
    s.execute("comp.new", json!({"name": name, "width": 1920, "height": 1080, "frameRate": 30, "duration": 10, "background": "#101418"}))?;
    s.active_comp_id().ok_or(EngineError::NoComp)
}

/// (width, height, duration in seconds, frame duration in seconds) of a composition.
fn dims(s: &Session, cid: ItemId) -> Result<(f64, f64, f64, f64)> {
    let c = s.project.comp(cid).ok_or(EngineError::NoComp)?;
    let fps = c.frame_rate.as_f64();
    Ok((c.width as f64, c.height as f64, c.duration.seconds(), if fps > 0.0 { 1.0 / fps } else { 1.0 / 30.0 }))
}

/// Comp seconds as a frame-aligned tick.
fn at(s: &Session, cid: ItemId, secs: f64) -> Tick {
    let rate = s.project.comp(cid).map(|c| c.frame_rate).unwrap_or(lumen_motion_time::FrameRate::FPS_30);
    rate.snap_nearest(Tick::from_seconds_f64(secs.max(0.0)))
}

/// Eased keyframes on `path` of a layer at comp times (seconds), merged with its existing keys.
fn put_keys(s: &mut Session, cid: ItemId, lid: LayerId, path: &str, keys: &[(f64, KV)]) -> Result<()> {
    let ticks: Vec<Tick> = keys.iter().map(|(t, _)| at(s, cid, *t)).collect();
    s.edit("Add Keyframes", None, |proj, _| {
        let layer = proj.comp_mut(cid).and_then(|c| c.layer_mut(lid)).ok_or_else(|| EngineError::Other("the layer is gone".into()))?;
        let times: Vec<Tick> = ticks.iter().map(|t| layer.layer_time(*t)).collect();
        let prop = layer.props.prop_mut(path).ok_or_else(|| EngineError::Other(format!("the layer has no {path}")))?;
        for (t, (_, v)) in times.into_iter().zip(keys) {
            lumen_motion_keyframe::set_key(&mut prop.keys, Keyframe::new(t, v.clone()).eased());
        }
        Ok(())
    })
}

/// A property's value at a comp time (its keys and static value; expressions aside).
fn value_at(s: &Session, cid: ItemId, lid: LayerId, path: &str, secs: f64) -> Option<KV> {
    let t = at(s, cid, secs);
    let layer = s.project.comp(cid)?.layer(lid)?;
    Some(layer.props.prop(path)?.value_at(layer.layer_time(t)))
}

fn new_text(s: &mut Session, cid: ItemId, text: &str, pos: [f64; 2], size: f64, fill: &str, justify: &str) -> Result<LayerId> {
    let r = s.execute(
        "layer.newText",
        json!({"comp": cid.0, "text": text, "position": pos, "size": size.round().clamp(8.0, 1000.0), "fill": fill, "justify": justify}),
    )?;
    Ok(LayerId(id_of(&r, "layer")?))
}

fn select(s: &mut Session, cid: ItemId, layers: &[LayerId]) -> Result<()> {
    s.execute("layer.select", json!({"comp": cid.0, "layers": layers.iter().map(|l| l.0).collect::<Vec<_>>()}))?;
    Ok(())
}

fn scalar(v: f64) -> KV {
    KV::Scalar(v)
}

/// `v` (a position or scale) moved by `d` / multiplied by `k` in x and y.
fn offset(v: &KV, d: [f64; 2]) -> KV {
    match v {
        KV::Vec2(a) => KV::Vec2([a[0] + d[0], a[1] + d[1]]),
        KV::Vec3(a) => KV::Vec3([a[0] + d[0], a[1] + d[1], a[2]]),
        other => other.clone(),
    }
}
fn scaled(v: &KV, k: f64) -> KV {
    match v {
        KV::Vec2(a) => KV::Vec2([a[0] * k, a[1] * k]),
        KV::Vec3(a) => KV::Vec3([a[0] * k, a[1] * k, a[2]]),
        KV::Scalar(a) => KV::Scalar(a * k),
        other => other.clone(),
    }
}

// ------------------------------------------------------------------------- text and titles

fn animate_text(s: &mut Session, p: &Value) -> Result<Value> {
    let style = str_p(p, "style").unwrap_or("fadeUp");
    let preset = TEXT_STYLES
        .iter()
        .find(|(k, _)| *k == style)
        .map(|(_, v)| *v)
        .ok_or_else(|| bad("lumen.animateText", format!("unknown style `{style}` (typewriter|fadeUp|bounce|tracking|blur|drop)")))?;
    grouped(s, "Animate Text", |s| {
        let cid = ensure_comp(s)?;
        let selected = if s.active_comp_id() == Some(cid) { layers_p(s, p).map(|(_, l)| l).unwrap_or_default() } else { vec![] };
        let comp = s.project.comp(cid).ok_or(EngineError::NoComp)?;
        let mut texts: Vec<LayerId> =
            selected.into_iter().filter(|l| comp.layer(*l).is_some_and(|l| matches!(l.source, LayerSource::Text) && !l.switches.locked)).collect();
        if texts.is_empty() {
            let (w, h, _, _) = dims(s, cid)?;
            let size = (h * 0.09).max(12.0);
            let default = if spanish(s) { "Tu texto aquí" } else { "Your text here" };
            let text = str_p(p, "text").filter(|t| !t.trim().is_empty()).unwrap_or(default).to_string();
            texts.push(new_text(s, cid, &text, [w / 2.0, h / 2.0 + size * 0.35], size, "#FFFFFF", "center")?);
        }
        for l in &texts {
            s.execute("layer.applyTextPreset", json!({"comp": cid.0, "layer": l.0, "preset": preset}))?;
        }
        select(s, cid, &texts)?;
        Ok(json!({"comp": cid.0, "layers": texts.iter().map(|l| l.0).collect::<Vec<_>>(), "preset": preset}))
    })
}

fn add_title(s: &mut Session, p: &Value) -> Result<Value> {
    grouped(s, "Add Animated Title", |s| {
        let cid = ensure_comp(s)?;
        let (w, h, dur, frame) = dims(s, cid)?;
        let t0 = s.time().seconds().clamp(0.0, (dur - frame).max(0.0));
        let end = (t0 + 5.0).min(dur - frame).max(t0 + 1.0);
        let es = spanish(s);
        let title = str_p(p, "title").unwrap_or(if es { "TU TÍTULO AQUÍ" } else { "YOUR TITLE HERE" }).to_string();
        let subtitle = str_p(p, "subtitle").unwrap_or(if es { "Un subtítulo que presenta la escena" } else { "A subtitle that sets the scene" }).to_string();
        let size = (h * 0.11).max(16.0);
        let y = h * 0.47 + size * 0.35;
        let main = new_text(s, cid, &title, [w / 2.0, y], size, "#FFFFFF", "center")?;
        let sub = new_text(s, cid, &subtitle, [w / 2.0, y + h * 0.075], (h * 0.037).max(10.0), "#D8F4F1", "center")?;
        let base = value_at(s, cid, main, "transform/scale", t0).unwrap_or(KV::Vec3([100.0, 100.0, 100.0]));
        put_keys(s, cid, main, "transform/scale", &[(t0, scaled(&base, 0.85)), (t0 + 0.7, base.clone()), (end, scaled(&base, 1.04))])?;
        put_keys(s, cid, main, "transform/opacity", &[(t0, scalar(0.0)), (t0 + 0.5, scalar(100.0)), (end - 0.5, scalar(100.0)), (end, scalar(0.0))])?;
        put_keys(
            s,
            cid,
            sub,
            "transform/opacity",
            &[(t0 + 0.4, scalar(0.0)), (t0 + 1.0, scalar(100.0)), (end - 0.6, scalar(100.0)), (end - 0.1, scalar(0.0))],
        )?;
        let pos = value_at(s, cid, sub, "transform/position", t0).unwrap_or(KV::Vec3([w / 2.0, y + h * 0.075, 0.0]));
        put_keys(s, cid, sub, "transform/position", &[(t0 + 0.4, offset(&pos, [0.0, h * 0.02])), (t0 + 1.0, pos)])?;
        s.execute("layer.rename", json!({"comp": cid.0, "layer": main.0, "name": if es { "Título" } else { "Title" }}))?;
        s.execute("layer.rename", json!({"comp": cid.0, "layer": sub.0, "name": if es { "Subtítulo" } else { "Subtitle" }}))?;
        select(s, cid, &[main, sub])?;
        Ok(json!({"comp": cid.0, "layers": [main.0, sub.0]}))
    })
}

fn add_lower_third(s: &mut Session, p: &Value) -> Result<Value> {
    grouped(s, "Add Lower Third", |s| {
        let cid = ensure_comp(s)?;
        let (w, h, dur, frame) = dims(s, cid)?;
        let t0 = s.time().seconds().clamp(0.0, (dur - frame).max(0.0));
        let end = (t0 + 5.0).min(dur - frame).max(t0 + 1.2);
        let es = spanish(s);
        let name = str_p(p, "name").unwrap_or(if es { "Nombre Apellido" } else { "Alex Rivera" }).to_string();
        let role = str_p(p, "role").unwrap_or(if es { "Cargo o descripción" } else { "Motion Designer" }).to_string();
        let color = str_p(p, "color").unwrap_or("#14A6B3").to_string();
        // A bar in the lower left third, a margin of 6% of the width from the edge.
        let (bw, bh) = ((w * 0.34).max(120.0), (h * 0.12).max(40.0));
        let (margin, cy) = (w * 0.06, h * 0.82);
        let r = s.execute(
            "layer.newShape",
            json!({"comp": cid.0, "name": if es { "Barra" } else { "Bar" }, "kind": "rect", "size": [bw, bh], "fill": color, "position": [0.0, 0.0]}),
        )?;
        let bar = LayerId(id_of(&r, "layer")?);
        let (out_x, in_x) = (-bw, margin + bw / 2.0);
        put_keys(
            s,
            cid,
            bar,
            "transform/position",
            &[
                (t0, KV::Vec3([out_x, cy, 0.0])),
                (t0 + 0.6, KV::Vec3([in_x, cy, 0.0])),
                (end - 0.6, KV::Vec3([in_x, cy, 0.0])),
                (end, KV::Vec3([out_x, cy, 0.0])),
            ],
        )?;
        let x = margin + w * 0.02;
        let title = new_text(s, cid, &name, [x, cy - bh * 0.06], (h * 0.05).max(10.0), "#FFFFFF", "left")?;
        let sub = new_text(s, cid, &role, [x, cy + bh * 0.33], (h * 0.028).max(8.0), "#E6FFFB", "left")?;
        for (l, a, b) in [(title, 0.35, 0.8), (sub, 0.5, 1.0)] {
            put_keys(s, cid, l, "transform/opacity", &[(t0 + a, scalar(0.0)), (t0 + b, scalar(100.0)), (end - 0.8, scalar(100.0)), (end - 0.4, scalar(0.0))])?;
        }
        select(s, cid, &[bar, title, sub])?;
        Ok(json!({"comp": cid.0, "layers": [bar.0, title.0, sub.0]}))
    })
}

// ------------------------------------------------------------------------- transitions

fn transition(s: &mut Session, p: &Value) -> Result<Value> {
    let style = str_p(p, "style").unwrap_or("fade").to_string();
    if !["fade", "slideLeft", "slideUp", "zoom", "pop"].contains(&style.as_str()) {
        return Err(bad("lumen.transition", format!("unknown style `{style}` (fade|slideLeft|slideUp|zoom|pop)")));
    }
    let edge = str_p(p, "edge").unwrap_or("both");
    let (do_in, do_out) = match edge {
        "in" => (true, false),
        "out" => (false, true),
        "both" => (true, true),
        other => return Err(bad("lumen.transition", format!("`edge` is in, out or both, not `{other}`"))),
    };
    let (cid, ids) = layers_p(s, p)?;
    let ids = super::unlocked(s, cid, ids, "lumen.transition")?;
    let comp = s.project.comp(cid).ok_or(EngineError::NoComp)?;
    let targets: Vec<(LayerId, f64, f64)> = ids
        .iter()
        .filter_map(|id| comp.layer(*id))
        .filter(|l| !l.is_camera() && !l.is_light())
        .map(|l| (l.id, l.in_point.seconds(), l.out_point.seconds()))
        .collect();
    if targets.is_empty() {
        return Err(bad("lumen.transition", "select the layers to animate first"));
    }
    let (w, h, _, frame) = dims(s, cid)?;
    grouped(s, "Quick Transition", |s| {
        for (lid, t_in, t_out) in &targets {
            let last = (t_out - frame).max(*t_in);
            let d = ((last - t_in) / 3.0).clamp(frame, 0.5);
            let opacity_in = value_at(s, cid, *lid, "transform/opacity", t_in + d).unwrap_or(scalar(100.0));
            let opacity_out = value_at(s, cid, *lid, "transform/opacity", last - d).unwrap_or(scalar(100.0));
            let fade = |s: &mut Session| -> Result<()> {
                if do_in {
                    put_keys(s, cid, *lid, "transform/opacity", &[(*t_in, scalar(0.0)), (t_in + d, opacity_in.clone())])?;
                }
                if do_out {
                    put_keys(s, cid, *lid, "transform/opacity", &[(last - d, opacity_out.clone()), (last, scalar(0.0))])?;
                }
                Ok(())
            };
            match style.as_str() {
                "fade" => fade(s)?,
                "slideLeft" | "slideUp" => {
                    let shift = if style == "slideLeft" { [-w * 0.25, 0.0] } else { [0.0, h * 0.2] };
                    let away = if style == "slideLeft" { [w * 0.25, 0.0] } else { [0.0, -h * 0.2] };
                    if do_in && let Some(pos) = value_at(s, cid, *lid, "transform/position", t_in + d) {
                        put_keys(s, cid, *lid, "transform/position", &[(*t_in, offset(&pos, shift)), (t_in + d, pos)])?;
                    }
                    if do_out && let Some(pos) = value_at(s, cid, *lid, "transform/position", last - d) {
                        put_keys(s, cid, *lid, "transform/position", &[(last - d, pos.clone()), (last, offset(&pos, away))])?;
                    }
                    fade(s)?;
                }
                "zoom" => {
                    if do_in && let Some(sc) = value_at(s, cid, *lid, "transform/scale", t_in + d) {
                        put_keys(s, cid, *lid, "transform/scale", &[(*t_in, scaled(&sc, 0.6)), (t_in + d, sc)])?;
                    }
                    if do_out && let Some(sc) = value_at(s, cid, *lid, "transform/scale", last - d) {
                        put_keys(s, cid, *lid, "transform/scale", &[(last - d, sc.clone()), (last, scaled(&sc, 1.3))])?;
                    }
                    fade(s)?;
                }
                _ => {
                    // pop: from nothing, past full size and back; out to nothing.
                    if do_in && let Some(sc) = value_at(s, cid, *lid, "transform/scale", t_in + d) {
                        put_keys(s, cid, *lid, "transform/scale", &[(*t_in, scaled(&sc, 0.0)), (t_in + d * 0.7, scaled(&sc, 1.12)), (t_in + d, sc)])?;
                    }
                    if do_out && let Some(sc) = value_at(s, cid, *lid, "transform/scale", last - d) {
                        put_keys(s, cid, *lid, "transform/scale", &[(last - d, sc.clone()), (last - d * 0.7, scaled(&sc, 1.12)), (last, scaled(&sc, 0.0))])?;
                    }
                }
            }
        }
        Ok(json!({"comp": cid.0, "layers": targets.iter().map(|t| t.0.0).collect::<Vec<_>>(), "style": style}))
    })
}

// ------------------------------------------------------------------------- easing presets

/// Easing presets: (id, in side, out side, influence of the eased sides, expression).
const CURVES: &[&str] = &["smooth", "slowDown", "speedUp", "snappy", "linear", "overshoot", "bounce"];

/// The elastic overshoot: after each keyframe the value swings past it and settles.
fn overshoot_expression() -> String {
    format!(
        "{EXPRESSION_TAG} elastic overshoot after each keyframe
var amp = 0.06, freq = 2.5, decay = 5.0;
var n = 0;
if (numKeys > 0) {{ n = nearestKey(time).index; if (key(n).time > time) {{ n--; }} }}
var t = n > 0 ? time - key(n).time : 0;
if (n > 0 && t > 0 && t < 3) {{
  var v = velocityAtTime(key(n).time - thisComp.frameDuration / 10);
  value + v * amp * Math.sin(freq * t * 2 * Math.PI) / Math.exp(decay * t);
}} else {{
  value;
}}"
    )
}

/// The bounce: after each keyframe the value rebounds from it a few times, lower each time.
fn bounce_expression() -> String {
    format!(
        "{EXPRESSION_TAG} bounce after each keyframe
var amp = 0.05, freq = 3.0, decay = 4.0;
var n = 0;
if (numKeys > 0) {{ n = nearestKey(time).index; if (key(n).time > time) {{ n--; }} }}
var t = n > 0 ? time - key(n).time : 0;
if (n > 0 && t > 0 && t < 3) {{
  var v = velocityAtTime(key(n).time - thisComp.frameDuration / 10);
  value - v * amp * Math.abs(Math.sin(freq * t * Math.PI)) / Math.exp(decay * t);
}} else {{
  value;
}}"
    )
}

fn ease(s: &mut Session, p: &Value) -> Result<Value> {
    let curve = str_p(p, "curve").unwrap_or("smooth").to_string();
    if !CURVES.contains(&curve.as_str()) {
        return Err(bad("lumen.ease", format!("unknown curve `{curve}` ({})", CURVES.join("|"))));
    }
    let cid = comp_id(s, p)?;
    // The selected keyframes, else every keyframe of the selected layers.
    let mut targets: Vec<(LayerId, Uid, Option<Tick>)> = s.state.selected_keys.iter().map(|k| (k.layer, k.prop, Some(k.time))).collect();
    if targets.is_empty() {
        let (_, layers) = layers_p(s, p)?;
        let comp = s.project.comp(cid).ok_or(EngineError::NoComp)?;
        for lid in layers {
            if let Some(l) = comp.layer(lid) {
                l.props.walk("", &mut |_, pr| {
                    if !pr.keys.is_empty() {
                        targets.push((lid, pr.uid, None));
                    }
                });
            }
        }
    }
    if targets.is_empty() {
        return Err(bad("lumen.ease", "select keyframes, or layers with keyframes, first"));
    }
    let (in_i, out_i, infl, expression) = match curve.as_str() {
        "smooth" => (Interp::Bezier, Interp::Bezier, 1.0 / 3.0, None),
        "slowDown" => (Interp::Bezier, Interp::Linear, 0.6, None),
        "speedUp" => (Interp::Linear, Interp::Bezier, 0.6, None),
        "snappy" => (Interp::Bezier, Interp::Bezier, 0.85, None),
        "linear" => (Interp::Linear, Interp::Linear, 0.0, None),
        // The value has to arrive with speed for it to swing past the keyframe.
        "overshoot" => (Interp::Linear, Interp::Bezier, 1.0 / 3.0, Some(overshoot_expression())),
        _ => (Interp::Linear, Interp::Bezier, 1.0 / 3.0, Some(bounce_expression())),
    };
    let mut skipped = 0usize;
    let mut changed = 0usize;
    s.edit("Easing Preset", None, |proj, _| {
        let comp = proj.comp_mut(cid).ok_or(EngineError::NoComp)?;
        let mut props: Vec<(LayerId, Uid)> = vec![];
        for (lid, uid, time) in &targets {
            let Some(pr) = comp.layer_mut(*lid).and_then(|l| l.props.find_mut(*uid)) else { continue };
            for k in pr.keys.iter_mut().filter(|k| time.is_none_or(|t| k.time == t)) {
                let n = k.value.dims().max(1);
                let side = |i: Interp| if i == Interp::Bezier { vec![Ease { speed: 0.0, influence: infl }; n] } else { vec![] };
                (k.in_interp, k.out_interp, k.in_ease, k.out_ease) = (in_i, out_i, side(in_i), side(out_i));
                (k.auto_bezier, k.continuous) = (false, false);
                changed += 1;
            }
            if !props.contains(&(*lid, *uid)) {
                props.push((*lid, *uid));
            }
        }
        for (lid, uid) in props {
            let Some(pr) = comp.layer_mut(lid).and_then(|l| l.props.find_mut(uid)) else { continue };
            let ours = pr.expr.as_ref().is_none_or(|e| e.text.starts_with(EXPRESSION_TAG) || e.text.trim().is_empty() || !e.enabled);
            match (&expression, ours) {
                (Some(text), true) => pr.expr = Some(Expression { text: text.clone(), enabled: true }),
                (None, true) if pr.expr.as_ref().is_some_and(|e| e.text.starts_with(EXPRESSION_TAG)) => pr.expr = None,
                (Some(_), false) => skipped += 1,
                _ => {}
            }
        }
        Ok(())
    })?;
    if skipped > 0 {
        s.toast(format!("{skipped} propert{} kept {} own expression.", if skipped == 1 { "y" } else { "ies" }, if skipped == 1 { "its" } else { "their" }));
    }
    Ok(json!({"curve": curve, "keyframes": changed, "keptExpressions": skipped}))
}

// ------------------------------------------------------------------------- formats and export

fn format_of(id: &str) -> Option<(&'static str, u32, u32, &'static str)> {
    FORMATS.iter().copied().find(|f| f.0 == id)
}

/// The name of `src`'s copy in a format.
fn copy_name(src: &str, suffix: &str) -> String {
    format!("{src} ({suffix})")
}

fn reframe(s: &mut Session, p: &Value) -> Result<Value> {
    let fmt = str_p(p, "format").unwrap_or("9x16");
    let (_, w, h, suffix) = format_of(fmt).ok_or_else(|| bad("lumen.reframe", format!("unknown format `{fmt}` (9x16|1x1|4x5|16x9)")))?;
    let mode = str_p(p, "mode").unwrap_or("blurFill");
    if !["blurFill", "fill", "fit"].contains(&mode) {
        return Err(bad("lumen.reframe", format!("unknown mode `{mode}` (blurFill|fill|fit)")));
    }
    let src = comp_id(s, p)?;
    let c = s.project.comp(src).ok_or(EngineError::NoComp)?;
    let (sw, sh) = (f64::from(c.width.max(1)), f64::from(c.height.max(1)));
    let (fps, dur, bg) = (c.frame_rate.as_f64(), c.duration.seconds(), c.background);
    let name = s.project.item(src).map(|i| i.name.clone()).unwrap_or_default();
    let bg =
        format!("#{:02X}{:02X}{:02X}", (bg[0].clamp(0.0, 1.0) * 255.0) as u8, (bg[1].clamp(0.0, 1.0) * 255.0) as u8, (bg[2].clamp(0.0, 1.0) * 255.0) as u8);
    let (fw, fh) = (f64::from(w), f64::from(h));
    let fill = (fw / sw).max(fh / sh) * 100.0;
    let fit = (fw / sw).min(fh / sh) * 100.0;
    let out_name = copy_name(&name, suffix);
    grouped(s, "Reframe Composition", |s| {
        let r = s.execute("comp.new", json!({"name": out_name, "width": w, "height": h, "frameRate": fps, "duration": dur, "background": bg, "open": true}))?;
        let out = ItemId(id_of(&r, "comp")?);
        let add = |s: &mut Session, scale: f64| -> Result<LayerId> {
            let r = s.execute("layer.addItem", json!({"comp": out.0, "item": src.0, "time": 0.0}))?;
            let l = LayerId(id_of(&r, "layer")?);
            s.execute("prop.set", json!({"comp": out.0, "layer": l.0, "path": "transform/scale", "value": [scale, scale, 100.0]}))?;
            s.execute("prop.set", json!({"comp": out.0, "layer": l.0, "path": "transform/position", "value": [fw / 2.0, fh / 2.0, 0.0]}))?;
            Ok(l)
        };
        let mut layers = vec![];
        if mode == "blurFill" && (fill - fit).abs() > 0.5 {
            // The same picture, filling the frame and blurred, behind the whole picture.
            let back = add(s, fill * 1.06)?;
            s.execute("effect.apply", json!({"comp": out.0, "layers": [back.0], "effect": "ec.blur.gaussian"}))?;
            s.execute("prop.set", json!({"comp": out.0, "layer": back.0, "path": "effects/#1/blurriness", "value": (fw.min(fh) * 0.06).round()}))?;
            s.execute("prop.set", json!({"comp": out.0, "layer": back.0, "path": "effects/#1/repeatEdge", "value": true}))?;
            s.execute("prop.set", json!({"comp": out.0, "layer": back.0, "path": "transform/opacity", "value": 70.0}))?;
            s.execute("layer.rename", json!({"comp": out.0, "layer": back.0, "name": if spanish(s) { "Fondo desenfocado" } else { "Blurred Background" }}))?;
            layers.push(back);
        }
        let front = add(s, if mode == "fill" { fill } else { fit })?;
        layers.push(front);
        select(s, out, &[front])?;
        Ok(json!({"comp": out.0, "name": out_name, "width": w, "height": h, "layers": layers.iter().map(|l| l.0).collect::<Vec<_>>()}))
    })
}

/// Formats `lumen.exportSocial` renders by default: the composition itself and every other shape.
const SOCIAL_DEFAULT: &[&str] = &["original", "9x16", "1x1", "4x5"];

fn export_social(s: &mut Session, p: &Value) -> Result<Value> {
    let src = comp_id(s, p)?;
    let c = s.project.comp(src).ok_or(EngineError::NoComp)?;
    let src_aspect = f64::from(c.width.max(1)) / f64::from(c.height.max(1));
    let name = s.project.item(src).map(|i| i.name.clone()).unwrap_or_default();
    let wanted: Vec<String> = match p.get("formats").and_then(Value::as_array) {
        Some(a) => a.iter().filter_map(Value::as_str).map(str::to_string).collect(),
        None => SOCIAL_DEFAULT.iter().map(|f| f.to_string()).collect(),
    };
    if wanted.is_empty() {
        return Err(bad("lumen.exportSocial", "`formats` is empty (original|9x16|1x1|4x5|16x9)"));
    }
    let start = p.get("start").and_then(Value::as_bool).unwrap_or(true);
    let r = grouped(s, "Export for Social Media", |s| {
        let mut comps: Vec<(String, ItemId)> = vec![];
        for f in &wanted {
            if f == "original" {
                comps.push(("original".into(), src));
                continue;
            }
            let (id, w, h, suffix) = format_of(f).ok_or_else(|| bad("lumen.exportSocial", format!("unknown format `{f}` (original|9x16|1x1|4x5|16x9)")))?;
            // The composition already has this shape: it is the original.
            if (f64::from(w) / f64::from(h) - src_aspect).abs() < 0.01 {
                continue;
            }
            // A copy made before (by name) is reused, with the changes made to it since.
            let existing = s.project.items.values().find(|i| i.name == copy_name(&name, suffix) && s.project.comp(i.id).is_some()).map(|i| i.id);
            let cid = match existing {
                Some(c) => c,
                None => ItemId(id_of(&reframe(s, &json!({"comp": src.0, "format": id}))?, "comp")?),
            };
            comps.push((id.into(), cid));
        }
        let mut queued = vec![];
        for (_, cid) in &comps {
            let r = s.execute("renderQueue.add", json!({"comp": cid.0, "format": "h264", "output": "[compName].[fileExtension]"}))?;
            queued.push(r);
        }
        Ok(json!({"comps": comps.iter().map(|(f, c)| json!({"format": f, "comp": c.0})).collect::<Vec<_>>(), "queued": queued}))
    })?;
    // Back to the composition the user was working on.
    s.execute("comp.open", json!({"comp": src.0}))?;
    if start {
        s.execute("renderQueue.render", json!({"wait": false}))?;
    }
    Ok(r)
}

fn export_alpha(s: &mut Session, p: &Value) -> Result<Value> {
    let src = comp_id(s, p)?;
    let format = str_p(p, "format").unwrap_or("webm");
    if !["webm", "prores"].contains(&format) {
        return Err(bad("lumen.exportAlpha", format!("unknown format `{format}` (webm|prores)")));
    }
    let mut params = json!({"comp": src.0, "format": format, "channels": "rgba", "output": "[compName] (alpha).[fileExtension]"});
    if format == "prores" {
        params["proresProfile"] = json!("4444");
    }
    let r = s.execute("renderQueue.add", params)?;
    if p.get("start").and_then(Value::as_bool).unwrap_or(true) {
        s.execute("renderQueue.render", json!({"wait": false}))?;
    }
    Ok(r)
}

fn fast_preview(s: &mut Session, p: &Value) -> Result<Value> {
    let cid = comp_id(s, p)?;
    let current = s.project.comp(cid).map(|c| c.resolution).unwrap_or_default();
    let is_on = current != Resolution::Auto && current != Resolution::Full || s.state.viewer.fast_previews != FastPreviews::Off;
    let on = p.get("on").and_then(Value::as_bool).unwrap_or(!is_on);
    if on {
        s.set_resolution(cid, Resolution::Half)?;
        s.state.viewer.fast_previews = FastPreviews::AdaptiveResolution;
    } else {
        s.set_resolution(cid, Resolution::Auto)?;
        s.state.viewer.fast_previews = FastPreviews::Off;
    }
    s.toast(if on { "Fast preview: half resolution while you work" } else { "Fast preview off: full quality" });
    Ok(json!({"on": on}))
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(
            "lumen.animateText",
            "Animate Text",
            ["Lumen", "Animate Text"],
            None,
            "{style?: typewriter|fadeUp|bounce|tracking|blur|drop (default fadeUp), layers?, text? (for a new text layer)} → the selected text layers, or a new one, animated from the current time",
            always,
            animate_text
        ),
        cmd!(
            "lumen.addTitle",
            "Add Animated Title",
            ["Lumen"],
            None,
            "{title?, subtitle?} → a title and subtitle that animate in at the current time and out 5 s later",
            always,
            add_title
        ),
        cmd!(
            "lumen.addLowerThird",
            "Add Lower Third",
            ["Lumen"],
            None,
            "{name?, role?, color? (#hex)} → a bar with a name and role that slides in at the current time",
            always,
            add_lower_third
        ),
        cmd!(
            "lumen.transition",
            "Quick Transition",
            ["Lumen", "Quick Transition"],
            None,
            "{style?: fade|slideLeft|slideUp|zoom|pop, edge?: in|out|both (default both), layers?} → the layers animate in at their In point and out at their Out point",
            has_comp,
            transition
        ),
        cmd!(
            "lumen.ease",
            "Easing Preset",
            ["Lumen", "Easing"],
            None,
            "{curve: smooth|slowDown|speedUp|snappy|linear|overshoot|bounce, layers?} → the selected keyframes (or every keyframe of the selected layers)",
            has_comp,
            ease
        ),
        cmd!(
            "lumen.reframe",
            "Reframe Copy",
            ["Lumen"],
            None,
            "{format?: 9x16|1x1|4x5|16x9 (default 9x16), mode?: blurFill|fill|fit (default blurFill), comp?} → a new composition in that shape with this one nested",
            has_comp,
            reframe
        ),
        cmd!(
            "lumen.exportSocial",
            "Export for Social Media",
            ["Lumen"],
            None,
            "{formats?: [original|9x16|1x1|4x5|16x9] (default all), start?: bool (default true), comp?} → MP4 renders of each format queued (and started)",
            has_comp,
            export_social
        ),
        cmd!(
            "lumen.exportAlpha",
            "Export with Transparency (WebM)",
            ["Lumen"],
            None,
            "{format?: webm|prores (default webm: VP9 with alpha), start?: bool (default true), comp?} → a render with an alpha channel queued (and started)",
            has_comp,
            export_alpha
        ),
        cmd!(
            "lumen.fastPreview",
            "Fast Preview",
            ["Lumen"],
            None,
            "{on?: bool (default: toggle)} → half resolution and adaptive previews, or back to full quality",
            has_comp,
            fast_preview
        ),
    ]
}
