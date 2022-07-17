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

    draw.background().color(WHITE);

    for i in -10..10 {
        for j in -10..10 {
            let space = 25.;
            let scl = ((i + j) as f32) + (angle.cos() * 100.);
            draw.rect()
                .x_y((i as f32)*space, (j as f32)*space)
                .w_h(scl, scl)
                .rotate(angle)
                .stroke(BLACK)
                .stroke_weight(5.)
                .color(srgba8(0, 0, 0, 0));

        }
    }

    draw.to_frame(app, &frame).unwrap();
}
