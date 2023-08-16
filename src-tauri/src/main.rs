// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command, os::windows::process::CommandExt,
};
const DETACHED_PROCESS: u32 = 0x00000008;

use tauri::api::path;

fn get_save_path(origin_path: &str) -> String {
    let path = origin_path.to_string().replace("mp4", "mp3");
    get_unique_name(path, None)
}

fn get_unique_name(path: String, id: Option<i8>) -> String {
    let parts: Vec<&str> = path.split(".").collect();
    let file_name = parts[0];
    let suffix = parts[1];

    let new_file_name = match id {
        Some(id) => {
            format!("{}({}).{}", file_name, id, suffix)
        }
        None => path.clone(),
    };

    let new_id = match id {
        Some(id) => id + 1,
        None => 1,
    };

    match fs::metadata(&new_file_name) {
        Ok(_) => get_unique_name(path, Some(new_id)),
        Err(_) => new_file_name,
    }
}

fn concat(origin_path: &str, ffmpeg: &OsString, silent_audio: String, raw_mp3: String) -> String {
    let file_name = get_save_path(origin_path);

    let concat_command = format!("concat:{}|{}", silent_audio.as_str(), raw_mp3.as_str());

    let res = Command::new(ffmpeg)
        .args([
            "-y",
            "-i",
            concat_command.as_str(),
            "-acodec",
            "copy",
            file_name.as_str(),
        ])
        .creation_flags(DETACHED_PROCESS)
        .output();

    match res {
        Ok(_) => {
            String::from(file_name)
        }
        Err(_) => panic!("concat failed"),
    }
}

fn store_to_dir(file_name: &str) -> PathBuf {
    let data_dir = path::data_dir();

    match data_dir {
        Some(dir) => Path::new(&dir).join(file_name),
        None => panic!("cannot find home dir"),
    }
}

fn get_silent_audio(num: &str, ffmpeg: &OsString) -> String {
    let file_name = store_to_dir("silent-audio.mp3").display().to_string();

    let res = Command::new(ffmpeg)
        .args([
            "-y",
            "-f",
            "lavfi",
            "-i",
            "anullsrc",
            "-t",
            num,
            file_name.as_str(),
        ])
        .creation_flags(DETACHED_PROCESS)
        .output();

    match res {
        Ok(_) => file_name,
        Err(_) => panic!("get silent audio failed"),
    }
}

fn get_mp3(video_url: &str, ffmpeg: &OsString) -> String {
    let file_name = store_to_dir("raw.mp3").display().to_string();

    let res = Command::new(ffmpeg)
        .args(["-y", "-i", video_url, file_name.as_str()])
        .creation_flags(DETACHED_PROCESS)
        .output();

    match res {
        Ok(_) => file_name,
        Err(_) => panic!("get raw mp3 failed"),
    }
}

fn get_ffmpeg(handle: tauri::AppHandle) -> OsString {
    let resource_path = handle
        .path_resolver()
        .resolve_resource("resources/ffmpeg.exe")
        .expect("failed to resolve resource");

    resource_path.into_os_string()
}

#[tauri::command]
fn transform(handle: tauri::AppHandle, num: &str, video_url: &str) -> String {
    let ffmpeg = get_ffmpeg(handle);
    let raw_mp3 = get_mp3(video_url, &ffmpeg);
    let silent_audio = get_silent_audio(num, &ffmpeg);
    let new_file = concat(video_url, &ffmpeg, silent_audio, raw_mp3);

    format!("save at{}", new_file)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![transform])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
