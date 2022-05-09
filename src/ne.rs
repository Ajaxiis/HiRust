use {egui,epi};


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
     });