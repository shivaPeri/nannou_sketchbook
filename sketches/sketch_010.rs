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
    
    draw.background().color(BLACK);
    
    for i in 20..100 {
        for j in 10..100 {
            let weight = (((i + j) as f32 + (time as f32)) / 100.0).sin();
            draw.ellipse()
                .x_y((i as f32) * 10.0 - 600.0, (j as f32) * 10.0 - 400.0)
                .radius(((j - i) as f32) * (time as f32 / 100.0).sin())
                .stroke(WHEAT).stroke_weight(weight)
                .no_fill();
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
