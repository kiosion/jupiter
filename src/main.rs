use eframe::{
    NativeOptions,
    egui::{
        CentralPanel,
        Color32,
        CtxRef,
        FontDefinitions,
        FontFamily,
        Hyperlink,
        Label,
        Layout,
        ScrollArea,
        Separator,
        Vec2
    },
    epi::App,
    run_native
};

use std::{borrow::Cow, iter::FromIterator};

const LIST_PADDING_TOP: f32 = 6.0;
const LIST_PADDING: f32 = 4.0;

const COLOR_WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const COLOR_CYAN: Color32 = Color32::from_rgb(0, 255, 255);

struct Jupiter {
    notes: Vec<NoteCardData>
}

struct NoteCardData {
    title: String,
    date: String,
    content: String,
    path: String
}

impl App for Jupiter {
    // One-time call for init of egui app
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        // N/A for now
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>
    ){
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        // Add any widgets/react to state changes
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Jupiter");

            // TODO
            self.render_note_cards(ui);
        });
    }

    fn name(&self) -> &str {
        "Jupiter"
    }
}

impl Jupiter {
    fn new() -> Jupiter {
        let iter = (0..10).map(|a| NoteCardData {
            title: format!("title{}", a),
            date: "n/a".to_string(),
            content: format!("content{}", a),
            path: format!("/home/kio/somewhere{}", a)
        });
        Jupiter {
            notes: Vec::from_iter(iter)
        }
    }

    fn configure_fonts(&self, ctx: &CtxRef) {
        // Create font def obj
        // load font
        // set size of various text styles
        // finally load font using ctx obj
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert("SourceSansPro".to_string(), Cow::Borrowed(include_bytes!("./fonts/SourceSansPro-Regular.ttf")));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Heading, (FontFamily::Proportional, 34.));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Body, (FontFamily::Proportional, 20.));
        font_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, "SourceSansPro".to_string());
        ctx.set_fonts(font_def);
    }

    fn render_note_cards(&self, ui: &mut eframe::egui::Ui) {
        ui.add_space(LIST_PADDING_TOP);

        ScrollArea::auto_sized().show(ui, |ui| {
            for a in &self.notes {
                ui.add_space(LIST_PADDING_TOP);
                
                let title = format!("> {}", a.title);
                ui.colored_label(COLOR_WHITE, title);
                ui.add_space(LIST_PADDING);
                ui.label(&a.date);
                
                let content = Label::new(&a.content).text_style(eframe::egui::TextStyle::Button);
                ui.add(content);

                // path / date (todo)
                ui.style_mut().visuals.hyperlink_color = COLOR_CYAN;
                ui.add_space(LIST_PADDING);
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Hyperlink::new(&a.path).text("Open <"));
                });
                
                // separator
                ui.add_space(LIST_PADDING);
                ui.add(Separator::default());
                // ui.label(&a.content);
            }
        });
    }
}

fn main() {
    let app = Jupiter::new();
    let mut win_options = NativeOptions::default();
    // Modify window opts
    win_options.initial_window_size = Some(Vec2::new(560., 740.));
    run_native(Box::new(app), win_options);
}
