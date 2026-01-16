use eframe::egui;
use rand::Rng;

// Logic constants matching your genpass.py exactly
const CHARS_LOWER: &str = "abcdefghijkmnopqrstuvwxyz"; // Excludes 'l'
const CHARS_UPPER: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ"; // Excludes 'I', 'O'
const CHARS_DIGITS: &str = "23456789";                  // Excludes '0', '1'
const CHARS_SYMBOLS: &str = "!@#$%^&*(){}[]<>;,.?/~-_+=";

fn main() -> eframe::Result<()> {
    // Set up window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 300.0]) // Match your QSize(600, 300)
            .with_min_inner_size([400.0, 250.0]),
        ..Default::default()
    };

    // Run the app
    eframe::run_native(
        "Random Password Generator",
        options,
        Box::new(|_cc| Ok(Box::new(PasswordApp::default()))),
    )
}

struct PasswordApp {
    // Config state
    length: usize,
    use_lower: bool,
    use_upper: bool,
    use_digits: bool,
    use_symbols: bool,

    // UI state
    generated_password: String,
}

impl Default for PasswordApp {
    fn default() -> Self {
        let mut app = Self {
            length: 16,
            use_lower: true,
            use_upper: true,
            use_digits: true,
            use_symbols: true,
            generated_password: String::new(),
        };
        app.generate_password();
        app
    }
}

impl PasswordApp {
    fn generate_password(&mut self) {
        let mut chars = String::new();

        if self.use_lower {
            chars.push_str(CHARS_LOWER);
        }
        if self.use_upper {
            chars.push_str(CHARS_UPPER);
        }
        if self.use_digits {
            chars.push_str(CHARS_DIGITS);
        }
        if self.use_symbols {
            chars.push_str(CHARS_SYMBOLS);
        }

        if chars.is_empty() {
            self.generated_password = String::new();
            return;
        }

        let mut rng = rand::rng();
        let chars_vec: Vec<char> = chars.chars().collect();

        // Generate the random string
        self.generated_password = (0..self.length)
            .map(|_| {
                let idx = rng.random_range(0..chars_vec.len());
                chars_vec[idx]
            })
            .collect();
    }
}

impl eframe::App for PasswordApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Vertical layout is the default in CentralPanel

            // Add some spacing to center things visually
            ui.add_space(20.0);

            ui.allocate_ui(egui::vec2(ui.available_width(), 50.0), |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {

                    // [Button] - Placed on the Right
                    // We use a symbol for compactness, but "Copy" works too.
                    if ui.button("🗐").on_hover_text("Copy to clipboard").clicked() {
                        // This command copies text to the OS clipboard
                        ui.ctx().copy_text(self.generated_password.clone());
                    }

                    // [Password Field] - Fills remaining space
                    ui.add(
                        egui::TextEdit::singleline(&mut self.generated_password)
                            .font(egui::FontId::proportional(30.0))
                            .frame(false)
                            .horizontal_align(egui::Align::Center)
                            .interactive(true)
                            .desired_width(f32::INFINITY) // Fill remaining width
                    );
                });
            });

            ui.add_space(20.0);

            // We track changes and re-generate password when they occur
            let mut params_changed = false;

            // --- Length Input (Centered) ---
            ui.vertical_centered(|ui| {
                // We attach "Length: " as a prefix so it moves with the number
                if ui.add(
                    egui::DragValue::new(&mut self.length)
                        .prefix("Length: ")
                        .range(4..=128)
                        .speed(0.1)
                    ).changed()
                {
                    params_changed = true;
                }
            });

            ui.add_space(5.0);

            // --- Checkboxes ---
            ui.vertical(|ui| {
                // We add some padding to the left to make it look neat
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    if ui.checkbox(&mut self.use_lower, "a-z excluding l")
                            .changed() { params_changed = true; }
                    if ui.checkbox(&mut self.use_upper, "A-Z excluding I and O")
                            .changed() { params_changed = true; }
                    if ui.checkbox(&mut self.use_digits, "2-9")
                            .changed() { params_changed = true; }
                    if ui.checkbox(&mut self.use_symbols,
                                "!@#$%^&&*(){}[]<>;,.?/~-_+=")
                            .changed() { params_changed = true; }
                });
            });

            ui.add_space(20.0);

            // --- The Generate Button ---
            // ui.vertical_centered again to center the button
            ui.vertical_centered(|ui| {
                // We make the button a bit larger to match the feel
                let btn = egui::Button::new("Generate!")
                    .min_size(egui::vec2(100.0, 40.0));

                if ui.add(btn).clicked() {
                    params_changed = true;
                }
            });

            if params_changed {
                self.generate_password();
            }

        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Manually exit the process with code 0 (success).
        // This prevents the OS from attempting the faulty Wayland cleanup.
        std::process::exit(0);
    }
}