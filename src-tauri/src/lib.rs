use font_enumeration::{Collection, Font, Style};
use serde::Serialize;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[derive(Serialize)]
struct SerializableFont {
    family_name: String,
    font_name: String,
    path: String,
    style: String,
    weight: f32,
    stretch: f32,
}

impl From<&Font> for SerializableFont {
    fn from(font: &Font) -> Self {
        Self {
            family_name: font.family_name.clone(),
            font_name: font.font_name.clone(),
            path: font.path.to_string_lossy().into_owned(),
            style: match &font.style {
                Style::Normal => "Normal".to_string(),
                Style::Italic => "Italic".to_string(),
                Style::Oblique(angle) => match angle {
                    Some(angle) => format!("Oblique ({:?}°)", angle),
                    None => "Oblique".to_string(),
                },
            },
            weight: font.weight.value(),
            stretch: font.stretch.value(),
        }
    }
}

#[tauri::command]
fn list_fonts() -> Vec<SerializableFont> {
    let font_collection = Collection::new().unwrap();
    font_collection
        .all()
        .map(SerializableFont::from)
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_fonts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
