use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
}

fn model(_app: &App) -> Model {
    let _window = _app.new_window().view(view).build().unwrap();

    Model {  }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect().w_h(1000.0, 1000.0).color(srgba(0., 0., 0., 0.001));

    draw.ellipse()
        .x_y(app.mouse.x, app.mouse.y)
        .radius(app.mouse.y - app.mouse.x)
        .stroke(WHEAT).stroke_weight(1.0)
        .no_fill();

    draw.to_frame(app, &frame).unwrap();
}
