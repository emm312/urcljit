use eframe::App;

#[derive(Default)]
pub struct IODriver;

impl IODriver {
    pub fn new(cc: &eframe::CreationContext<'_>) -> IODriver {
        Default::default()
    }
}

impl App for IODriver {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("text_panel")
            .show(ctx, |ui| {
                ui.heading("Text IO");
                ui.separator();
                ui.monospace(crate::io::TEXT_IO_BUFFER.lock().unwrap().as_str());
            });
    }
}

pub fn ide_main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "urcljit IO",
        native_options,
        Box::new(|cc| Box::new(IODriver::new(cc))),
    )
    .unwrap();
}
