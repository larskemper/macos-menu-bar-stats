#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri_plugin_clipboard_manager::ClipboardExt;
use sysinfo::System;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
    current_stats: Arc<Mutex<Option<SystemStats>>>,
}

fn get_stats_sync(sys: &mut System) -> SystemStats {
    // Refresh system information
    sys.refresh_cpu();
    sys.refresh_memory();
    
    // Get CPU usage (average across all cores)
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    
    // Get memory info
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();
    let memory_percent = (memory_used as f32 / memory_total as f32) * 100.0;
    
    // Get battery info
    let mut battery_percent = 0.0;
    let mut battery_state = "Unknown".to_string();
    
    if let Ok(battery_manager) = battery::Manager::new() {
        if let Ok(batteries) = battery_manager.batteries() {
            for battery in batteries {
                if let Ok(battery) = battery {
                    battery_percent = battery.state_of_charge().value * 100.0;
                    battery_state = format!("{:?}", battery.state());
                    break;
                }
            }
        }
    }
    
    SystemStats {
        cpu_usage,
        memory_used,
        memory_total,
        memory_percent,
        battery_percent,
        battery_state,
    }
}

#[tauri::command]
async fn get_system_stats(state: tauri::State<'_, AppState>) -> Result<SystemStats, String> {
    let mut sys = state.system.lock().map_err(|e| e.to_string())?;
    Ok(get_stats_sync(&mut sys))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    sys.refresh_memory();
    
    let current_stats = Arc::new(Mutex::new(None));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(AppState {
            system: Mutex::new(sys),
            current_stats: current_stats.clone(),
        })
        .invoke_handler(tauri::generate_handler![get_system_stats])
        .setup(move |app| {
            // Create menu items
            let battery_item = MenuItem::with_id(app, "battery", "Battery: Loading...", true, None::<&str>)?;
            let cpu_item = MenuItem::with_id(app, "cpu", "CPU: Loading...", true, None::<&str>)?;
            let memory_item = MenuItem::with_id(app, "memory", "Memory: Loading...", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            
            let menu = Menu::with_items(
                app,
                &[&battery_item, &cpu_item, &memory_item, &separator, &quit_item],
            )?;

            let current_stats_for_menu = current_stats.clone();
            let tray = TrayIconBuilder::with_id("main")
                .menu(&menu)
                .title("Loading...")
                .icon_as_template(true)
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "battery" => {
                            if let Ok(stats) = current_stats_for_menu.lock() {
                                if let Some(stats) = stats.as_ref() {
                                    let percentage = format!("{}%", stats.battery_percent.round() as i32);
                                    let _ = app.clipboard().write_text(percentage);
                                }
                            }
                        }
                        "cpu" => {
                            if let Ok(stats) = current_stats_for_menu.lock() {
                                if let Some(stats) = stats.as_ref() {
                                    let percentage = format!("{:.1}%", stats.cpu_usage);
                                    let _ = app.clipboard().write_text(percentage);
                                }
                            }
                        }
                        "memory" => {
                            if let Ok(stats) = current_stats_for_menu.lock() {
                                if let Some(stats) = stats.as_ref() {
                                    let percentage = format!("{:.1}%", stats.memory_percent);
                                    let _ = app.clipboard().write_text(percentage);
                                }
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // Clone menu items for the update thread
            let battery_item_clone = battery_item.clone();
            let cpu_item_clone = cpu_item.clone();
            let memory_item_clone = memory_item.clone();

            // Update tray title and menu periodically
            let app_handle = app.handle().clone();
            let current_stats_for_thread = current_stats.clone();
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(Duration::from_secs(2));
                    
                    if let Some(state) = app_handle.try_state::<AppState>() {
                        if let Ok(mut sys) = state.system.lock() {
                            let stats = get_stats_sync(&mut sys);
                            
                            // Store current stats for clipboard access
                            if let Ok(mut current) = current_stats_for_thread.lock() {
                                *current = Some(stats.clone());
                            }
                            let title = format!(
                                "🔋{}%   🧠{}%   💾{}%",
                                stats.battery_percent.round() as i32,
                                stats.cpu_usage.round() as i32,
                                stats.memory_percent.round() as i32
                            );
                            
                            if let Some(tray) = app_handle.tray_by_id("main") {
                                let _ = tray.set_title(Some(&title));
                            }
                            
                            // Update menu items
                            let memory_used_gb = stats.memory_used as f32 / (1024.0 * 1024.0 * 1024.0);
                            let memory_total_gb = stats.memory_total as f32 / (1024.0 * 1024.0 * 1024.0);
                            
                            let _ = battery_item_clone.set_text(format!(
                                "🔋 Battery: {}% ({})",
                                stats.battery_percent.round() as i32,
                                stats.battery_state
                            ));
                            let _ = cpu_item_clone.set_text(format!(
                                "🧠 CPU Usage: {:.1}%",
                                stats.cpu_usage
                            ));
                            let _ = memory_item_clone.set_text(format!(
                                "💾 Memory: {:.1}% ({:.2} GB / {:.2} GB)",
                                stats.memory_percent,
                                memory_used_gb,
                                memory_total_gb
                            ));
                        }
                    }
                }
            });

            // Set initial tray title and menu
            if let Some(state) = app.try_state::<AppState>() {
                if let Ok(mut sys) = state.system.lock() {
                    let stats = get_stats_sync(&mut sys);
                    
                    // Store initial stats
                    if let Ok(mut current) = current_stats.lock() {
                        *current = Some(stats.clone());
                    }
                    let title = format!(
                        "🔋{}%   🧠{}%   💾{}%",
                        stats.battery_percent.round() as i32,
                        stats.cpu_usage.round() as i32,
                        stats.memory_percent.round() as i32
                    );
                    let _ = tray.set_title(Some(&title));
                    
                    // Update initial menu items
                    let memory_used_gb = stats.memory_used as f32 / (1024.0 * 1024.0 * 1024.0);
                    let memory_total_gb = stats.memory_total as f32 / (1024.0 * 1024.0 * 1024.0);
                    
                    let _ = battery_item.set_text(format!(
                        "🔋 Battery: {}% ({})",
                        stats.battery_percent.round() as i32,
                        stats.battery_state
                    ));
                    let _ = cpu_item.set_text(format!(
                        "🧠 CPU Usage: {:.1}%",
                        stats.cpu_usage
                    ));
                    let _ = memory_item.set_text(format!(
                        "💾 Memory: {:.1}% ({:.2} GB / {:.2} GB)",
                        stats.memory_percent,
                        memory_used_gb,
                        memory_total_gb
                    ));
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}