use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
}

fn model(_app: &App) -> Model {
    let _window = _app.new_window().view(view).build().unwrap();

    Model { }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.elapsed_frames();
    let angle = time as f32 / 100.;
    let angle2 = time as f32 / 300.;

    if app.elapsed_frames() == 1 {
        draw.background().color(WHITE);
    }
    draw.rect().w_h(1000.0, 1000.0).color(srgba(1., 1., 1., 0.001));

    let r1 = 50. * (angle2.sin() + 1.);
    let r2 = 300. * (angle.sin() + 1.);

    draw.line()
        .start(vec2(r1*angle.sin(), r1*angle.cos()))
        .end(vec2(r2*angle2.cos(), r2*angle2.sin()))
        .stroke_weight(1.)
        .caps_round()
        .color(ROYALBLUE);

    draw.line()
        .start(vec2(r1*angle2.sin(), r1*angle2.cos()))
        .end(vec2(r2*angle.cos(), r2*angle.sin()))
        .stroke_weight(1.)
        .caps_round()
        .color(RED);

    draw.to_frame(app, &frame).unwrap();
}
