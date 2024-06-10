use egui::*;

#[derive(Debug, PartialEq)]
enum Modes {
    Add,
    Connect,
    Move
}

pub struct Graphs {

    vertices: Vec<Pos2>,
    edges: Vec<Vec<Pos2>>,
    stroke: Stroke,
    fill: Color32,
    mode: Modes,
    
}

impl Default for Graphs {
    fn default() -> Self {
        Self {
            vertices: Default::default(),
            edges: Default::default(),
            stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
            fill: Color32::from_rgb(50, 100, 150).linear_multiply(0.25),
            mode: Modes::Add
        }
    }
}
impl Graphs {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    fn buttons(&mut self, ui: &mut egui::Ui) {
        let Self {
            vertices,
            edges,
            stroke,
            fill,
            mode,
        } = self;

        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.selectable_value(mode, Modes::Add, "Add Vertices");
            ui.selectable_value(mode, Modes::Connect, "Add Edges");
            ui.selectable_value(mode, Modes::Move, "Move Vertices");
        });
    }
}

impl eframe::App for Graphs {
   
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                
                self.buttons(ui);


            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter();
           
        });
    }
}


