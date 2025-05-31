use readerlib::{self, book::Book, constr};

use eframe::egui;

fn main() -> eframe::Result {
    println!("Hello, world!");
    let book = readerlib::constr::construct("/home/sk/Documents/projects/reader-rust/way_of_kings.epub").unwrap();
    println!("{}", book);

    println!("\n\ndone\n\n");

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Rust Reader",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<ReaderApp>::default())
        }),
    )
}

struct ReaderApp {
    book: Option<Book>,
}

impl Default for ReaderApp {
    fn default() -> Self {
        let cla = std::env::args().nth(1).unwrap_or("/home/sk/Documents/projects/reader-rust/way_of_kings.epub".to_string());
        Self {
            book: constr::construct(&cla),
        }
    }
}

impl eframe::App for ReaderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.book.as_ref().unwrap().to_string())
                    .labelled_by(name_label.id);
            });
            ui.label(format!("Book Manifest: {}", &self.book.as_ref().unwrap().to_string()));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}