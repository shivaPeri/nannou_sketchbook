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
    let angle = time as f32 / 100.0;

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect().w_h(1000.0, 1000.0).color(srgba(0.0, 0.0, 0.0, 0.01));

    draw.ellipse().x_y(200. * angle.cos(), 200. * angle.sin()).color(WHEAT);
    draw.ellipse().radius(10.).x_y(100. * angle.cos(), 100. * angle.sin()).color(RED);

    draw.ellipse().radius(1.).x_y(120. * angle.sin(), 120. * angle.cos()).color(SLATEBLUE);
    draw.ellipse().radius(1.).x_y(125. * angle.sin(), 125. * angle.cos()).color(SLATEBLUE);
    draw.ellipse().radius(1.).x_y(130. * angle.sin(), 130. * angle.cos()).color(SLATEBLUE);
    draw.ellipse().radius(1.).x_y(140. * angle.sin(), 140. * angle.cos()).color(SLATEBLUE);

    draw.to_frame(app, &frame).unwrap();
}
