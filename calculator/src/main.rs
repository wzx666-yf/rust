use eframe::egui;
use meval::eval_str;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My Calculator App",
        options,
        Box::new(|_cc| Ok(Box::new(MyCalculator::default()) as Box<dyn eframe::App>)),
    );
}

#[derive(Default)]
struct MyCalculator {
    input: String,
    result: Option<f64>,
}

impl eframe::App for MyCalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Calculator");
            ui.add_space(10.0);

            // 使用 RichText 设置字体大小
            ui.add(
                egui::TextEdit::singleline(&mut self.input)
                    .font(egui::FontId::proportional(20.0))
                    .desired_width(f32::INFINITY),
            );

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                for button_label in ["1", "2", "3", "+"] {
                    if ui.button(button_label).clicked() {
                        self.input.push_str(button_label);
                    }
                    ui.add_space(5.0);
                }
            });

            ui.horizontal(|ui| {
                for button_label in ["4", "5", "6", "-"] {
                    if ui.button(button_label).clicked() {
                        self.input.push_str(button_label);
                    }
                    ui.add_space(5.0);
                }
            });

            ui.horizontal(|ui| {
                for button_label in ["7", "8", "9", "*"] {
                    if ui.button(button_label).clicked() {
                        self.input.push_str(button_label);
                    }
                    ui.add_space(5.0);
                }
            });

            ui.horizontal(|ui| {
                if ui.button("0").clicked() {
                    self.input.push('0');
                }
                ui.add_space(5.0);
                if ui.button(".").clicked() {
                    self.input.push('.');
                }
                ui.add_space(5.0);
                if ui.button("=").clicked() {
                    // 使用 meval 库来解析和计算表达式
                    self.result = eval_str(&self.input).ok();
                }
                ui.add_space(5.0);
                if ui.button("/").clicked() {
                    self.input.push('/');
                }
            });

            if let Some(result) = self.result {
                ui.add_space(10.0);
                // 使用 RichText 设置字体大小和颜色
                ui.label(
                    egui::RichText::new(format!("Result: {}", result))
                        .size(24.0)
                        .color(egui::Color32::GREEN),
                );
            }
        });
    }
}
