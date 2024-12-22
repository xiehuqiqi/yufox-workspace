use tauri::{AppHandle, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|yufox, argv, _cwd| {
            println!("a new app instance was opened with {argv:?} and the deep link event was already triggered");
            // when defining deep link schemes at runtime, you must also check `argv` here
            let _ = show_window(yufox);
        }));
    }

    builder = builder.plugin(tauri_plugin_deep_link::init());
    
    builder
        // https://github.com/ayangweb/tauri-plugin-fs-pro
        .plugin(tauri_plugin_fs_pro::init())
        // https://github.com/HuakunShen/tauri-plugin-network
        .plugin(tauri_plugin_network::init())
        // https://github.com/HuakunShen/tauri-plugin-system-info
        .plugin(tauri_plugin_system_info::init())
        .plugin(tauri_plugin_log::Builder::new()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("logs".to_string()),
                },
            ))
            .format(|out, message, record| {
                out.finish(format_args!(
                  "[{} {}] {}",
                  record.level(),
                  record.target(),
                  message
                ))
              })
            .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
            .max_file_size(40_960 /* bytes */)
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
            .build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_upload::init())
        // .plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init());

    builder
        .setup(|yufox| {
            let salt_path = yufox
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            yufox.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        .setup(|yufox| {
            if cfg!(debug_assertions) {
                yufox.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .setup(|yufox| {
            #[cfg(desktop)]
            let _ = yufox
                .handle()
                .plugin(tauri_plugin_updater::Builder::new().build());

            Ok(())
        });

    builder
        .run(context)
        .expect("error while running tauri application");
}


fn show_window(yufox: &AppHandle) {
    let windows = yufox.webview_windows();
    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}
