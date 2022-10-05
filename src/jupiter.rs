use eframe::egui::{CtxRef, FontDefinitions, FontFamily, Label, Layout, Hyperlink, Separator, TopBottomPanel, Button};
use std::borrow::Cow;
use crate::consts::COLOR_CYAN_DARK;

use super::consts::{PADDING_MD, PADDING_SM, COLOR_LIGHT, COLOR_CYAN, COLOR_DARK};

pub struct JupiterConfig {
    pub dark_mode: bool
}

impl JupiterConfig {
    fn new() -> Self {
        Self {
            dark_mode: true
        }
    }
}

pub struct Jupiter {
    notes: Vec<NoteCardData>,
    pub config: JupiterConfig
}

struct NoteCardData {
    title: String,
    date: String,
    content: String,
    path: String
}

impl Jupiter {
    pub fn new() -> Jupiter {
        let iter = (0..10).map(|a| NoteCardData {
            title: format!("title{}", a),
            date: "n/a".to_string(),
            content: format!("content{}", a),
            path: format!("/home/kio/somewhere{}", a)
        });
        Jupiter {
            notes: Vec::from_iter(iter),
            config: JupiterConfig::new()
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {
        // Create font def obj, load font, set size of various text styles, finally load font using ctx obj
        let mut source_sans_def = FontDefinitions::default();
        source_sans_def.font_data.insert("SourceSansPro".to_string(), Cow::Borrowed(include_bytes!("./fonts/SourceSansPro-Regular.ttf")));
        source_sans_def.family_and_size.insert(eframe::egui::TextStyle::Body, (FontFamily::Proportional, 20.));
        source_sans_def.family_and_size.insert(eframe::egui::TextStyle::Small, (FontFamily::Proportional, 14.));
        source_sans_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, "SourceSansPro".to_string());
        ctx.set_fonts(source_sans_def);
        let mut ubuntu_def = FontDefinitions::default();
        ubuntu_def.font_data.insert("UbuntuBold".to_string(), Cow::Borrowed(include_bytes!("./fonts/Ubuntu-Bold.ttf")));
        ubuntu_def.family_and_size.insert(eframe::egui::TextStyle::Heading, (FontFamily::Proportional, 30.));
        ubuntu_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, "UbuntuBold".to_string());
        ctx.set_fonts(ubuntu_def);
    }

    pub fn render_note_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.notes {
            // Title
            let title = Label::new(format!("> {}", a.title));
            if self.config.dark_mode {
              ui.colored_label(COLOR_LIGHT, title);
            } else {
              ui.colored_label(COLOR_DARK, title);
            }
            // Date
            ui.add_space(PADDING_SM);
            ui.label(&a.date);
            // Desc (TODO: truncate)
            let content = Label::new(&a.content).text_style(eframe::egui::TextStyle::Button);
            ui.add(content);
            // Path
            if self.config.dark_mode {
              ui.style_mut().visuals.hyperlink_color = COLOR_CYAN;
            } else {
              ui.style_mut().visuals.hyperlink_color = COLOR_CYAN_DARK;
            }
            ui.add_space(PADDING_SM);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::new(&a.path).text("> Open").text_style(eframe::egui::TextStyle::Body));
            });
            // Separator
            ui.add_space(PADDING_SM);
            ui.add(Separator::default());
            ui.add_space(PADDING_MD);
        }
    }

    pub(crate) fn render_top_panel(&mut self, ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
      // Define topBottomPanel widget, add menu bar, then two layout widgets for logo on left / buttons on right
      TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(PADDING_MD);
        eframe::egui::menu::bar(ui, |ui| {
          ui.with_layout(Layout::left_to_right(), |ui| {
            ui.add(Label::new("Jupiter").text_style(eframe::egui::TextStyle::Body));
          });
          ui.with_layout(Layout::right_to_left(), |ui| {
            let close_button = ui.add(Button::new("❌").text_style(eframe::egui::TextStyle::Body));
            let refresh_button = ui.add(Button::new("🔄").text_style(eframe::egui::TextStyle::Body));
            let theme_button = ui.add(Button::new({
              if self.config.dark_mode {
                "🌙"
              } else {
                "☀"
              }
            }).text_style(eframe::egui::TextStyle::Body));

            if close_button.clicked() {
              frame.quit();
            }

            if theme_button.clicked() {
              self.config.dark_mode = !self.config.dark_mode;
            }
          });
        });
        ui.add_space(PADDING_MD);
      });
    }
}
