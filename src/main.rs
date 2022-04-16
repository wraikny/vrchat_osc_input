#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate async_std;
extern crate rosc;
extern crate eframe;

mod my_app;
mod osc;

use async_std::{channel, task};

fn main() -> Result<(), String> {
    let (send, recv) = channel::unbounded();

    _ = task::spawn(async {
        let client = osc::Client::new(recv).await;
        client.run().await.unwrap_or_else(|e| panic!("{}", e))
    });

    let app = my_app::MyApp::new(send);
    let options = my_app::create_options();
    eframe::run_native(Box::new(app), options);
}
