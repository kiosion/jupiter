use eframe::{NativeOptions, run_native};
use eframe::epi::App;
use eframe::egui::{CentralPanel, Label, Vec2, Ui, Separator, ScrollArea, TopBottomPanel, Hyperlink, CtxRef, Layout};

mod consts;
use consts::{PADDING_XL, PADDING_LG, PADDING_MD, COLOR_WHITE};

mod jupiter;
use jupiter::{Jupiter};

impl App for Jupiter {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>
    ){
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {

        if self.config.dark_mode {
            ctx.set_visuals(eframe::egui::Visuals::dark());
        } else {
            ctx.set_visuals(eframe::egui::Visuals::light());
        }

        self.render_top_panel(ctx, frame);

        // TODO: Central panel should become a left panel when item is opened. collapsable.
        CentralPanel::default().show(ctx, |ui| {
            // Main heading
            render_header(ui);
            ui.add_space(PADDING_LG);

            // Content (temp. just a hardcoded list)
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_note_cards(ui);
            });

            // Can get Ui handle from ctx
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "Jupiter"
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn save(&mut self, _storage: &mut dyn eframe::epi::Storage) {}

    fn on_exit(&mut self) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self) -> eframe::egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        eframe::egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(PADDING_LG);

            ui.add(Label::new("Made with <3 by kio").text_style(eframe::egui::TextStyle::Small));
            ui.style_mut().visuals.hyperlink_color = COLOR_WHITE;
            // ui.add(Hyperlink::new("https://github.com/kiosion/jupiter").text("Github").text_style(eframe::egui::TextStyle::Small));
            
            ui.add_space(PADDING_MD);
        });
    });
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Home");
    });
    ui.add_space(PADDING_XL);
    let sep = Separator::default().spacing(PADDING_XL);
    ui.add(sep);
}

fn main() {
    let app = Jupiter::new();
    let mut win_opts = NativeOptions::default();
    win_opts.initial_window_size = Some(Vec2::new(560., 740.));
    win_opts.transparent = true;
    win_opts.decorated = false;
    run_native(Box::new(app), win_opts);
}
