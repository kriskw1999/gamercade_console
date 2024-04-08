use eframe::egui;

mod app;
mod ips;
mod local_directory;
mod task_manager;
mod view;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Gamercade Platform",
        options,
        Box::new(|_cc| Box::<app::App>::default()),
    )
}
