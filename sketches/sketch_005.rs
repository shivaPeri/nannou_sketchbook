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
    draw.rect().w_h(1000.0, 1000.0).color(srgba(1., 1., 1., 0.03));

    let _r1 = 50. * (angle2.sin() + 1.);
    let r2 = 200.;

    draw.ellipse()
        .radius(100.)
        .x_y(r2 * angle.cos(), r2 * angle.sin())
        .color(srgba8(255, 100, 100, 10));

    draw.ellipse()
        .radius(100.)
        .x_y((r2 - 20.) * angle.cos() + 30., (r2 - 20.) * angle.sin() + 30.)
        .color(srgba8(10, 10, 255, 10));

    draw.to_frame(app, &frame).unwrap();
}
