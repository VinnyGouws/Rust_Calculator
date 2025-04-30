use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    input: String,
    result: f64,  // Renamed from 'output' to 'result' to match usage
}

impl MyApp {
    // Add the missing 'new' method
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            input: String::new(),
            result: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Enter expression:");
            ui.text_edit_singleline(&mut self.input);

            if ui.button("Evaluate").clicked() {
                self.result = meval::eval_str(&self.input).unwrap_or(f64::NAN);
            }

            ui.label(format!("Result: {}", self.result));
        });
    }
}