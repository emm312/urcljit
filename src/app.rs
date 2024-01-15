use eframe::App;

use crate::io;

#[derive(Default)]
pub struct IODriver {
    text_io_buffer: String,
}

impl IODriver {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> IODriver {
        Default::default()
    }
}

impl App for IODriver {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("text_panel").show(ctx, |ui| {
            ui.heading("Text IO");
            ui.separator();
            egui::ScrollArea::vertical().min_scrolled_width(128.).show(ui, |ui| {
                ui.monospace(self.text_io_buffer.as_str());
            });
        });

        if let Ok(buffer) = io::TEXT_IO_BUFFER.try_read() {
            self.text_io_buffer = buffer.to_string();
        }
    }
}

pub fn ide_main() {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "urcljit IO",
        native_options,
        Box::new(|cc| Box::new(IODriver::new(cc))),
    )
    .unwrap();
}
