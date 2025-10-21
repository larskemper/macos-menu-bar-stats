#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use battery::State as BatteryState;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::System;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

const UPDATE_INTERVAL_SECS: u64 = 2;
const BYTES_TO_GB: f32 = 1024.0 * 1024.0 * 1024.0;
const TRAY_ID: &str = "main";

const MENU_BATTERY: &str = "battery";
const MENU_CPU: &str = "cpu";
const MENU_MEMORY: &str = "memory";
const MENU_QUIT: &str = "quit";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SystemStats {
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
    memory_percent: f32,
    battery_percent: f32,
    battery_state: String,
}

struct AppState {
    system: Mutex<System>,
}

fn collect_system_stats(sys: &mut System) -> SystemStats {
    sys.refresh_cpu();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();
    let memory_percent = (memory_used as f32 / memory_total as f32) * 100.0;

    let (battery_percent, battery_state) = get_battery_info();

    SystemStats {
        cpu_usage,
        memory_used,
        memory_total,
        memory_percent,
        battery_percent,
        battery_state,
    }
}

fn get_battery_info() -> (f32, String) {
    battery::Manager::new()
        .ok()
        .and_then(|manager| manager.batteries().ok())
        .and_then(|mut batteries| batteries.next())
        .and_then(|battery| battery.ok())
        .map(|battery| {
            let percent = battery.state_of_charge().value * 100.0;
            let state = format_battery_state(battery.state());
            (percent, state)
        })
        .unwrap_or((0.0, "Unknown".to_string()))
}

fn format_battery_state(state: BatteryState) -> String {
    match state {
        BatteryState::Charging => "Charging".to_string(),
        BatteryState::Discharging => "Discharging".to_string(),
        BatteryState::Full => "Full".to_string(),
        BatteryState::Empty => "Empty".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn bytes_to_gb(bytes: u64) -> f32 {
    bytes as f32 / BYTES_TO_GB
}

fn format_tray_title(stats: &SystemStats) -> String {
    format!(
        "🔋{}%   🧠{}%   💾{}%",
        stats.battery_percent.round() as i32,
        stats.cpu_usage.round() as i32,
        stats.memory_percent.round() as i32
    )
}

fn format_battery_text(stats: &SystemStats) -> String {
    format!(
        "🔋 Battery: {}% ({})",
        stats.battery_percent.round() as i32,
        stats.battery_state
    )
}

fn format_cpu_text(stats: &SystemStats) -> String {
    format!("🧠 CPU Usage: {:.1}%", stats.cpu_usage)
}

fn format_memory_text(stats: &SystemStats) -> String {
    format!(
        "💾 Memory: {:.1}% ({:.2} GB / {:.2} GB)",
        stats.memory_percent,
        bytes_to_gb(stats.memory_used),
        bytes_to_gb(stats.memory_total)
    )
}

fn update_menu_items<R: tauri::Runtime>(
    stats: &SystemStats,
    battery: &MenuItem<R>,
    cpu: &MenuItem<R>,
    memory: &MenuItem<R>,
) {
    let _ = battery.set_text(format_battery_text(stats));
    let _ = cpu.set_text(format_cpu_text(stats));
    let _ = memory.set_text(format_memory_text(stats));
}

fn handle_menu_click<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    event_id: &str,
    current_stats: &Arc<Mutex<Option<SystemStats>>>,
) {
    let stats = match current_stats.lock() {
        Ok(guard) => match guard.as_ref() {
            Some(stats) => stats.clone(),
            None => return,
        },
        Err(_) => return,
    };

    let text = match event_id {
        MENU_BATTERY => format!("{}%", stats.battery_percent.round() as i32),
        MENU_CPU => format!("{:.1}%", stats.cpu_usage),
        MENU_MEMORY => format!("{:.1}%", stats.memory_percent),
        _ => return,
    };

    let _ = app.clipboard().write_text(text);
}

fn spawn_stats_updater<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    current_stats: Arc<Mutex<Option<SystemStats>>>,
    battery_item: MenuItem<R>,
    cpu_item: MenuItem<R>,
    memory_item: MenuItem<R>,
) {
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(UPDATE_INTERVAL_SECS));

        let Some(state) = app_handle.try_state::<AppState>() else {
            continue;
        };

        let Ok(mut sys) = state.system.lock() else {
            continue;
        };

        let stats = collect_system_stats(&mut sys);

        if let Ok(mut current) = current_stats.lock() {
            *current = Some(stats.clone());
        }

        if let Some(tray) = app_handle.tray_by_id(TRAY_ID) {
            let _ = tray.set_title(Some(&format_tray_title(&stats)));
        }

        update_menu_items(&stats, &battery_item, &cpu_item, &memory_item);
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    sys.refresh_memory();

    let current_stats: Arc<Mutex<Option<SystemStats>>> = Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(AppState {
            system: Mutex::new(sys),
        })
        .setup(move |app| {
            let battery_item =
                MenuItem::with_id(app, MENU_BATTERY, "Battery: Loading...", true, None::<&str>)?;
            let cpu_item = MenuItem::with_id(app, MENU_CPU, "CPU: Loading...", true, None::<&str>)?;
            let memory_item =
                MenuItem::with_id(app, MENU_MEMORY, "Memory: Loading...", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, MENU_QUIT, "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &battery_item,
                    &cpu_item,
                    &memory_item,
                    &separator,
                    &quit_item,
                ],
            )?;

            let current_stats_for_menu = current_stats.clone();
            let tray = TrayIconBuilder::with_id(TRAY_ID)
                .menu(&menu)
                .title("Loading...")
                .icon_as_template(true)
                .on_menu_event(move |app, event| {
                    if event.id.as_ref() == MENU_QUIT {
                        app.exit(0);
                    } else {
                        handle_menu_click(app, event.id.as_ref(), &current_stats_for_menu);
                    }
                })
                .build(app)?;

            if let Some(state) = app.try_state::<AppState>() {
                if let Ok(mut sys) = state.system.lock() {
                    let stats = collect_system_stats(&mut sys);

                    if let Ok(mut current) = current_stats.lock() {
                        *current = Some(stats.clone());
                    }

                    let _ = tray.set_title(Some(&format_tray_title(&stats)));
                    update_menu_items(&stats, &battery_item, &cpu_item, &memory_item);
                }
            }

            spawn_stats_updater(
                app.handle().clone(),
                current_stats.clone(),
                battery_item.clone(),
                cpu_item.clone(),
                memory_item.clone(),
            );

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
