# 🐶 Spotty AI Detector

![Spotty Banner](https://img.shields.io/badge/Status-Active-success) ![Tauri Version](https://img.shields.io/badge/Tauri-v2.0-blue) ![Rust](https://img.shields.io/badge/Backend-Rust-orange) ![Svelte](https://img.shields.io/badge/Frontend-Svelte-red)

**Spotty** is a lightweight, frameless, and transparent native Windows desktop assistant designed to sniff out AI-generated content (Text, Images, and Video) directly from your screen. 

Instead of forcing users to copy, paste, or upload files to slow web detectors, Spotty lives natively on your desktop. Simply drag him over any suspicious student essay or image, click scan, and he uses **Google Gemini Vision** to contextually analyze the pixels on your screen and return a confident verdict.

---

## ✨ Features
- **👻 Transparent Overlay:** Built cleanly with Tauri v2, Spotty functions as an always-on-top, frameless widget with native OS dragging (`-webkit-app-region: drag`).
- **🧠 Multi-Modal AI Brain:** Integrates directly with the `gemini-1.5-flash` API to contextually analyze not just pixel artifacts, but the actual logical setting of a photo or essay.
- **🎥 Video Temporal Scanning:** Includes a custom Rust engine that captures sequential frames across 1.5 seconds to calculate pixel deltas, ensuring a video is actually playing before sending it to the AI.
- **⚡ Native Performance:** Compiles to a tiny, highly-optimized `.exe` using Rust. Uses `<15MB` of RAM while idling in the system tray.

---

## 🛠️ Tech Stack
* **Core Framework:** [Tauri v2](https://v2.tauri.app/)
* **Backend:** Rust (Crates: `xcap`, `reqwest`, `serde_json`, `image`)
* **Frontend UI:** Svelte + Vite
* **Styling:** Tailwind CSS (Custom keyframe animations for mascot)
* **AI Provider:** Google Generative AI (Gemini Vision API)

---

## 🚀 Getting Started (For Developers)

To run Spotty locally on your machine, you will need **Node.js**, **Rust**, and the **Microsoft C++ Build Tools** installed.

### 1. Clone the Repository
```bash
git clone https://github.com/KevyKode/Spotty-AI-Detector.git
cd Spotty-AI-Detector
npm install
