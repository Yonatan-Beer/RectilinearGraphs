use std::ops::Add;

use egui::*;
use epaint::CircleShape;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Modes {
    Add,
    Connect,
    Move,
    Delete
}

fn are_incident(e1: [CircleShape; 2], e2: [CircleShape; 2]) -> bool{
    if e1[0].center == e2[0].center || e1[1].center == e2[0].center || e1[0].center == e2[1].center || e1[1].center == e2[1].center{
        return true;
    }
    return false;
}

fn intersect(e1: Vec<[Pos2; 2]>, e2: Vec<[Pos2; 2]>) -> bool{

    return false
}

fn unordeq(e1:&[CircleShape; 2],e2: &[CircleShape; 2]) -> bool{
    if (e1[0].eq(&e2[1]) && e1[1].eq(&e2[0])) || e1.eq(e2) {
        return true;
    }
    return false;
}

pub struct Graphs {

    vertices: Vec<CircleShape>,
    edges: Vec<[CircleShape; 2]>,
    stroke: Stroke,
    fill: Color32,
    highlight: Color32,
    mode: Modes,
    radius: f32,
    cur: CircleShape
}

impl Default for Graphs {
    fn default() -> Self {
        Self {
            vertices: Default::default(),
            edges: Default::default(),
            stroke: Stroke::new(1.0, Color32::from_rgb(200, 100, 100)),
            fill: Color32::from_rgb(50, 100, 150),
            highlight: Color32::from_rgb(255, 255, 0),
            mode: Modes::Add,
            radius: 12.0, 
            cur: CircleShape::stroke(Pos2::ZERO, 0.0, Stroke::NONE)
        }
        
    }
}

fn makeboundbox(center: Pos2, radius:f32) -> Rect{ 
    let topleft = Pos2::new(center.x - radius, center.y - radius);
    return Rect::from_min_size(topleft, Vec2::new(radius+radius, radius+radius));
}

impl Graphs {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn buttons(&mut self, ui: &mut egui::Ui) {
        let Self {
            vertices,
            edges,
            stroke,
            fill,
            highlight,
            mode,
            radius,
            cur,
        } = self;

        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.selectable_value(mode, Modes::Add, "Add Vertices");
            ui.selectable_value(mode, Modes::Connect, "Add Edges");
            ui.selectable_value(mode, Modes::Move, "Move Vertices");
        });

        ui.collapsing("Colors", |ui| {
            Grid::new("colors")
                .num_columns(2)
                .spacing([12.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("vertex radius");
                    ui.add(egui::DragValue::new(&mut self.radius).speed(0.1));

                    ui.label("vertex color");
                    ui.color_edit_button_srgba(&mut self.fill);

                    ui.label("edge width");
                    egui::widgets::stroke_ui(ui,&mut self.stroke, "edge color");
                    
                });

                
        });        
    }

    fn onclick(&mut self, ui: &mut egui::Ui) {
        let (mut response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());
        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();
        if self.mode == Modes::Add {
            self.cur = CircleShape::stroke(Pos2::ZERO, 0.0, Stroke::NONE);
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                let vert = CircleShape::filled(pointer_pos, self.radius, self.fill);
                self.vertices.push(vert);
            }
        } 

        if self.mode == Modes::Connect {
            let mut responses: Vec<Response> = Vec::new();
            for node in self.vertices.clone(){
                responses.push(ui.put(makeboundbox(node.center, self.radius), egui::widgets::Button::new("")));
            }

            if let Some(pointer_pos) = response.interact_pointer_pos() {
                
            }
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
            self.onclick(ui);
            let painter = ui.painter();

            let edgelist = self.edges.clone();
            for edge in edgelist {
                let cordedge = [edge[0].center, edge[1].center];
                painter.line_segment(cordedge, self.stroke);
            }

            let verlist = self.vertices.clone();
            for vertex in verlist {
                painter.circle_filled(vertex.center, self.radius, self.fill);
            }
            if self.cur.center != Pos2::ZERO{
                painter.circle_stroke(self.cur.center, self.radius, Stroke::new(0.0, Color32::from_rgb(0, 0,0)));
            }
        });
    }
}


