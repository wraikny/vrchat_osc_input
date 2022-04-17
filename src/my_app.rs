use eframe::{egui, epi};
use async_std::channel::Sender;

use crate::osc::Msg;

pub fn create_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        always_on_top: true,
        initial_window_size: Some(egui::Vec2::new(100.0, 60.0) * 3.0),
        // resizable: false,
        ..Default::default()
    }
}

type MySender = Sender<Msg>;

#[derive(Debug)]
pub struct MyApp {
    sender: MySender,
    move_forward: bool,
    run: bool,
}

impl MyApp {
    pub fn new(sender: MySender) -> Self {
        Self {
            sender,
            move_forward: false,
            run: false,
        }
    }
}

fn osc_toggle(
    ui: &mut egui::Ui,
    label: impl Into<egui::WidgetText>,
    sender: &MySender,
    current_value: &mut bool,
    to_msg: impl FnOnce(bool) -> Msg
) {
    let current = current_value.clone();
    let clicked = ui.selectable_value(current_value, true, label).clicked();

    if clicked {
        *current_value = !current;
        let msg = to_msg(!current);
        sender.try_send(msg).unwrap()
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "vrchat osc input"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            osc_toggle(ui, "Run", &self.sender, &mut self.run, Msg::Run);
            osc_toggle(ui, "MoveForward", &self.sender, &mut self.move_forward, Msg::MoveForward);
        });
    }
}
