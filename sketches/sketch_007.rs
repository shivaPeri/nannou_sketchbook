use std::collections::VecDeque;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

const N_POINTS: usize = 100;

struct Model {
    points: VecDeque<Vec2>
}

fn model(_app: &App) -> Model {
    let _window = _app.new_window().view(view).build().unwrap();

    Model { points: VecDeque::new() }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.points.push_front(vec2(app.mouse.x, app.mouse.y));

    if model.points.len() > N_POINTS {
        model.points.pop_back();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.elapsed_frames();
    let _angle = time as f32 / 100.;

    draw.background().color(WHEAT);

    for i in 1..model.points.len() {
        draw.line().start(model.points[i-1]).end(model.points[i])
            .stroke_weight(10.).color(BLACK).caps_round();
    }

    draw.to_frame(app, &frame).unwrap();
}
