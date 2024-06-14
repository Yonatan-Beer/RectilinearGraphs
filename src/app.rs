use itertools::Itertools;

use egui::*;
use epaint::CircleShape;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Modes {
    Add,
    Connect,
    Move,
    Delete,
    Disconnect,
}

fn are_incident(e1: [Pos2; 2], e2: [Pos2; 2]) -> bool{
    if e1[0] == e2[0] || e1[1] == e2[0] || e1[0] == e2[1] || e1[1] == e2[1]{
        return true;
    }
    return false;
}

fn intersect(e1: [Pos2; 2], e2: [Pos2; 2]) -> bool{
    let det = (e1[1].x - e1[0].x) * (e2[1].y - e2[0].y) - (e1[1].y - e1[0].y) * (e2[1].x - e2[0].x);
    if det == 0.0 {
        return false;
    } else {
        let lam = ((e2[1].y - e2[0].y) * (e2[1].x - e1[0].x) + (e2[0].x - e2[1].x) * (e2[1].y - e1[0].y))/det;
        let gam = ((e1[0].y - e1[1].y) * (e2[1].x - e1[0].x) + (e1[1].x - e1[0].x) * (e2[1].y - e1[0].y))/det;
        return (0.0 < lam  && lam < 1.0) && (0.0 < gam && gam < 1.0); 
    }
}

fn unordeq(e1:&[CircleShape; 2],e2: &[CircleShape; 2]) -> bool{
    if (e1[0].eq(&e2[1]) && e1[1].eq(&e2[0])) || e1.eq(e2) {
        return true;
    }
    return false;
}

pub struct Graphs {

    vertices: Vec<CircleShape>,
    edges: Vec<[usize; 2]>,
    stroke: Stroke,
    fill: Color32,
    highlight: Stroke,
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
            highlight: Stroke::new(2.0,Color32::from_rgb(255, 255, 0)),
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


    fn count_intersections(&mut self) -> i64{
        let mut count = 0;
        let copyof =  self.edges.clone();
        let edgepairs = copyof.iter().cartesian_product(self.edges.iter());
        for pair in edgepairs{
            let e1 = [self.vertices[pair.0[0]].center, self.vertices[pair.0[1]].center];
            let e2 = [self.vertices[pair.1[0]].center, self.vertices[pair.1[1]].center];
            if (intersect(e1, e2) && !are_incident(e1, e2)){
                count+=1;
            }

        }
        return count/2;
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

            ui.add_space(20.0);

            ui.selectable_value(mode, Modes::Delete, "Delete Vertices");
            ui.selectable_value(mode, Modes::Disconnect, "Delete Edges");
        });

        ui.add_space(40.0);

        if ui.add(egui::Button::new("Delete Graph").stroke(Stroke::new(1.0, Color32::from_rgb(244, 244, 244)))).clicked(){
            self.vertices =  Default::default();
            self.edges = Default::default();
        }

        ui.add_space(40.0);
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
                    
                    ui.label("highlight width");
                    egui::widgets::stroke_ui(ui,&mut self.highlight, "highlight color");
                });

                
        });        
    }

    fn onclick(&mut self, ui: &mut egui::Ui) {
        let (mut response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());
        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );

        if self.mode == Modes::Add {
            self.cur = CircleShape::stroke(Pos2::ZERO, 0.0, Stroke::NONE);
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                let mut safezone = true;
                for node in self.vertices.clone() {
                    if (pointer_pos.distance(node.center)) < self.radius + self.radius {
                        safezone = false;
                    }
                }
                if safezone {
                    let vert = CircleShape::filled(pointer_pos, self.radius, self.fill);
                    self.vertices.push(vert);
                }
            }
        } 

        if self.mode == Modes::Connect {
            let mut responses: Vec<Response> = Vec::new();
            for node in self.vertices.clone(){
                responses.push(ui.put(makeboundbox(node.center, self.radius), egui::widgets::Button::new("")));
            }

            for i in 0..responses.len(){
                if responses[i].clicked(){
                    if self.cur.center == Pos2::ZERO{
                        self.cur = self.vertices[i];
                    } else if self.vertices[i] == self.cur{
                        continue;
                    } else {
                        
                        let newedge = [self.cur, self.vertices[i]];
                        let mut alreadyhere = false;
                        for edge in self.edges.clone(){
                            let pointedge = [self.vertices[edge[0]],self.vertices[edge[1]]];
                            if unordeq(&newedge, &pointedge){
                                alreadyhere = true;
                            }
                        }
                        if !alreadyhere{
                            for j in 0..self.vertices.len(){
                                if self.vertices[j] == self.cur {
                                    self.edges.push([j,i]);
                                }
                            }
                        
                        }
                        self.cur = CircleShape::stroke(Pos2::ZERO, 0.0, Stroke::NONE);
                    }
                }
            }
        }

        if self.mode == Modes::Disconnect {
            let mut responses: Vec<Response> = Vec::new();
            for node in self.vertices.clone(){
                responses.push(ui.put(makeboundbox(node.center, self.radius), egui::widgets::Button::new("")));
            }

            for i in 0..responses.len(){
                if responses[i].clicked(){
                    if self.cur.center == Pos2::ZERO{
                        self.cur = self.vertices[i];
                    } else if self.vertices[i] == self.cur{
                        continue;
                    } else {
                        for j in 0..self.vertices.len(){
                            if self.vertices[j] == self.cur {
                               let e1 = [i,j];
                               let e2 = [j,i];
                               self.edges.retain(|x| *x != e1 && *x != e2);
                            }
                        }
                            
                        self.cur = CircleShape::stroke(Pos2::ZERO, 0.0, Stroke::NONE);
                    }
                }
            }
        }

        if self.mode == Modes::Delete {
            self.cur = CircleShape::stroke(Pos2::ZERO, 0.0, Stroke::NONE);
            let mut responses: Vec<Response> = Vec::new();
            for node in self.vertices.clone(){
                responses.push(ui.put(makeboundbox(node.center, self.radius), egui::widgets::Button::new("")));
            }
            for i in 0..responses.len(){
                if responses[i].clicked(){
                    self.vertices.remove(i);
                    let mut marked: Vec<usize> = Vec::new();
                    let mut egs = self.edges.clone();
                    for k in 0..egs.len(){
                        if egs[k][0] == i || egs[k][1] == i {
                            marked.push(k);
                        }
                        if egs[k][0] > i {
                            egs[k][0] -= 1;
                        }
                        if egs[k][1] > i {
                            egs[k][1] -= 1;
                        }
                    }
                    marked.reverse();
                    for ind in marked{
                        egs.remove(ind);
                    }
                    self.edges = egs;
                }
            }
        }

        if self.mode == Modes::Move {
            self.cur = CircleShape::stroke(Pos2::ZERO, 0.0, Stroke::NONE);
            let mut responses: Vec<Response> = Vec::new();
            for node in self.vertices.clone(){
                responses.push(ui.put(makeboundbox(node.center, self.radius), egui::widgets::Button::new("")).interact(Sense::click_and_drag()));
            }
            for i in 0..responses.len(){
                if responses[i].is_pointer_button_down_on(){ 
                    self.vertices[i] = CircleShape::filled(self.vertices[i].center + responses[i].drag_delta(), self.radius, self.fill);

   
                }
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
            let arr = egui::Window::new("")
            .title_bar(false)
            .auto_sized()
            .movable(true)
            .default_pos(Pos2::new(2.0, 2.0))
            .show(ctx, |ui| {
                let crosstext = format!("Crossings: {}", self.count_intersections());
                let rt = RichText::new(crosstext).size(20.0).underline();
                ui.colored_label(Color32::from_rgb(240, 120, 40), rt);
             });
    

            ui.painter();
            self.onclick(ui);
            let painter = ui.painter();

            let edgelist = self.edges.clone();
            for e in edgelist {
                let v1 = self.vertices[e[0]]; let v2 = self.vertices[e[1]];
                let cordedge = [v1.center, v2.center];
                painter.line_segment(cordedge, self.stroke);
            }

            let verlist = self.vertices.clone();
            for vertex in verlist {
                painter.circle_filled(vertex.center, self.radius, self.fill);
            }
            if self.cur.center != Pos2::ZERO{
                painter.circle_stroke(self.cur.center, self.radius, self.highlight);
            }
        });
    }
}


