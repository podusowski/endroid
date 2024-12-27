use eframe::egui;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(
    app: egui_winit::winit::platform::android::activity::AndroidApp,
) -> Result<(), Box<dyn std::error::Error>> {
    use eframe::{NativeOptions, Renderer};
    use egui_winit::winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("walkers")
            .with_max_level(log::LevelFilter::Info),
    );
    let mut options = NativeOptions::default();
    options.renderer = Renderer::Wgpu;
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));
    eframe::run_native(
        "Walkers",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::default()))),
    )?;

    Ok(())
}

struct MyApp {
    name: String,
    error: Option<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            error: None,
        }
    }
}

impl MyApp {
    fn unwrap_or_show_error<T>(&mut self, result: Result<T, arboard::Error>) -> Option<T> {
        match result {
            Ok(value) => {
                self.error = None;
                Some(value)
            }
            Err(error) => {
                self.error = Some(error.to_string());
                None
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui on Android");

            ui.label("Clipboard");
            ui.text_edit_singleline(&mut self.name);

            if ui.button("clear").clicked() {
                self.name.clear();
            }

            if ui.button("foo").clicked() {
                self.name = "foo".to_owned();
            }

            if ui.button("bar").clicked() {
                self.name = "bar".to_owned();
            }

            if ui.button("copy to clipboard").clicked() {
                self.unwrap_or_show_error(
                    arboard::Clipboard::new()
                        .unwrap()
                        .set_text(self.name.clone()),
                );
            }

            if ui.button("copy from clipboard").clicked() {
                if let Some(value) =
                    self.unwrap_or_show_error(arboard::Clipboard::new().unwrap().get_text())
                {
                    self.name = value;
                }
            }

            if let Some(error) = &self.error {
                ui.colored_label(egui::Color32::RED, error);
            }
        });
    }
}
