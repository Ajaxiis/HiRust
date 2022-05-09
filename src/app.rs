use eframe::{egui, epi};
use egui::RichText;
//use cmd_lib;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    component: String,
    version: String,
    authors: String,
    edition: String,
    rustver: String,
    methods: String,
    parametres: String,
    scode: String,
    rscode: String,
    wstatus: String,
    prj:String,
    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: i32,
    
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "".to_owned(),
            component: "Component".to_owned(),
            version: "0.0.1".to_owned(),
            authors: "Author".to_owned(),
            edition: "2022".to_owned(),
            rustver: "1.56".to_owned(),
            methods: "array of methods by string ".to_owned(),
            parametres: "array of parameters by string ".to_owned(),
            scode: "algorithm code ".to_owned(),
            rscode: "Rust code".to_owned(),
            wstatus: "help".to_string(),
            value: 0,
            prj:"new project".to_owned(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "IGRA-конструктор апликаций"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            label,
            component,
            version,
            authors,
            edition,
            rustver,
            methods,
            parametres,
            scode,
            rscode,
            wstatus,
            value,
            prj,
        } = self;
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.horizontal(|ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);
                    ui.menu_button("Файл", |ui| { *value = 1;
                        if ui.button("Новый проект").clicked() { 
                        *wstatus = "newprj".to_string();
                        };
                        if ui.button("Новый элемент").clicked() { 
                            *wstatus = "nelement".to_string();
                            }
                    });

                    ui.menu_button("Правка", |ui| {
                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    });
                    ui.menu_button("Вид", |ui| {
                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    });
                    ui.menu_button("Настройки", |ui| {
                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    });
                    ui.menu_button("Помощь", |ui| {
                        *wstatus = "help".to_string();

                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    });
                    ui.separator();
                    ui.menu_button("Сменить тему", |ui| {
                        egui::widgets::global_dark_light_mode_switch(ui);
                    });
                });
            });
            ui.horizontal(|ui| {
                if ui
                    .button(RichText::new("🖹").size(32.0))
                    .on_hover_text("Новый проект")
                    .clicked()
                { *wstatus = "newprj".to_string();};
                if ui
                    .button(RichText::new("📂").size(32.0))
                    .on_hover_text("Открыть")
                    .clicked()
                {};
                if ui
                    .button(RichText::new("💾").size(32.0))
                    .on_hover_text("Сохранить")
                    .clicked()
                {};
                if ui
                    .button(RichText::new("🖴").size(32.0))
                    .on_hover_text("Сохранить как")
                    .clicked()
                {};

                ui.separator();
                if ui
                    .button(RichText::new("🗐").size(32.0))
                    .on_hover_text("Копировать")
                    .clicked()
                {};
                if ui
                    .button(RichText::new("📚").size(32.0))
                    .on_hover_text("Вставить")
                    .clicked()
                {};
                if ui
                    .button(RichText::new("🗑").size(32.0))
                    .on_hover_text("Удалить")
                    .clicked()
                {};
                ui.separator();

                if ui
                    .button(RichText::new("🎨").size(32.0))
                    .on_hover_text("Схема")
                    .clicked()
                {  *wstatus = "sha".to_string();};
                if ui
                    .button(RichText::new("🔙").size(32.0))
                    .on_hover_text("Наружу")
                    .clicked()
                {  *wstatus = "back".to_string();};
                if ui
                    .button(RichText::new("🔚").size(32.0))
                    .on_hover_text("В глубь")
                    .clicked()
                {  *wstatus = "multy".to_string();};
                ui.separator();
                if ui
                    .button(RichText::new("👁").size(32.0))
                    .on_hover_text("Псевдокод")
                    .clicked()
                {  *wstatus = "p-cod".to_string();};
                if ui
                    .button(RichText::new("👀").size(32.0))
                    .on_hover_text("Rust-код")
                    .clicked()
                {*wstatus = "R-cod".to_string();};
                if ui
                    .button(RichText::new("🚛").size(32.0))
                    .on_hover_text("Cargo-код")
                    .clicked()
                { *wstatus = "C-cod".to_string();};
                ui.separator();
                if ui
                    .button(RichText::new("▶").size(32.0))
                    .on_hover_text("Запустить")
                    .clicked()
                {};
                if ui
                    .button(RichText::new("☑").size(32.0))
                    .on_hover_text("Компилировать")
                    .clicked()
                {};
                if ui
                    .button(RichText::new("✈").size(32.0))
                    .on_hover_text("Cargo-format")
                    .clicked()
                {};
            });
        });

        // }; //- end FN!
        egui::SidePanel::left("elements_panel").show(ctx, |ui| {
            ui.heading("Элементы");
            // сюда заполняем элементы
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.heading("Параметры");

            
                // сюда в цикле пишем все параметры элемента
           

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            // окно схема -алгоритм \ исходный код \ код схемы

            //===============================================
           if wstatus == "nelement" {
            egui::Window::new("New Component").title_bar(true).collapsible(true).show(ctx, |ui| {
                ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Packeth: ");
                    ui.text_edit_singleline(label)
                        .on_hover_text("Пакет - это библиотека  из https://crates.io");
                });
                ui.horizontal(|ui| {
                    ui.label("Component: ");
                    ui.text_edit_singleline(component).on_hover_text(
                        "Название компонента (обычно виджет импортируемой библиотеки",
                    );
                });
            });
                    ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Version");
                    ui.text_edit_singleline(version)
                        .on_hover_text("версия компонента");
                });
                        ui.horizontal(|ui| {
                    ui.label("Authors: ");
                    ui.text_edit_singleline(authors)
                        .on_hover_text("Авторы компонента");
                });
            });
                    ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Edition: ");
                    ui.text_edit_singleline(edition)
                        .on_hover_text("Дата последней модификации");
                });
                ui.horizontal(|ui| {
                    ui.label("Rust Ver: ");
                    ui.text_edit_singleline(rustver)
                        .on_hover_text("Минимальная версия Раст для работы");
                });
            });
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Methods: ");
                    ui.text_edit_multiline(methods)
                        .on_hover_text("Перечислить мeтоды построчно ");
                });
                ui.vertical(|ui| {
                    ui.label("Parameters: ");
                    ui.text_edit_multiline(parametres)
                        .on_hover_text("Параметры перечислить построчно (Если есть опции в параметре, то через \\|");
                });
            });
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("S-Code: ");
                    ui.text_edit_multiline(scode)
                        .on_hover_text("Алгоритмический код");
                });
                ui.vertical(|ui| {
                    ui.label("Rust Code: ");
                    ui.text_edit_multiline(rscode)
                        .on_hover_text("Конечный код для компиляции");
                });
            });
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
            ui.horizontal(|ui| {
            if ui.button("Сохранить").on_hover_text("Сохранить изменения").clicked(){ };
            if ui.button("Закрыть").on_hover_text("не сохранять изменения").clicked(){   };

            });  
        });
                 }); //-> end window
                };
                if wstatus =="newprj" { 
                    egui::Window::new("New project").title_bar(true).collapsible(true).show(ctx, |ui| { 
                    ui.label("Название нового проекта: ");
                    ui.text_edit_singleline(prj)
                        .on_hover_text("Создаст директорию с новым именем проекта в папке текущего пользователя");
                    if ui.button("OK").clicked() { 

                       
                       // use std::process::Command;
                       // Command::new("cargo").arg ("new").arg(label);
                    };
                 });  
                };
                if wstatus == "help"{
                    _ui.label("read.me");
                };
        }); //-> end panel
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            //окно дебага \ консоль команд \ помощь по компоненту
            egui::warn_if_debug_build(ui);
        });
        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
