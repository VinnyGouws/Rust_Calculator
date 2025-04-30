use eframe::egui;
use log::info;
use log4rs;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    info!(target: "calculator", "In Main");
    // Make sure logs directory exists
    fs::create_dir_all("logs")?;
    
    // Initialize logging
    log4rs::init_file("log4rs.yaml", Default::default())?;
    
    let mut options = eframe::NativeOptions::default();
    options.viewport = egui::ViewportBuilder::default().with_inner_size([350.0, 350.0]);

    eframe::run_native(
        "Calculator",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )?;
    
    Ok(())
}

struct MyApp {
    input: String,
    result: f64,
}

impl MyApp {
    // Set the entry box
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        info!(target: "calculator", "In MyApp");
        Self {
            input: String::new(),
            result: 0.0,
        }
    }
    
    fn add_to_input(&mut self, character: &str) {
        info!(target: "calculator", "In add_to_input");
        self.input.push_str(character);
    }
    
    fn preprocess_expression(&self, expr: &str) -> String {
        info!(target: "calculator", "In preprocess_expression");
        let mut result = String::new();
        let chars: Vec<char> = expr.chars().collect();

        info!(target: "calculator", "Chars: {:?}", chars);
        
        for i in 0..chars.len() {
            result.push(chars[i]);

            // If current char is a number and next char is an opening parenthesis,
            // insert an explicit multiplication operator
            if i + 1 < chars.len() && 
               (chars[i].is_digit(10) || chars[i] == ')') && 
               chars[i+1] == '(' {
                result.push('*');
            }
            
            // Handle cases like )( - add multiplication between closing and opening parentheses
            if i + 1 < chars.len() && 
               chars[i] == ')' && 
               chars[i+1] == '(' {
                result.push('*');
            }
            
            // Handle cases like 3sin or 5cos (if we add trig functions later)
            // This would look similar to the number-parenthesis check above
        }

        info!(target: "calculator", "Result: {}", result);
        
        result
    }
    
    fn calculate(&mut self) {
        info!(target: "calculator", "In calculate");
        if !self.input.is_empty() {
            // Preprocess the expression to handle implicit multiplication
            info!(target: "calculator", "Expression: {}", self.input);
            let processed_expr = self.preprocess_expression(&self.input);
            
            info!(target: "calculator", "Processed expression: {}", processed_expr);
            // Using meval crate to evaluate the processed expression
            match meval::eval_str(&processed_expr) {
                Ok(result) => {
                    self.result = result;
                    self.input = self.result.to_string();
                    info!(target: "calculator", "Result: {}", self.result);
                },
                Err(e) => {
                    // Handle invalid expressions
                    self.input = "Error".to_string();
                    self.result = f64::NAN;
                    info!(target: "calculator", "Error: {}", e);
                }
            }
        } else {
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Display entry box at top
            ui.text_edit_singleline(&mut self.input);
            
            // Button layout
            let button_size = egui::vec2(60.0, 60.0);
            
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
                if ui.add(egui::Button::new("C").min_size(button_size)).clicked() {
                    self.input.clear();
                    self.result = 0.0;
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
                if ui.add(egui::Button::new("(").min_size(button_size)).clicked() {
                    self.add_to_input("(");
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
                if ui.add(egui::Button::new(")").min_size(button_size)).clicked() {
                    self.add_to_input(")");
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
                    self.calculate();
                }
            });
        });
    }
}