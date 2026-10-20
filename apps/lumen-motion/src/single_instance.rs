//! One Lumen Motion window per user: opening a project while the app is already running (a
//! double-click on a `.lumeneffects` file in Explorer, Open With, a second launch) hands the files
//! to the running app, which opens them and comes to the front, instead of starting a second copy.
//!
//! The first instance listens on a localhost port derived from the user name ([`port`]). A later
//! launch connects; the running app greets it with [`BANNER`], the newcomer sends its files as one
//! JSON line (`{"open": ["C:\\…\\Intro.lumeneffects"]}`), gets `ok` back and exits. Anything else on
//! that port (another program, no answer within a couple of seconds) is ignored and the app simply
//! starts as usual: this is a convenience, never a reason not to start.
//!
//! Agent-driven runs (`--control`) skip it, so tests and agents can run several apps side by side;
//! so does `LUMEN_MOTION_MULTI_INSTANCE=1`.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

/// What the running app says first, so a launch never sends files to some other program.
pub const BANNER: &str = "LUMEN-MOTION-INSTANCE 1";

/// How long a launch waits for the running app before starting on its own.
const TIMEOUT: Duration = Duration::from_secs(2);

/// The longest request accepted (a few hundred paths).
const MAX_REQUEST: u64 = 256 * 1024;

/// The localhost port of this user's running Lumen Motion: in the dynamic range, from a hash of
/// the user name, so two people signed in to the same machine don't share one.
pub fn port(user: &str) -> u16 {
    // FNV-1a: stable across builds and platforms.
    let mut h: u32 = 0x811c_9dc5;
    for b in user.bytes() {
        h ^= u32::from(b);
        h = h.wrapping_mul(0x0100_0193);
    }
    49_400u16.saturating_add((h % 15_000) as u16)
}

/// This user's port ([`port`] of `USERNAME` / `USER`).
pub fn user_port() -> u16 {
    let user = std::env::var("USERNAME").or_else(|_| std::env::var("USER")).unwrap_or_default();
    port(&user)
}

/// How this launch starts.
pub enum Start {
    /// The first instance: open its own files and take other launches' ([`Inbox::listen`]).
    Primary(TcpListener),
    /// Another Lumen Motion took the files: exit.
    Forwarded,
    /// No single-instance service (the port belongs to something else): start as usual.
    Alone,
}

/// Become the running instance on `port`, or hand `files` to the one that already is.
pub fn start(port: u16, files: &[String]) -> Start {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    match TcpListener::bind(addr) {
        Ok(l) => Start::Primary(l),
        Err(_) => match forward(addr, files) {
            Ok(()) => Start::Forwarded,
            Err(e) => {
                log::info!("single instance: port {port} is not a Lumen Motion ({e}); starting on our own");
                Start::Alone
            }
        },
    }
}

/// Send `files` (made absolute against this process's folder) to the app listening at `addr`.
pub fn forward(addr: SocketAddr, files: &[String]) -> Result<(), String> {
    let mut s = TcpStream::connect_timeout(&addr, TIMEOUT).map_err(|e| e.to_string())?;
    s.set_read_timeout(Some(TIMEOUT)).map_err(|e| e.to_string())?;
    s.set_write_timeout(Some(TIMEOUT)).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(s.try_clone().map_err(|e| e.to_string())?).take(1024);
    let mut banner = String::new();
    reader.read_line(&mut banner).map_err(|e| e.to_string())?;
    if banner.trim_end() != BANNER {
        return Err("no greeting".into());
    }
    let paths: Vec<String> = files.iter().map(|f| std::path::absolute(f).map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|_| f.clone())).collect();
    let line = serde_json::json!({ "open": paths }).to_string();
    s.write_all(line.as_bytes()).and_then(|()| s.write_all(b"\n")).and_then(|()| s.flush()).map_err(|e| e.to_string())?;
    let mut reply = String::new();
    reader.read_line(&mut reply).map_err(|e| e.to_string())?;
    if reply.trim_end() == "ok" { Ok(()) } else { Err(format!("unexpected answer `{}`", reply.trim_end())) }
}

/// Files other launches handed to this app, waiting for the window to take them.
#[derive(Clone, Default)]
pub struct Inbox {
    queue: Arc<Mutex<Vec<PathBuf>>>,
}

impl Inbox {
    /// Answer other launches on `listener` from a background thread; `wake` runs when files
    /// arrive (the window repaints and comes to the front).
    pub fn listen(listener: TcpListener, wake: impl Fn() + Send + 'static) -> Self {
        let inbox = Inbox::default();
        let queue = inbox.queue.clone();
        let spawned = std::thread::Builder::new().name("single-instance".into()).spawn(move || {
            for conn in listener.incoming() {
                let Ok(conn) = conn else { continue };
                match serve(conn) {
                    Ok(paths) => {
                        queue.lock().unwrap_or_else(PoisonError::into_inner).extend(paths);
                        wake();
                    }
                    Err(e) => log::info!("single instance: a request was ignored: {e}"),
                }
            }
        });
        if let Err(e) = spawned {
            log::warn!("single instance: not listening: {e}");
        }
        inbox
    }

    /// The files received since the last call.
    pub fn take(&self) -> Vec<PathBuf> {
        std::mem::take(&mut *self.queue.lock().unwrap_or_else(PoisonError::into_inner))
    }
}

/// One launch's request: greet it, read its files and confirm.
fn serve(mut s: TcpStream) -> Result<Vec<PathBuf>, String> {
    s.set_read_timeout(Some(TIMEOUT)).map_err(|e| e.to_string())?;
    s.set_write_timeout(Some(TIMEOUT)).map_err(|e| e.to_string())?;
    s.write_all(format!("{BANNER}\n").as_bytes()).map_err(|e| e.to_string())?;
    let mut line = String::new();
    BufReader::new(s.try_clone().map_err(|e| e.to_string())?).take(MAX_REQUEST).read_line(&mut line).map_err(|e| e.to_string())?;
    let v: serde_json::Value = serde_json::from_str(line.trim()).map_err(|e| e.to_string())?;
    let paths = parse_request(&v);
    s.write_all(b"ok\n").map_err(|e| e.to_string())?;
    Ok(paths)
}

/// The existing files a request names (anything else is dropped).
fn parse_request(v: &serde_json::Value) -> Vec<PathBuf> {
    v.get("open")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(serde_json::Value::as_str)
        .map(PathBuf::from)
        .filter(|p| p.is_absolute() && p.is_file())
        .collect()
}

/// A file another launch asked us to open, presented to egui as a dropped file (the window's
/// drop handling opens projects, asking about unsaved changes first, and imports media).
#[derive(Debug)]
pub struct HandedFile(pub PathBuf);

impl egui::DroppedFile for HandedFile {
    fn path(&self) -> &Path {
        &self.0
    }

    fn bytes(&self) -> Result<Vec<u8>, String> {
        std::fs::read(&self.0).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ports_are_per_user_and_in_the_dynamic_range() {
        assert_eq!(port("ana"), port("ana"));
        assert_ne!(port("ana"), port("luis"));
        for u in ["", "ana", "Administrador", "ñandú", &"x".repeat(500)] {
            assert!((49_400..64_400).contains(&port(u)), "{u}");
        }
    }

    #[test]
    fn a_second_launch_hands_its_files_to_the_first() {
        let dir = std::env::temp_dir().join(format!("lm-single-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let project = dir.join("Intro.lumeneffects");
        std::fs::write(&project, "{}").unwrap();
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        let inbox = Inbox::listen(listener, move || {
            let _ = tx.send(());
        });
        forward(addr, &[project.to_string_lossy().to_string(), dir.join("missing.mp4").to_string_lossy().to_string()]).unwrap();
        rx.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(inbox.take(), vec![project.clone()], "only existing files arrive");
        assert!(inbox.take().is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn another_program_on_the_port_is_never_sent_files() {
        // A server that says nothing: the launch gives up and starts on its own.
        let quiet = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let addr = quiet.local_addr().unwrap();
        std::thread::spawn(move || {
            let _held = quiet.accept();
            std::thread::sleep(Duration::from_secs(4));
        });
        assert!(forward(addr, &["a.lumeneffects".into()]).is_err());
        // One that greets differently.
        let other = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let addr = other.local_addr().unwrap();
        std::thread::spawn(move || {
            if let Ok((mut s, _)) = other.accept() {
                let _ = s.write_all(b"HTTP/1.1 400 Bad Request\r\n");
            }
        });
        assert!(forward(addr, &["a.lumeneffects".into()]).is_err());
    }

    #[test]
    fn malformed_requests_open_nothing() {
        for v in [serde_json::json!(null), serde_json::json!({"open": "x"}), serde_json::json!({"open": [1, null, "relative.lumeneffects"]})] {
            assert!(parse_request(&v).is_empty(), "{v}");
        }
    }
}
