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

    draw.background().color(WHEAT);

    
    for i in 1..100 {
        for j in 1..100 {
            let x = (i as f32) * angle.sin();
            let y = (j as f32) * angle.cos();
            draw.rect().x_y(x,y).w_h(200., 200.).no_fill().stroke(BLACK).stroke_weight(1.0);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
