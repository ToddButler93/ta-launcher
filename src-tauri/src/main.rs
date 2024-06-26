// main.rs

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
use commands::{
    inject_command, 
    launch_game_command, 
    fetch_players_command, 
    fetch_packages_command, 
    downloader_command,
    find_executable_command,
    get_available_fonts_command,
    util_command,
    log_cleaner_command,
    directory_shortcuts_command,
    config_parser_command,
    route_parser_command,
    route_decoder_command,
    fetch_available_dlls,
    python_decoder_command,
    arrange_maps_folder_command
};


fn main() {
    // Tauri app builder and run
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        inject_command::inject, 
        launch_game_command::launch_game,
        launch_game_command::launch_game_non_steam,
        fetch_players_command::fetch_players_online, 
        fetch_packages_command::fetch_package_metadata,
        fetch_packages_command::fetch_packages,
        fetch_packages_command::fetch_dependency_tree,
        downloader_command::download_package,
        find_executable_command:: find_executable,
        get_available_fonts_command::get_system_fonts,
        util_command::is_process_running,
        util_command::is_pid_running,
        util_command::get_process_pid,
        log_cleaner_command::check_directory_stats,
        log_cleaner_command::clear_log_folder,
        directory_shortcuts_command::open_config_dir,
        directory_shortcuts_command::open_routes_sub_dir,
        directory_shortcuts_command::open_hud_modules_sub_dir,
        directory_shortcuts_command::open_launcher_dir,
        config_parser_command::parse_tribes_ini,
        config_parser_command::parse_tribes_input_ini,
        config_parser_command::update_ini_file,
        route_parser_command::get_route_files,
        route_parser_command::delete_route_file,
        route_decoder_command::decode_route,
        fetch_available_dlls::get_available_dlls,
        python_decoder_command::python_route_decoder,
        arrange_maps_folder_command::arrange_maps_folder,
        
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
