// use eframe::egui::CentralPanel;
use eframe::egui::Panel;
use std::path::PathBuf;

/* LEARNING UI */

#[derive(Default)]
struct AppLab{
    text_field_1:String,
    text_field_2:String,
    crt_menu_tab:ActiveTab, 
}

#[derive(Default)]
struct Cue{
    cue_number:u16,
    cue_color:String, //I hate this spelling smh
    cue_name:String,
    cue_target:Option<PathBuf>,
    cue_note:String,
    cue_duration:String,
    pre_wait:String,
    post_wait:String,
    cue_continue:bool, 
}

#[derive(PartialEq, Default)]
enum ActiveTab{
    #[default] Basic, 
    Triggers, 
    TimeAndLoops, 
    AudioLevels, 
    AudioTrim, 
    AudioEffects}

impl eframe::App for AppLab {
    // A method to directly refer to the ui
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        //Show_inside gives a background/margin
        Panel::top("the_top_panel").show_inside(ui, |ui| {
            ui.vertical(|ui|{
                ui.horizontal(|ui|{
                    let row_height:f32 = 100.0;
                //1. GO BUTTON
                    //Button stylization
                    let button_size = eframe::egui::vec2(150.0, row_height);
                    let button_text = eframe::egui::RichText::new("GO!")
                        .size(80.0)
                        .text_style(eframe::egui::TextStyle::Button)
                        .color(eframe::egui::Color32::WHITE);
                    
                    //Save old visuals before changing them
                    let default_visuals = ui.visuals().clone();
                    
                    //Define visuals with a green border
                    let but_border = eframe::egui::Stroke::new(5.0, eframe::egui::Color32::from_rgb(0, 200, 0));
                        ui.visuals_mut().widgets.inactive.bg_stroke = but_border;
                        ui.visuals_mut().widgets.hovered.bg_stroke = but_border;
                        ui.visuals_mut().widgets.active.bg_stroke = but_border;
                    
                    //If button event becomes true
                    if ui.add(eframe::egui::Button::new(button_text).min_size(button_size)).clicked() {
                        println!("Button was pressed")
                    }
                    
                    ui.separator();
                    
                    //Reset visuals to default
                    *ui.visuals_mut() = default_visuals;
                
                //2. CUE NAME & NOTES
                    ui.vertical(|ui| {
                        let remaining_width = ui.available_width();

                        ui.add_space(row_height/7.0);

                        ui.add_sized(
                            eframe::egui::vec2(remaining_width, row_height/3.5),
                            eframe::egui::TextEdit::singleline(&mut self.text_field_1)
                            .hint_text("Cue Name")
                            .font(eframe::egui::FontId::proportional(row_height/5.0)));

                        ui.add_space(row_height/7.0);

                        ui.add_sized(
                            eframe::egui::vec2(remaining_width, row_height/3.5),
                            eframe::egui::TextEdit::singleline(&mut self.text_field_2)
                            .hint_text("Notes")
                            .font(eframe::egui::FontId::proportional(row_height/5.0)));
                    });
                });
                
            });
        });

        Panel::bottom("the_lower_panel").show_inside(ui, |ui|{

            ui.vertical(|ui|{
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.crt_menu_tab, ActiveTab::Basic, "Basic");
                    ui.selectable_value(&mut self.crt_menu_tab, ActiveTab::Triggers, "Triggers");
                    ui.selectable_value(&mut self.crt_menu_tab, ActiveTab::TimeAndLoops, "Time & Loops");
                    ui.selectable_value(&mut self.crt_menu_tab, ActiveTab::AudioLevels, "Audio Levels");
                    ui.selectable_value(&mut self.crt_menu_tab, ActiveTab::AudioTrim, "Audio Trim");
                    ui.selectable_value(&mut self.crt_menu_tab, ActiveTab::AudioEffects, "Audio Effects");
                });
                
                ui.separator();
                
                let available_size = eframe::egui::vec2(ui.available_width(), ui.available_height() / 3.0);

                ui.allocate_ui_with_layout(
                  available_size,
                  eframe::egui::Layout::top_down(eframe::egui::Align::LEFT),
                  |ui| {
                    eframe::egui::ScrollArea::vertical()
                    .id_salt("lower_panel_scroll")
                    .show(ui, |ui|{
                        match self.crt_menu_tab{
                            ActiveTab::Basic => {
                                let button_text = eframe::egui::RichText::new("GO!");
                                if ui.add(eframe::egui::Button::new(button_text)).clicked() {
                                    println!("Currently the Basic Tab");
                                }
                            },
                            ActiveTab::Triggers => {
                                let button_text = eframe::egui::RichText::new("GO!");
                                if ui.add(eframe::egui::Button::new(button_text)).clicked() {
                                    println!("Currently the Triggers Tab");
                                }
                            },
                            ActiveTab::TimeAndLoops => {
                                let button_text = eframe::egui::RichText::new("GO!");
                                if ui.add(eframe::egui::Button::new(button_text)).clicked() {
                                    println!("Currently the Time & Loops Tab");
                                }
                            },
                            ActiveTab::AudioLevels => {
                                let button_text = eframe::egui::RichText::new("GO!");
                                if ui.add(eframe::egui::Button::new(button_text)).clicked() {
                                    println!("Currently the Audio Levels Tab");
                                }
                            },
                            ActiveTab::AudioTrim => {
                                let button_text = eframe::egui::RichText::new("GO!");
                                if ui.add(eframe::egui::Button::new(button_text)).clicked() {
                                    println!("Currently the Audio Trim Tab");
                                }
                            },
                            ActiveTab::AudioEffects => {
                                let button_text = eframe::egui::RichText::new("GO!");
                                if ui.add(eframe::egui::Button::new(button_text)).clicked() {
                                    println!("Currently the Audio Effects Tab");
                                }
                            },
                        }
                    })
                  }  
                );
            });
        });

    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1000.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native("ALab - Untitled Workspace(1.)", options, Box::new(|_cc| Ok(Box::<AppLab>::default())))
}