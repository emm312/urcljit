use eframe::App;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[serde(default)]
pub struct Ide {
    current_file: String,
    root_folder: String,
}

impl Ide {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Ide {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl App for Ide {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self)
    }
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("file tree").default_width(200.0).show(ctx, |ui| {
            if ui.button("Open Folder").clicked() {
                let folder = rfd::FileDialog::new()
                    .pick_folder();
                self.root_folder = folder.unwrap().to_str().unwrap().to_string();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {

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
        "urcljit",
        native_options,
        Box::new(|cc| Box::new(Ide::new(cc))),
    ).unwrap();    
}