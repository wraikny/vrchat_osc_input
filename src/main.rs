#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate async_std;
extern crate rosc;
extern crate eframe;

mod my_app;
mod osc;

use async_std::{channel, task};

fn main() -> Result<(), String> {
    let (sender, receiver) = channel::unbounded();

    let app = my_app::MyApp::new(sender);

    _ = task::spawn(async move {
        let client = osc::Client::new().await;
        if let Ok(msg) = receiver.recv().await {
            client.send_msg(msg).await.unwrap_or_else(|e| panic!("{}", e));
        }
    });

    
    let options = my_app::create_options();
    eframe::run_native(Box::new(app), options);
}
