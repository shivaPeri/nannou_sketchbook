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
    let time = app.elapsed_frames();
    let angle = time as f32 / 100.;

    draw.background().color(BLACK);

    for i in 20..100 {
        for j in 10..500 {
            draw.polygon()
            .x_y((i as f32), (j as f32))
            .points()
            .color(WHITE);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
