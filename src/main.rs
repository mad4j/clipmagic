#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod config;

#[cfg(not(target_os = "linux"))]
use std::{cell::RefCell, rc::Rc};

use anyhow::Result;

use eframe::egui;
use enigo::{Enigo, Keyboard, Settings};
use livesplit_hotkey::{Hook, KeyCode, Modifiers};
use tray_icon::TrayIconBuilder;

use clipboard_win::{formats, Clipboard, Setter};

const SAMPLE: &str = "MY loli sample ^^";

fn main() -> Result<()> {
    env_logger::init();

    let _config = config::init_configuration();

    let hook = Hook::new().unwrap();

    hook.register(
        KeyCode::Digit1.with_modifiers(Modifiers::ALT | Modifiers::SHIFT | Modifiers::CONTROL),
        || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            enigo.text(SAMPLE).unwrap();

            #[cfg(windows)]
            {
                let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
                formats::Unicode
                    .write_clipboard(&SAMPLE)
                    .expect("Write sample");
            }
        },
    )
    .unwrap();

    hook.register(
        KeyCode::Digit2.with_modifiers(Modifiers::ALT | Modifiers::SHIFT | Modifiers::CONTROL),
        || {
            let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            formats::Unicode
                .write_clipboard(&"anohter example")
                .expect("Write sample");

            enigo.text("anohter example").unwrap();
        },
    )
    .unwrap();

    run_app().unwrap();

    Ok(())
}

fn run_app() -> Result<(), eframe::Error> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.ico");
    let icon = load_icon(std::path::Path::new(path));

    // Since egui uses winit under the hood and doesn't use gtk on Linux, and we need gtk for
    // the tray icon to show up, we need to spawn a thread
    // where we initialize gtk and create the tray_icon
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        use tray_icon::menu::Menu;

        gtk::init().unwrap();
        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(Menu::new()))
            .with_icon(icon)
            .build()
            .unwrap();

        gtk::main();
    });

    #[cfg(not(target_os = "linux"))]
    let mut _tray_icon = Rc::new(RefCell::new(None));
    #[cfg(not(target_os = "linux"))]
    let tray_c = _tray_icon.clone();

    eframe::run_native(
        "My egui App",
        eframe::NativeOptions::default(),
        Box::new(move |_cc| {
            #[cfg(not(target_os = "linux"))]
            {
                tray_c
                    .borrow_mut()
                    .replace(TrayIconBuilder::new().with_icon(icon).build().unwrap());
            }
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use tray_icon::TrayIconEvent;

        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
            println!("tray event: {event:?}");
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
