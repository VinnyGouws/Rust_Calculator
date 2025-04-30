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
    result: f64,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            input: String::new(),
            result: 0.0,
        }
    }
    
    fn add_to_input(&mut self, character: &str) {
        self.input.push_str(character);
    }
    
    fn evaluate(&mut self) {
        if !self.input.is_empty() {
            self.result = meval::eval_str(&self.input).unwrap_or(f64::NAN);
            self.input = self.result.to_string();
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Display entry box at top
            ui.text_edit_singleline(&mut self.input);
            
            // Button layout
            let button_size = egui::vec2(40.0, 40.0);
            
            // Row 1: 7 8 9 +
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("7").min_size(button_size)).clicked() {
                    self.add_to_input("7");
                }
                if ui.add(egui::Button::new("8").min_size(button_size)).clicked() {
                    self.add_to_input("8");
                }
                if ui.add(egui::Button::new("9").min_size(button_size)).clicked() {
                    self.add_to_input("9");
                }
                if ui.add(egui::Button::new("+").min_size(button_size)).clicked() {
                    self.add_to_input("+");
                }
            });
            
            // Row 2: 4 5 6 -
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("4").min_size(button_size)).clicked() {
                    self.add_to_input("4");
                }
                if ui.add(egui::Button::new("5").min_size(button_size)).clicked() {
                    self.add_to_input("5");
                }
                if ui.add(egui::Button::new("6").min_size(button_size)).clicked() {
                    self.add_to_input("6");
                }
                if ui.add(egui::Button::new("-").min_size(button_size)).clicked() {
                    self.add_to_input("-");
                }
            });
            
            // Row 3: 1 2 3 x
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("1").min_size(button_size)).clicked() {
                    self.add_to_input("1");
                }
                if ui.add(egui::Button::new("2").min_size(button_size)).clicked() {
                    self.add_to_input("2");
                }
                if ui.add(egui::Button::new("3").min_size(button_size)).clicked() {
                    self.add_to_input("3");
                }
                if ui.add(egui::Button::new("×").min_size(button_size)).clicked() {
                    self.add_to_input("*");
                }
            });
            
            // Row 4: 0 . ÷ =
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("0").min_size(button_size)).clicked() {
                    self.add_to_input("0");
                }
                if ui.add(egui::Button::new(".").min_size(button_size)).clicked() {
                    self.add_to_input(".");
                }
                if ui.add(egui::Button::new("÷").min_size(button_size)).clicked() {
                    self.add_to_input("/");
                }
                if ui.add(egui::Button::new("=").min_size(button_size)).clicked() {
                    self.evaluate();
                }
            });
        });
    }
}