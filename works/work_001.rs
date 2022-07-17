use nannou::{prelude::*, noise::*};

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    p1: Vec2,
    p2: Vec2,
    c: Rgb<u8>,
    w: f32,
}

impl Thing {
    pub fn new(p1: Vec2, p2: Vec2, c: Rgb<u8>, w: f32) -> Self {
        Thing { p1, p2, c, w }
    }
}

const N_THINGS: usize = 20;
const MAX_WIDTH: f32 = 10.0;

struct Model {
    things: Vec<Thing>,
    noise: Perlin, 
    radius: f32
}

fn model(_app: &App) -> Model {
    let _window = _app.new_window().view(view).build().unwrap();
    let mut things = Vec::new();

    for _i in 0..N_THINGS {
        let line = Thing::new(
            vec2(random_f32()-0.5, random_f32()-0.5) * 1000.0, 
            vec2(random_f32()-0.5, random_f32()-0.5) * 1000.0, 
            rgb8(random(), random(),random()),
            random_f32() * MAX_WIDTH
            // 0.1
        );
        things.push(line);
    }

    let noise = Perlin::new();
    Model { things, noise, radius: 100.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

    let scl = 0.01;
    let speed = 3.5;

    for line in model.things.iter_mut() {
        line.p1 += vec2(
            model.noise.get([ scl*line.p1.x as f64,  scl*line.p1.y as f64, 0.0]) as f32,
            model.noise.get([ scl*line.p1.x as f64,  scl*line.p1.y as f64, 1.0]) as f32,
        ) * speed;
        line.p2 += vec2(
            model.noise.get([ scl*line.p2.x as f64,  scl*line.p2.y as f64, 0.0]) as f32,
            model.noise.get([ scl*line.p2.x as f64,  scl*line.p2.y as f64, 1.0]) as f32,
        ) * speed;

        // line.w = 100.0 * model.noise.get([scl*line.p1.x as f64, scl*line.p1.y as f64]) as f32;
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.elapsed_frames();
    let angle = time as f32 / 100.0;

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect().w_h(1000.0, 1000.0).color(srgba(0.0, 0.0, 0.0, 0.1));

    for line in model.things.iter() {
        draw.line().start(line.p1).end(line.p2 - line.p1).color(line.c).stroke_weight(line.w).caps_round();
    }

    draw.ellipse().x_y(model.radius * angle.cos(), model.radius * angle.sin()).color(WHEAT);

    draw.to_frame(app, &frame).unwrap();
}
