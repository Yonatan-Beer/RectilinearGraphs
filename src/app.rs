use egui::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Modes {
    Add,
    Connect,
    Move
}

fn are_incident(e1: Vec<[Pos2; 2]>, e2: Vec<[Pos2; 2]>) -> bool{
    if(e1[0] == e2[0] || e1[1] == e2[0] || e1[0] == e2[1] || e1[1] == e2[1]){
        return true;
    }
    return false;
}

fn intersect(e1: Vec<[Pos2; 2]>, e2: Vec<[Pos2; 2]>) -> bool{

    return false
}

pub struct Graphs {

    vertices: Vec<Pos2>,
    edges: Vec<[Pos2; 2]>,
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

    fn draw_vertices(self, painter: &Painter){
        for vertex in self.vertices {
            painter.circle(vertex, 0.1, self.fill, self.stroke);
        }
    }

    fn draw_edges(self, painter: &Painter){
        for edge in self.edges {
            painter.line_segment(edge, self.stroke);
        }
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
                self.buttons(ui);

            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter();
            let painter = ui.painter();

            let edgelist = self.edges.clone();
            for edge in edgelist {
                painter.line_segment(edge, self.stroke);
            }

            let verlist = self.vertices.clone();
            for vertex in verlist {
                painter.circle(vertex, 0.1, self.fill, self.stroke);
            }
        });
    }
}


