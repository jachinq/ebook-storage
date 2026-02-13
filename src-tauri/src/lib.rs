use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_store::StoreExt;
use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &["pdf", "epub", "mobi", "azw3", "txt"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified: String,
}

pub struct AppState {
    books: Mutex<Vec<BookInfo>>,
}

// ========== 目录配置 ==========

#[tauri::command]
async fn get_directories(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let store = app
        .store("config.json")
        .map_err(|e| e.to_string())?;
    let dirs: Vec<String> = store
        .get("directories")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    Ok(dirs)
}

#[tauri::command]
async fn add_directory(app: tauri::AppHandle, path: String) -> Result<Vec<String>, String> {
    let p = Path::new(&path);
    if !p.exists() || !p.is_dir() {
        return Err("目录不存在".into());
    }

    let store = app.store("config.json").map_err(|e| e.to_string())?;
    let mut dirs: Vec<String> = store
        .get("directories")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let canonical = p.to_string_lossy().to_string();
    if !dirs.contains(&canonical) {
        dirs.push(canonical);
        store.set("directories", serde_json::json!(dirs));
    }

    Ok(dirs)
}

#[tauri::command]
async fn remove_directory(app: tauri::AppHandle, path: String) -> Result<Vec<String>, String> {
    let store = app.store("config.json").map_err(|e| e.to_string())?;
    let mut dirs: Vec<String> = store
        .get("directories")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    dirs.retain(|d| d != &path);
    store.set("directories", serde_json::json!(dirs));

    Ok(dirs)
}

// ========== 书籍扫描 ==========

fn scan_directory(dir: &str) -> Vec<BookInfo> {
    let mut books = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());
        let ext = match ext {
            Some(e) => e,
            None => continue,
        };
        if !SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }

        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let modified = metadata
            .modified()
            .ok()
            .map(|t| {
                let datetime: chrono::DateTime<chrono::Local> = t.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            })
            .unwrap_or_default();

        books.push(BookInfo {
            name: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: path.to_string_lossy().to_string(),
            size: metadata.len(),
            modified,
        });
    }
    books
}

#[tauri::command]
async fn scan_books(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<Vec<BookInfo>, String> {
    let store = app.store("config.json").map_err(|e| e.to_string())?;
    let dirs: Vec<String> = store
        .get("directories")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let mut all_books = Vec::new();
    for dir in &dirs {
        all_books.extend(scan_directory(dir));
    }

    let mut books = state.books.lock().map_err(|e| e.to_string())?;
    *books = all_books.clone();

    Ok(all_books)
}

// ========== 模糊搜索 ==========

#[tauri::command]
async fn search_books(state: State<'_, AppState>, keyword: String) -> Result<Vec<BookInfo>, String> {
    let books = state.books.lock().map_err(|e| e.to_string())?;

    if keyword.is_empty() {
        return Ok(books.clone());
    }

    let keyword_lower = keyword.to_lowercase();
    let results: Vec<BookInfo> = books
        .iter()
        .filter(|b| b.name.to_lowercase().contains(&keyword_lower))
        .cloned()
        .collect();

    Ok(results)
}

// ========== 文件操作 ==========

#[tauri::command]
async fn open_file(path: String) -> Result<(), String> {
    tauri_plugin_opener::open_path(&path, None::<&str>).map_err(|e| e.to_string())
}

#[tauri::command]
async fn reveal_file(path: String) -> Result<(), String> {
    tauri_plugin_opener::reveal_item_in_dir(&path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn rename_file(
    state: State<'_, AppState>,
    path: String,
    new_name: String,
) -> Result<BookInfo, String> {
    let old_path = Path::new(&path);
    if !old_path.exists() {
        return Err("文件不存在".into());
    }

    let new_path = old_path
        .parent()
        .ok_or("无法获取父目录")?
        .join(&new_name);

    if new_path.exists() {
        return Err("目标文件名已存在".into());
    }

    fs::rename(old_path, &new_path).map_err(|e| e.to_string())?;

    let metadata = fs::metadata(&new_path).map_err(|e| e.to_string())?;
    let modified = metadata
        .modified()
        .ok()
        .map(|t| {
            let datetime: chrono::DateTime<chrono::Local> = t.into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
        .unwrap_or_default();

    let book = BookInfo {
        name: new_name.clone(),
        path: new_path.to_string_lossy().to_string(),
        size: metadata.len(),
        modified,
    };

    // 更新内存缓存
    if let Ok(mut books) = state.books.lock() {
        if let Some(b) = books.iter_mut().find(|b| b.path == path) {
            *b = book.clone();
        }
    }

    Ok(book)
}

// ========== 入口 ==========

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            books: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![
            get_directories,
            add_directory,
            remove_directory,
            scan_books,
            search_books,
            open_file,
            reveal_file,
            rename_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
