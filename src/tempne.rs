#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WindowOptions {
    title: String,
    title_bar: bool,
    closable: bool,
    collapsible: bool,
    resizable: bool,
    scroll2: [bool; 2],
    disabled_time: f64,

    anchored: bool,
    anchor: egui::Align2,
    anchor_offset: egui::Vec2,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "🗖 Window Options".to_owned(),
            title_bar: true,
            closable: true,
            collapsible: true,
            resizable: true,
            scroll2: [true; 2],
            disabled_time: f64::NEG_INFINITY,
            anchored: false,
            anchor: egui::Align2::RIGHT_TOP,
            anchor_offset: egui::Vec2::ZERO,
        }
    }
}

impl super::Demo for WindowOptions {
    fn name(&self) -> &'static str {
        "🗖 Window Options"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let Self {
            title,
            title_bar,
            closable,
            collapsible,
            resizable,
            scroll2,
            disabled_time,
            anchored,
            anchor,
            anchor_offset,
        } = self.clone();

        let enabled = ctx.input().time - disabled_time > 2.0;
        if !enabled {
            ctx.request_repaint();
        }

        use super::View as _;
        let mut window = egui::Window::new(title)
            .id(egui::Id::new("demo_window_options")) // required since we change the title
            .resizable(resizable)
            .collapsible(collapsible)
            .title_bar(title_bar)
            .scroll2(scroll2)
            .enabled(enabled);
        if closable {
            window = window.open(open);
        }
        if anchored {
            window = window.anchor(anchor, anchor_offset);
        }
        window.show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for WindowOptions {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            title,
            title_bar,
            closable,
            collapsible,
            resizable,
            scroll2,
            disabled_time: _,
            anchored,
            anchor,
            anchor_offset,
        } = self;
        ui.horizontal(|ui| {
            ui.label("title:");
            ui.text_edit_singleline(title);
        });

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.checkbox(title_bar, "title_bar");
                    ui.checkbox(closable, "closable");
                    ui.checkbox(collapsible, "collapsible");
                    ui.checkbox(resizable, "resizable");
                    ui.checkbox(&mut scroll2[0], "hscroll");
                    ui.checkbox(&mut scroll2[1], "vscroll");
                });
            });
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.checkbox(anchored, "anchored");
                    ui.set_enabled(*anchored);
                    ui.horizontal(|ui| {
                        ui.label("x:");
                        ui.selectable_value(&mut anchor[0], egui::Align::LEFT, "Left");
                        ui.selectable_value(&mut anchor[0], egui::Align::Center, "Center");
                        ui.selectable_value(&mut anchor[0], egui::Align::RIGHT, "Right");
                    });
                    ui.horizontal(|ui| {
                        ui.label("y:");
                        ui.selectable_value(&mut anchor[1], egui::Align::TOP, "Top");
                        ui.selectable_value(&mut anchor[1], egui::Align::Center, "Center");
                        ui.selectable_value(&mut anchor[1], egui::Align::BOTTOM, "Bottom");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Offset:");
                        ui.add(egui::DragValue::new(&mut anchor_offset.x));
                        ui.add(egui::DragValue::new(&mut anchor_offset.y));
                    });
                });
            });
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Disable for 2 seconds").clicked() {
                self.disabled_time = ui.input().time;
            }
            egui::reset_button(ui, self);
           // ui.add(crate::__egui_github_link_file!());
        });
    }
}

/*
//-----------------
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

     */