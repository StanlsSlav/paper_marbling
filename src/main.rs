use nannou::{prelude::*, state::mouse};
use nannou_egui::{self, egui, Egui};

mod ellipse;
use ellipse::Ellipse;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug, Clone, Copy)]
struct Settings {
    edges: i32,
    radius: f32,
    color: Srgb<u8>,
    stroke: Srgb<u8>,
    stroke_width: f32,
}

struct Model {
    drops: Vec<Ellipse>,

    egui: Egui,
    settings: Settings,
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        const SPACE: f32 = 16.0;
        ui.label("Edges:");
        ui.add(egui::Slider::new(&mut settings.edges, 0..=100));

        ui.label("Radius:");
        ui.add(egui::Slider::new(&mut settings.radius, 0.0..=100.0));

        ui.add_space(SPACE);
        ui.label("Color:");
        ui.group(|ui| {
            ui.label("Red:");
            ui.add(egui::Slider::new(&mut settings.color.red, 0..=255));

            ui.label("Green:");
            ui.add(egui::Slider::new(&mut settings.color.green, 0..=255));

            ui.label("Blue:");
            ui.add(egui::Slider::new(&mut settings.color.blue, 0..=255));
        });

        ui.add_space(SPACE);
        ui.label("Stroke");
        ui.group(|ui| {
            ui.label("Width:");
            ui.add(egui::Slider::new(&mut settings.stroke_width, 0.0..=100.0));

            ui.label("Color:");
            ui.group(|ui| {
                ui.label("Red:");
                ui.add(egui::Slider::new(&mut settings.stroke.red, 0..=255));

                ui.label("Green:");
                ui.add(egui::Slider::new(&mut settings.stroke.green, 0..=255));

                ui.label("Blue:");
                ui.add(egui::Slider::new(&mut settings.stroke.blue, 0..=255));
            });
        });
    });
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    let random_u8 = || -> u8 {
        random_range(0, 255) as u8
    };
    Model {
        drops: Vec::new(),

        egui,
        settings: Settings {
            edges: 32,
            radius: 64.0,
            color: Srgb::new(random_u8(), random_u8(), random_u8()),
            stroke: Srgb::new(random_u8(), random_u8(), random_u8()),
            stroke_width: 4.0,
        },
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let drops = <Vec<ellipse::Ellipse> as Clone>::clone(&model.drops);
    drops.into_iter().for_each(|x| {
        x.show(&draw);
    });

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn mouse_pressed(app: &App, model: &mut Model, mouse_button: MouseButton) {
    if MouseButton::Right != mouse_button {
        return;
    }

    let mouse: &mouse::Mouse = &app.mouse;
    let drop = Ellipse::new(mouse.x, mouse.y, model.settings.clone());

    let _ = model.drops.clone().into_iter().map(|mut other| other.marble(drop));
    model.drops.push(drop);
}
