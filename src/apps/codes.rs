use crate::apps::{CheatCodeApp, CodeStruct, Region};
use crate::helpers::custom_widget::toggle_ui;
use crate::helpers::layouter;
use crate::{Align, View};
use egui::{Context, Layout, ScrollArea, TextEdit, Ui, FontSelection};
use egui_extras::{Size, TableBuilder};

impl Default for CheatCodeApp {
    fn default() -> Self {
        Self {
            enabled: false,
            selected_code: 0,
            selected_region: Region::NTSC,
            codes: Box::from(vec![CodeStruct::default()]),
            vertical_scroll_offset: None,
        }
    }
}

impl View for CheatCodeApp {
    fn name(&self) -> &'static str {
        "Cheat Code Manager"
    }

    fn ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("cheat_code_manager").resizable(false).show(ctx, |ui| {
            ui.heading(self.name());
            ui.horizontal(|ui| {
                toggle_ui(ui, &mut self.enabled);
                ui.label("Enable")
            })
            .response
            .on_hover_text("Enable cheat codes and implement them");
            ui.separator();
            ui.add_enabled_ui(self.enabled, |ui| {
                ui.group(|ui| {
                    ui.add(TextEdit::singleline(&mut self.codes[self.selected_code].name).hint_text("Enter name of code"));
                    ui.horizontal_wrapped(|ui| {
                        ui.selectable_value(&mut self.selected_region, Region::NTSC, "NTSC")
                            .on_hover_text("Toggle to allow entering code for NTSC-U");
                        ui.selectable_value(&mut self.selected_region, Region::PAL, "PAL")
                            .on_hover_text("Toggle to allow entering code for PAL");
                        ui.selectable_value(&mut self.selected_region, Region::JAP, "JAP")
                            .on_hover_text("Toggle to allow entering code for NTSC-J");
                        ui.selectable_value(&mut self.selected_region, Region::KOR, "KOR")
                            .on_hover_text("Toggle to allow entering code for NTSC-K");
                    });
                    ScrollArea::vertical().id_source("code_editor_area").max_height(230.0).show(ui, |ui| {
                        ui.add(
                            TextEdit::multiline({
                                match self.selected_region {
                                    Region::NTSC => &mut self.codes[self.selected_code].code_ntsc,
                                    Region::PAL => &mut self.codes[self.selected_code].code_pal,
                                    Region::JAP => &mut self.codes[self.selected_code].code_jp,
                                    Region::KOR => &mut self.codes[self.selected_code].code_kor,
                                }
                            })
                            .desired_rows(16)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .hint_text("04XXXXXX 00000000\r\nC2XXXXXX 00000002\r\n00000000 00000000\r\n60000000 00000000"),
                        )
                    });
                });
                ui.horizontal(|ui| {
                    if ui.button("New").on_hover_text("Add new entry in code list").clicked() {
                        println!("Code:New");
                        self.codes.push(CodeStruct::default())
                    }
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if ui.button("Delete").on_hover_text("Remove the selected entry from list").clicked() {
                            println!("Code:Remove");
                            if self.codes.len() <= 1 {
                                return;
                            }
                            let selected = self.selected_code;
                            self.selected_code = if selected <= 0 { self.selected_code } else { self.selected_code - 1 };
                            self.codes.remove(selected);
                        }
                    })
                });
                self.gen_tables(ui);
            })
        });
    }
}

impl CheatCodeApp {
    fn gen_tables(&mut self, ui: &mut Ui) {
        let table = TableBuilder::new(ui)
            .striped(true)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Size::remainder().at_least(60.0));

        table.body(|mut body| {
            for code_num in 0..self.codes.len() {
                body.row(16.0, |mut row| {
                    row.col(|ui| {
                        if ui
                            .add_sized(
                                ui.available_size(),
                                egui::SelectableLabel::new(code_num == self.selected_code, &self.codes[code_num].name),
                            )
                            .clicked()
                        {
                            self.selected_code = code_num;
                        }
                    });
                })
            }
        })
    }
}
