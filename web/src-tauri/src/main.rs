use std::io;
use std::net::{SocketAddr, TcpStream};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

const GATEWAY_HOST: &str = "127.0.0.1";
const GATEWAY_PORT: u16 = 9573;
const GATEWAY_BOOT_TIMEOUT_SECS: u64 = 20;

type SharedChild = Arc<Mutex<Option<Child>>>;

fn gateway_addr() -> String {
    format!("http://{GATEWAY_HOST}:{GATEWAY_PORT}")
}

fn wait_for_gateway_ready() -> io::Result<()> {
    let start = Instant::now();
    let addr: SocketAddr = format!("{GATEWAY_HOST}:{GATEWAY_PORT}")
        .parse()
        .map_err(io::Error::other)?;

    while start.elapsed() < Duration::from_secs(GATEWAY_BOOT_TIMEOUT_SECS) {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(250)).is_ok() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(250));
    }

    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        "gateway did not start in time",
    ))
}

fn spawn_gateway() -> io::Result<Child> {
    let bin = std::env::var("ZEROCLAW_BIN").unwrap_or_else(|_| "zeroclaw".to_string());

    Command::new(bin)
        .arg("gateway")
        .arg("--host")
        .arg(GATEWAY_HOST)
        .arg("--port")
        .arg(GATEWAY_PORT.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

fn kill_gateway(child: &SharedChild) {
    if let Ok(mut guard) = child.lock() {
        if let Some(process) = guard.as_mut() {
            let _ = process.kill();
            let _ = process.wait();
        }
        *guard = None;
    }
}

fn start_gateway_with_fallback(app: &AppHandle, child: &SharedChild) {
    match spawn_gateway().and_then(|process| {
        if let Ok(mut guard) = child.lock() {
            *guard = Some(process);
        }
        wait_for_gateway_ready()
    }) {
        Ok(()) => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval(&format!("window.location.replace({:?});", gateway_addr()));
            }
        }
        Err(error) => {
            let message = format!(
                "<h2>ZeroClaw Desktop</h2><p>Unable to start local gateway.</p><pre>{}</pre><p>Run <code>zeroclaw gateway --host 127.0.0.1 --port 9573</code> and relaunch.</p>",
                error
            );
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval(&format!(
                    "document.body.innerHTML = {message:?}; document.title = 'ZeroClaw Desktop';"
                ));
            }
        }
    }
}

fn main() {
    let gateway_process: SharedChild = Arc::new(Mutex::new(None));
    let gateway_process_for_setup = Arc::clone(&gateway_process);
    let gateway_process_for_exit = Arc::clone(&gateway_process);

    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let child_ref = Arc::clone(&gateway_process_for_setup);
            thread::spawn(move || start_gateway_with_fallback(&app_handle, &child_ref));
            Ok(())
        })
        .on_window_event(move |window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                if window.label() == "main" {
                    kill_gateway(&gateway_process_for_exit);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run desktop app");
}
