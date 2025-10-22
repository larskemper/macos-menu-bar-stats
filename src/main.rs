#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use battery::State as BatteryState;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::System;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, CheckMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_clipboard_manager::ClipboardExt;

const UPDATE_INTERVAL_SECS: u64 = 3;
const BYTES_TO_GB: f32 = 1024.0 * 1024.0 * 1024.0;
const TRAY_ID: &str = "menu_bar_stats_tray";

const MENU_BATTERY: &str = "battery";
const MENU_CPU: &str = "cpu";
const MENU_MEMORY: &str = "memory";
const MENU_AUTOSTART: &str = "autostart";
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

    let cpu_usage = sys.global_cpu_info().cpu_usage().clamp(0.0, 100.0);

    let memory_total = sys.total_memory();
    let memory_available = sys.available_memory().min(memory_total);
    let memory_used = memory_total - memory_available;

    let memory_percent = if memory_total > 0 {
        ((memory_used as f64 / memory_total as f64) * 100.0).clamp(0.0, 100.0) as f32
    } else {
        0.0
    };

    let (battery_percent, battery_state) = get_battery_info();

    SystemStats {
        cpu_usage,
        memory_used,
        memory_total,
        memory_percent,
        battery_percent: battery_percent.clamp(0.0, 100.0),
        battery_state,
    }
}

fn get_battery_info() -> (f32, String) {
    match battery::Manager::new() {
        Ok(manager) => match manager.batteries() {
            Ok(mut batteries) => {
                if let Some(Ok(battery)) = batteries.next() {
                    let percent = battery.state_of_charge().value * 100.0;
                    let state = format_battery_state(battery.state());
                    (percent, state)
                } else {
                    (0.0, "No Battery".to_string())
                }
            }
            Err(e) => {
                eprintln!("Failed to get batteries: {}", e);
                (0.0, "Unknown".to_string())
            }
        },
        Err(e) => {
            eprintln!("Failed to create battery manager: {}", e);
            (0.0, "Unknown".to_string())
        }
    }
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
    if let Err(e) = battery.set_text(format_battery_text(stats)) {
        eprintln!("Failed to update battery menu item: {}", e);
    }

    if let Err(e) = cpu.set_text(format_cpu_text(stats)) {
        eprintln!("Failed to update CPU menu item: {}", e);
    }
    
    if let Err(e) = memory.set_text(format_memory_text(stats)) {
        eprintln!("Failed to update memory menu item: {}", e);
    }
}

fn handle_menu_click<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    event_id: &str,
    current_stats: &Arc<Mutex<Option<SystemStats>>>,
) {
    let stats = match current_stats.lock() {
        Ok(guard) => match guard.as_ref() {
            Some(stats) => stats.clone(),
            None => {
                eprintln!("No stats available to copy");
                return;
            }
        },
        Err(e) => {
            eprintln!("Failed to lock stats mutex: {}", e);
            return;
        }
    };

    let text = match event_id {
        MENU_BATTERY => format!("{}%", stats.battery_percent.round() as i32),
        MENU_CPU => format!("{:.1}%", stats.cpu_usage),
        MENU_MEMORY => format!("{:.1}%", stats.memory_percent),
        _ => return,
    };

    if let Err(e) = app.clipboard().write_text(text) {
        eprintln!("Failed to write to clipboard: {}", e);
    }
}

fn spawn_stats_updater<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    current_stats: Arc<Mutex<Option<SystemStats>>>,
    battery_item: MenuItem<R>,
    cpu_item: MenuItem<R>,
    memory_item: MenuItem<R>,
) {
    std::thread::Builder::new()
        .name("stats-updater".to_string())
        .spawn(move || loop {
            std::thread::sleep(Duration::from_secs(UPDATE_INTERVAL_SECS));

            let Some(state) = app_handle.try_state::<AppState>() else {
                eprintln!("Failed to get app state");
                continue;
            };

            let Ok(mut sys) = state.system.lock() else {
                eprintln!("Failed to lock system mutex");
                continue;
            };

            let stats = collect_system_stats(&mut sys);

            if let Ok(mut current) = current_stats.lock() {
                *current = Some(stats.clone());
            } else {
                eprintln!("Failed to lock current stats mutex");
            }

            if let Some(tray) = app_handle.tray_by_id(TRAY_ID) {
                if let Err(e) = tray.set_title(Some(&format_tray_title(&stats))) {
                    eprintln!("Failed to update tray title: {}", e);
                }
            }

            update_menu_items(&stats, &battery_item, &cpu_item, &memory_item);
        })
        .expect("Failed to spawn stats updater thread");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let mut sys = System::new();
    sys.refresh_cpu();
    sys.refresh_memory();

    let current_stats: Arc<Mutex<Option<SystemStats>>> = Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--flag", "autostart"]),
        ))
        .manage(AppState {
            system: Mutex::new(sys),
        })
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let battery_item =
                MenuItem::with_id(app, MENU_BATTERY, "Battery: Loading...", true, None::<&str>)?;
            let cpu_item = MenuItem::with_id(app, MENU_CPU, "CPU: Loading...", true, None::<&str>)?;
            let memory_item =
                MenuItem::with_id(app, MENU_MEMORY, "Memory: Loading...", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            
            let autostart_manager = app.autolaunch();
            let is_autostart_enabled = autostart_manager.is_enabled().unwrap_or(false);
            let autostart_item = CheckMenuItem::with_id(
                app,
                MENU_AUTOSTART,
                "Start at Login",
                true,
                is_autostart_enabled,
                None::<&str>,
            )?;
            
            let quit_item = MenuItem::with_id(app, MENU_QUIT, "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &battery_item,
                    &cpu_item,
                    &memory_item,
                    &separator,
                    &autostart_item,
                    &quit_item,
                ],
            )?;

            let current_stats_for_menu = current_stats.clone();
            let autostart_item_clone = autostart_item.clone();
            let tray = TrayIconBuilder::with_id(TRAY_ID)
                .menu(&menu)
                .title("Loading...")
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        MENU_QUIT => app.exit(0),
                        MENU_AUTOSTART => {
                            let autostart_manager = app.autolaunch();
                            match autostart_manager.is_enabled() {
                                Ok(is_enabled) => {
                                    let result = if is_enabled {
                                        autostart_manager.disable()
                                    } else {
                                        autostart_manager.enable()
                                    };
                                    
                                    match result {
                                        Ok(_) => {
                                            if let Err(e) = autostart_item_clone.set_checked(!is_enabled) {
                                                eprintln!("Failed to update autostart checkbox: {}", e);
                                            }
                                        }
                                        Err(e) => eprintln!("Failed to toggle autostart: {}", e),
                                    }
                                }
                                Err(e) => eprintln!("Failed to check autostart status: {}", e),
                            }
                        }
                        _ => handle_menu_click(app, event.id.as_ref(), &current_stats_for_menu),
                    }
                })
                .build(app)?;

            if let Some(state) = app.try_state::<AppState>() {
                if let Ok(mut sys) = state.system.lock() {
                    let stats = collect_system_stats(&mut sys);

                    if let Ok(mut current) = current_stats.lock() {
                        *current = Some(stats.clone());
                    }

                    if let Err(e) = tray.set_title(Some(&format_tray_title(&stats))) {
                        eprintln!("Failed to set initial tray title: {}", e);
                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_gb_conversion() {
        assert_eq!(bytes_to_gb(0), 0.0);
        assert_eq!(bytes_to_gb(1073741824), 1.0);
        assert_eq!(bytes_to_gb(2147483648), 2.0);

        let half_gb = bytes_to_gb(536870912);
        assert!((half_gb - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_format_battery_state() {
        assert_eq!(format_battery_state(BatteryState::Charging), "Charging");
        assert_eq!(
            format_battery_state(BatteryState::Discharging),
            "Discharging"
        );
        assert_eq!(format_battery_state(BatteryState::Full), "Full");
        assert_eq!(format_battery_state(BatteryState::Empty), "Empty");
    }

    #[test]
    fn test_format_tray_title() {
        let stats = SystemStats {
            cpu_usage: 45.7,
            memory_used: 8589934592,
            memory_total: 17179869184,
            memory_percent: 50.0,
            battery_percent: 85.3,
            battery_state: "Charging".to_string(),
        };

        let title = format_tray_title(&stats);
        assert!(title.contains("85%"));
        assert!(title.contains("46%"));
        assert!(title.contains("50%"));
    }

    #[test]
    fn test_format_battery_text() {
        let stats = SystemStats {
            cpu_usage: 0.0,
            memory_used: 0,
            memory_total: 0,
            memory_percent: 0.0,
            battery_percent: 75.5,
            battery_state: "Discharging".to_string(),
        };

        let text = format_battery_text(&stats);
        assert!(text.contains("76%"));
        assert!(text.contains("Discharging"));
    }

    #[test]
    fn test_format_cpu_text() {
        let stats = SystemStats {
            cpu_usage: 33.7,
            memory_used: 0,
            memory_total: 0,
            memory_percent: 0.0,
            battery_percent: 0.0,
            battery_state: "Unknown".to_string(),
        };

        let text = format_cpu_text(&stats);
        assert!(text.contains("33.7%"));
    }

    #[test]
    fn test_format_memory_text() {
        let stats = SystemStats {
            cpu_usage: 0.0,
            memory_used: 8589934592,
            memory_total: 17179869184,
            memory_percent: 50.0,
            battery_percent: 0.0,
            battery_state: "Unknown".to_string(),
        };

        let text = format_memory_text(&stats);
        assert!(text.contains("50.0%"));
        assert!(text.contains("8.00 GB"));
        assert!(text.contains("16.00 GB"));
    }

    #[test]
    fn test_collect_system_stats_validation() {
        let mut sys = System::new();
        sys.refresh_cpu();
        sys.refresh_memory();

        let stats = collect_system_stats(&mut sys);

        assert!(stats.cpu_usage >= 0.0 && stats.cpu_usage <= 100.0);
        assert!(stats.memory_percent >= 0.0 && stats.memory_percent <= 100.0);
        assert!(stats.battery_percent >= 0.0 && stats.battery_percent <= 100.0);
        assert!(stats.memory_used <= stats.memory_total);
    }

    #[test]
    fn test_system_stats_clone() {
        let stats = SystemStats {
            cpu_usage: 50.0,
            memory_used: 1073741824,
            memory_total: 2147483648,
            memory_percent: 50.0,
            battery_percent: 80.0,
            battery_state: "Charging".to_string(),
        };

        let cloned = stats.clone();
        assert_eq!(stats.cpu_usage, cloned.cpu_usage);
        assert_eq!(stats.memory_used, cloned.memory_used);
        assert_eq!(stats.memory_total, cloned.memory_total);
        assert_eq!(stats.memory_percent, cloned.memory_percent);
        assert_eq!(stats.battery_percent, cloned.battery_percent);
        assert_eq!(stats.battery_state, cloned.battery_state);
    }

    #[test]
    fn test_constants() {
        assert_eq!(UPDATE_INTERVAL_SECS, 3);
        assert_eq!(BYTES_TO_GB, 1073741824.0);
        assert_eq!(TRAY_ID, "menu_bar_stats_tray");
        assert_eq!(MENU_BATTERY, "battery");
        assert_eq!(MENU_CPU, "cpu");
        assert_eq!(MENU_MEMORY, "memory");
        assert_eq!(MENU_AUTOSTART, "autostart");
        assert_eq!(MENU_QUIT, "quit");
    }
}
