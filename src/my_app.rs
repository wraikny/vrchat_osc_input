use eframe::{egui, epi};
use async_std::{channel::Sender, task};

use crate::osc::Msg;

pub fn create_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(100.0, 60.0) * 3.0),
        // resizable: false,
        ..Default::default()
    }
}

#[derive(Debug)]
pub struct MyApp {
    sender: Sender<Msg>,
    move_forward: bool,
    run: bool,
}

impl MyApp {
    pub fn new(sender: Sender<Msg>) -> Self {
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
    sender: &Sender<Msg>,
    current_value: &mut bool,
    to_msg: impl FnOnce(bool) -> Msg
) {
    let current = current_value.clone();
    let clicked = ui.selectable_value(current_value, true, label).clicked();

    if clicked {
        *current_value = !current;
        let msg = to_msg(!current);
        let sender = sender.clone();
        _ = task::spawn(async move {
            sender.send(msg).await.unwrap()
        });
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

    fn on_exit(&mut self) {
        let msgs = vec![
            (self.run, Msg::Run(false)),
            (self.move_forward, Msg::MoveForward(false))
        ];

        if msgs.iter().any(|x| x.0) {
            let sender = self.sender.clone();
            task::block_on(async {
                for (_, msg) in msgs.into_iter().filter(|x| x.0) {
                    sender.send(msg).await.unwrap();
                }
            });
        }
    }
}
