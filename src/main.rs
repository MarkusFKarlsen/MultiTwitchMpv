use eframe::egui;
use std::process::Command;
use std::thread;

const VIDEO_URL: &str = "https://www.twitch.tv/";

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Mpv Stream selector", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

//#[derive(Default)]
struct MyEguiApp {
    inputstring: String,
    streamers: Vec<String>,
    del_index: i32,
}

impl Default for MyEguiApp{
    fn default() -> Self {
        Self { 
            inputstring: "".to_string(),
            streamers: Vec::new(),
            del_index: 99,
        }
    }
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::TopBottomPanel::top("UpperPanel").show(&ctx, |ui|{
           ui.label("Welcome to the MPV stream selector!")
       });
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Input a streamer name in the text field");
           ui.label("Press enter to lock in");
           //let response = ui.add(text_edit);
           let output = egui::TextEdit::singleline(&mut self.inputstring)
               .hint_text("Input Streamer name").show(ui);
           if ui.input(|i| i.key_pressed(egui::Key::Enter)){
              self.streamers.push(String::from(& self.inputstring));
           }
           for (i,x) in self.streamers.iter().enumerate() {
               ui.horizontal(|ui| {
                ui.label(x);
                let red_button = egui::Button::new("Delete").fill(egui::Color32::RED);
                if ui.add(red_button)
                    .clicked() {
                    self.del_index = i as i32;
                };
               });
           }
           if self.del_index < 99 {
              self.streamers.remove(self.del_index as usize);
              self.del_index = 99;
           };
           if ui.button("Click to activate Mpv").clicked(){
               for x in &self.streamers  {
                  run_mpv(x.to_string());
               }
           };
       });
   }
}


fn run_mpv(streamer:String){
    let stream_name = VIDEO_URL.to_owned() + &streamer;
    thread::spawn(move || {
    let _output = Command::new("mpv")
        .arg(stream_name)
        .output()
        .expect("failed to execute");
    });
}
