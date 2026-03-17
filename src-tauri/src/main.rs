#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use xcap::Monitor;
use base64::{engine::general_purpose, Engine as _};
use tauri::{Manager, menu::{Menu, MenuItem}, tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState}};

// Put your real Gemini API token here!
const API_TOKEN: &str = "YOUR_GEMINI_KEY_HERE"; 

const GOOGLE_ENDPOINTS: &[&str] = &[
    "v1beta/models/gemini-2.5-flash",
    "v1/models/gemini-2.5-flash",
    "v1beta/models/gemini-flash-latest"
];

#[tauri::command]
async fn check_models() -> Result<bool, String> { Ok(true) }

async fn ask_the_ai_brain(image_bytes: Vec<u8>) -> Result<String, String> {
    if API_TOKEN == "YOUR_GEMINI_KEY_HERE" || API_TOKEN.is_empty() {
        return Ok("Grrr! Please add an API key in main.rs so I can think!".to_string());
    }

    let b64_img = general_purpose::STANDARD.encode(&image_bytes);
    let client = reqwest::Client::new();
    let mut last_error_text = String::new();
    let mut last_status_code = 0;

    let payload = serde_json::json!({
        "contents": [{
            "parts": [
                {"text": "You are an expert AI detection system. Analyze this screenshot. Look closely at the primary photo/video being displayed. Is it AI-generated, or is it a genuine photograph/video of real people (like a DJ set or IRL event)? Look at lighting, hands, and contextual clues. Respond ONLY with a valid JSON format: {\"is_fake\": true/false, \"confidence\": 95}"},
                {
                    "inline_data": {
                        "mime_type": "image/jpeg",
                        "data": b64_img
                    }
                }
            ]
        }]
    });

    for &endpoint in GOOGLE_ENDPOINTS.iter() {
        let url = format!("https://generativelanguage.googleapis.com/{}:generateContent?key={}", endpoint, API_TOKEN.trim());

        let res = match client.post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await 
        {
            Ok(r) => r,
            Err(e) => {
                last_error_text = format!("Network crash: {}", e);
                continue;
            }
        };

        let status = res.status().as_u16();
        let raw_text = res.text().await.unwrap_or_default();

        if status == 200 {
            let json_response: serde_json::Value = serde_json::from_str(&raw_text).unwrap_or_default();
            
            let ai_text_reply = json_response.get("candidates")
                .and_then(|c| c.get(0))
                .and_then(|c| c.get("content"))
                .and_then(|c| c.get("parts"))
                .and_then(|c| c.get(0))
                .and_then(|c| c.get("text"))
                .and_then(|t| t.as_str())
                .unwrap_or("{}");

            let clean_json_string = ai_text_reply.replace("```json", "").replace("```", "").trim().to_string();
            let spotty_math: serde_json::Value = serde_json::from_str(&clean_json_string).unwrap_or_default();

            let is_fake = spotty_math.get("is_fake").and_then(|f| f.as_bool()).unwrap_or(false);
            let confidence = if let Some(c) = spotty_math.get("confidence") {
                if let Some(num) = c.as_f64() { num as u32 } else { 99 }
            } else { 99 };

            if is_fake {
                return Ok(format!("Arf! {}% AI GENERATED!", confidence));
            } else {
                return Ok(format!("Woof! {}% Real genuine media!", confidence));
            }
        }

        last_status_code = status;
        let clean_err = raw_text.replace("\n", "").replace("  ", " ");
        last_error_text = format!("Error {} on {}: {}", status, endpoint, clean_err);
    }

    if last_status_code == 404 {
        let list_url = format!("https://generativelanguage.googleapis.com/v1beta/models?key={}", API_TOKEN.trim());
        if let Ok(list_res) = client.get(&list_url).send().await {
            let list_text = list_res.text().await.unwrap_or_default();
            let list_json: serde_json::Value = serde_json::from_str(&list_text).unwrap_or_default();
            
            let mut available_models = Vec::new();
            if let Some(models) = list_json.get("models").and_then(|m| m.as_array()) {
                for m in models {
                    if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                        if name.contains("gemini") {
                            let clean_name = name.replace("models/", "");
                            available_models.push(clean_name);
                        }
                    }
                }
            }
            if !available_models.is_empty() {
                return Ok(format!("Bark! Google rejected the new model! Your key has: {:?}", available_models));
            }
        }
    }

    let short_err = last_error_text.chars().take(220).collect::<String>();
    Ok(format!("Bark! {}", short_err))
}

#[tauri::command]
async fn analyze_screen() -> Result<String, String> {
    let image_bytes = {
        let monitors = Monitor::all().map_err(|e| e.to_string())?;
        let primary = monitors.first().ok_or("No monitor found")?;
        
        let rgba_image = primary.capture_image().map_err(|e| e.to_string())?;
        
        let dynamic_img = image::DynamicImage::ImageRgba8(rgba_image);
        let resized = dynamic_img.resize_exact(800, 800, image::imageops::FilterType::Triangle);
        let rgb_img = image::DynamicImage::ImageRgb8(resized.into_rgb8());
        
        let mut buffer = std::io::Cursor::new(Vec::new());
        rgb_img.write_to(&mut buffer, image::ImageFormat::Jpeg).map_err(|e| e.to_string())?;
        
        buffer.into_inner()
    };

    ask_the_ai_brain(image_bytes).await
}

#[tauri::command]
async fn analyze_video() -> Result<String, String> {
    let image_bytes = {
        let monitors = Monitor::all().map_err(|e| e.to_string())?;
        let primary = monitors.first().ok_or("No monitor found")?;
        
        let frame1 = primary.capture_image().map_err(|e| e.to_string())?;
        std::thread::sleep(std::time::Duration::from_millis(500));
        let frame2 = primary.capture_image().map_err(|e| e.to_string())?;
        
        let pixels1 = frame1.into_raw();
        let pixels2 = frame2.into_raw();
        let mut differences = 0;
        let total_pixels = pixels1.len();
        
        for i in (0..total_pixels).step_by(4) {
            if pixels1[i] != pixels2[i] { differences += 1; }
        }
        
        let diff_percent = (differences as f64 / (total_pixels / 4) as f64) * 100.0;
        if diff_percent < 0.5 {
            return Err("No moving video detected!".to_string());
        }
        
        let final_frame = primary.capture_image().map_err(|e| e.to_string())?;
        
        let dynamic_img = image::DynamicImage::ImageRgba8(final_frame);
        let resized = dynamic_img.resize_exact(800, 800, image::imageops::FilterType::Triangle);
        let rgb_img = image::DynamicImage::ImageRgb8(resized.into_rgb8());
        
        let mut buffer = std::io::Cursor::new(Vec::new());
        rgb_img.write_to(&mut buffer, image::ImageFormat::Jpeg).map_err(|e| e.to_string())?;
        
        buffer.into_inner()
    };
    
    ask_the_ai_brain(image_bytes).await
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit Spotty", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let icon = app.default_window_icon().cloned().unwrap();
            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .tooltip("Spotty - AI Sniffer")
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_models, 
            analyze_screen,
            analyze_video
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}