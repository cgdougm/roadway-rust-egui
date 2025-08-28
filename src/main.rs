use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Hello World - eGUI",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, eGUI!");
            
            ui.horizontal(|ui| {
                ui.label("Your name:");
                ui.text_edit_singleline(&mut self.name);
            });
            
            ui.horizontal(|ui| {
                ui.label("Your age:");
                ui.add(egui::DragValue::new(&mut self.age));
            });
            
            ui.label(format!("Hello, {}! You are {} years old.", self.name, self.age));
            
            ui.separator();
            
            if ui.button("Click me!").clicked() {
                ui.label("Button was clicked!");
            }
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add(egui::github_link_file_line!(
                    "https://github.com/emilk/egui/blob/master/examples/",
                    "Source code."
                ));
            });
        });
    }
}