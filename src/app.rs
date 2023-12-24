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
        
    }
}