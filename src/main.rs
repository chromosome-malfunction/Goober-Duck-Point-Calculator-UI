#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use eframe::{egui, NativeOptions};
use egui::IconData;
use image::{ImageFormat, GenericImageView};

fn main() -> Result<(), eframe::Error> {
    
    let icon_bytes = include_bytes!("icon.png");
    let image = image::load_from_memory_with_format(icon_bytes, ImageFormat::Png).expect("Failed to decode embedded icon image");
    let rgba = image.to_rgba8().into_raw();
    let (width, height) = image.dimensions();
    let icon = IconData {rgba, width, height};
    let options = NativeOptions {viewport: egui::ViewportBuilder::default().with_icon(icon), ..Default::default()};
    
    eframe::run_native("Point Calculator for Goober & Duck", options, Box::new(|_cc| Ok(Box::<MyApp>::default())))
}

#[derive(Default)]
struct PointMaps {
    duckmap: HashMap<String, i32>,
    goobermap: HashMap<String, i32>,
}

#[derive(Default)]
struct MyApp {
    light_mode: bool,
    given: String,
    user: String,
    maps: PointMaps,
    dg: String,
    goober_pop: bool,
    duck_pop: bool,
    goobercheck: bool,
    duckcheck: bool,
    gooberoption: String,
    duckoption: String,
    option: String,
    err: String,
    errs: String,
    option2: String,
    number: String,
    check: String,
    opt: String,
    points: String,
    pointsgoober: String,
    pointsduck: String,
    
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.light_mode {
            ctx.set_visuals(egui::Visuals::light());
        } else {
            ctx.set_visuals(egui::Visuals::dark());
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.checkbox(&mut self.light_mode, "light mode");
            });

            ui.heading("Point calculator");

            ui.label("Enter username:");
            ui.text_edit_singleline(&mut self.user);
            
            let user = (|| -> bool {
                if self.user.to_lowercase().chars().count() < 2 || self.user.to_lowercase().chars().count() > 32 {
                    ui.label("Username must be at least 2 characters long or shorter than 32 characters.");
                    return true;
                }

                return false;
            })();
            if user { return; }
            
            ui.separator();
            
            egui::ComboBox::from_label("")
                .selected_text(&self.dg)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.goober_pop, true, "Goober");
                    ui.selectable_value(&mut self.duck_pop, true, "Duck");
                    //ui.selectable_value(&mut self.dg, "None".to_string(), "None");
                });

            self.points = {
                let mut dv: i32 = 0;
                let mut gv: i32 = 0;

                match self.maps.duckmap.get(&self.user) {
                    Some(value) => { dv = *value; },
                    None => {}
                }
                match self.maps.goobermap.get(&self.user) {
                    Some(value) => { gv = *value; },
                    None => {}
                }
                format!("{} has {} points in Duck.\n{} has {} points in Goober.", self.user, dv, self.user, gv)
            };

            self.pointsgoober = {
                let mut gv: i32 = 0;

                match self.maps.goobermap.get(&self.user) {
                    Some(value) => { gv = *value; },
                    None => {}
                }
                format!("{} has {} points in Goober.", self.user, gv)
            };

            self.pointsduck = {
                let mut dv: i32 = 0;

                match self.maps.duckmap.get(&self.user) {
                    Some(value) => { dv = *value; },
                    None => {}
                }

                format!("{} has {} points in Duck.", self.user, dv)
            };

            ui.label(&self.points);

            self.errs = format!("Latest error: '{}'", self.err);
            ui.label(&self.errs);

            /*if self.dg == "None" {
                self.goober_pop = false;
                self.duck_pop = false;
            }*/if self.duck_pop && self.goober_pop {
                self.dg = "Duck & Goober".to_string();
                //self.goober_pop = false;
            } else if self.duck_pop && !self.goober_pop {
                self.dg = "Duck".to_string();
            } else if self.goober_pop && !self.duck_pop {
                self.dg = "Goober".to_string();
            } else {
                self.dg = "None".to_string();
            }

            if self.goober_pop {
                egui::Window::new("Goober Point System").default_size(egui::Vec2::new(600.0, 400.0))
                    .show(ctx, |ui| {
                        ui.label("Which point value would you like to add?");

                        egui::ComboBox::from_label("")
                            .selected_text(&self.gooberoption)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.gooberoption, "R".to_string(), "Raider Elimination");
                                ui.selectable_value(&mut self.gooberoption, "B".to_string(), "Employee Bonus");
                                ui.selectable_value(&mut self.gooberoption, "LR".to_string(), "Lever Reset");
                                ui.selectable_value(&mut self.gooberoption, "SPT".to_string(), "Shift playtime");
                                ui.selectable_value(&mut self.gooberoption, "CR".to_string(), "CR work");
                                ui.selectable_value(&mut self.gooberoption, "Ns".to_string(), "Nuisances");
                                ui.selectable_value(&mut self.gooberoption, "RKE".to_string(), "RKing another Employee");
                                ui.selectable_value(&mut self.gooberoption, "RKV".to_string(), "RKing a Visitor");
                                ui.selectable_value(&mut self.gooberoption, "N".to_string(), "Being a nuisance");
                                ui.selectable_value(&mut self.gooberoption, "Rg".to_string(), "Raiding");
                                ui.selectable_value(&mut self.gooberoption, "NJ".to_string(), "Not doing your job");
                            });
                        
                        let mut numb = 0;
    
                        match self.maps.goobermap.get(&self.user) {
                            Some(/*value*/_) => { /*println!("User found: {}, with {} points", self.user, value);*/ },
                            None => { self.maps.goobermap.insert(self.user.to_string().to_lowercase(), 0); }
                        };
                        match self.gooberoption.to_lowercase().as_str() {
                            "r" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of raiders eliminated:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 5 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.errs);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "b" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of points for employee bonus:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 1 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "lr" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of levers reset:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 3 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "spt" => {
                                egui::ComboBox::from_label("Time unit")
                                    .selected_text(&self.option2)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.option2, "Hours".to_string(), "Hours");
                                        ui.selectable_value(&mut self.option2, "Minutes".to_string(), "Minutes");
                                    });
                                ui.vertical(|ui| {
                                    if self.option2 == "Hours" {
                                        ui.label("Number of hours played on shift:");
                                        ui.text_edit_singleline(&mut self.option);
                                    } else if self.option2 == "Minutes" {
                                        ui.label("Number of minutes played on shift:");
                                        ui.text_edit_singleline(&mut self.number);

                                        if let Ok(mins) = self.number.trim().parse::<f32>() {
                                            let hours = mins / 60.0;
                                            self.option = format!("{}", hours);
                                        } else {
                                            self.err = "Please enter a valid number.".to_string();
                                        }
                                    }
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 10 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "cr" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of visitors let in via airlock:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 1 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }

                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "ns" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of nuisances:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 3 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "rke" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of employees RK'd: (10 point decrease)");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 10 * num;
                                            *value -= numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "rkv" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of visitors RK'd: (5 point decrease):");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 5 * num;
                                            *value -= numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "n" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of times being a nuisance: (3 point decrease):");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 3 * num;
                                            *value -= numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "rg" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of times raiding: (30 point decrease):");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 30 * num;
                                            *value -= numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "nj" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of times not doing your job: (4 point decrease):");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.goobermap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 4 * num;
                                            *value -= numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            _ => {}
                        }
                        
                        self.check = format!("Override {}", self.user);
                        ui.checkbox(&mut self.goobercheck, &self.check);
                        if self.goobercheck {
                            ui.label("Override options:");
                            ui.separator();

                            ui.vertical(|ui| {
                                ui.text_edit_singleline(&mut self.opt);
                                if ui.button("Add").clicked() {
                                    match self.opt.trim().parse::<i32>() {
                                        Ok(o) => {
                                            match self.maps.goobermap.get_mut(&self.user) {
                                                Some(value) => {
                                                    *value += o;
                                                }
                                                None => {
                                                    self.err = format!(
                                                        "User '{}' not found; Something went wrong.",
                                                        &self.user
                                                    );
                                                    //ui.label(&self.err);
                                                    std::process::exit(-1);
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                        }
                                    }
                                }
                            });

                            ui.vertical(|ui| {
                                ui.text_edit_singleline(&mut self.opt);
                                if ui.button("Subtract").clicked() {
                                    match self.opt.trim().parse::<i32>() {
                                        Ok(o) => {
                                            match self.maps.goobermap.get_mut(&self.user) {
                                                Some(value) => {
                                                    *value -= o;
                                                }
                                                None => {
                                                    self.err = format!(
                                                        "User '{}' not found; Something went wrong. Exiting...",
                                                        &self.user
                                                    );
                                                    //ui.label(&self.err);
                                                    std::process::exit(-1);
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                        }
                                    }
                                }
                            });

                            ui.vertical(|ui| {
                                ui.text_edit_singleline(&mut self.opt);
                                if ui.button("Set").clicked() {
                                    match self.opt.trim().parse::<i32>() {
                                        Ok(o) => {
                                            match self.maps.goobermap.get_mut(&self.user) {
                                                Some(value) => {
                                                    *value = o;
                                                }
                                                None => {
                                                    self.err = format!(
                                                        "User '{}' not found; Something went wrong. Exiting...",
                                                        &self.user
                                                    );
                                                    //ui.label(&self.err);
                                                    std::process::exit(-1);
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                        }
                                    }
                                }
                            });
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui.button("Close").clicked() {self.goober_pop = false; /*self.dg = String::new();*/}
                        });

                        ui.label(&self.pointsgoober);
                        ui.label(&self.given);
                        ui.label(&self.errs);
                    });
            }

            if self.duck_pop {
                egui::Window::new("Duck Point System").default_size(egui::Vec2::new(600.0, 400.0))
                    .show(ctx, |ui| {
                        ui.label("Which point value would you like to add?");

                        egui::ComboBox::from_label("")
                            .selected_text(&self.duckoption)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.duckoption, "R".to_string(), "Raider Killed");
                                ui.selectable_value(&mut self.duckoption, "RB".to_string(), "Rule Breaker");
                                ui.selectable_value(&mut self.duckoption, "CTO".to_string(), "Corpse Thrown Out");
                                ui.selectable_value(&mut self.duckoption, "PHD".to_string(), "People Healed/Defibrillated");
                                ui.selectable_value(&mut self.duckoption, "PEO".to_string(), "People Experimented On");
                                ui.selectable_value(&mut self.duckoption, "P".to_string(), "Playtime");
                                ui.selectable_value(&mut self.duckoption, "MA".to_string(), "Managing Airlock");
                                ui.selectable_value(&mut self.duckoption, "MMD".to_string(), "Model Made for DSF");
                                ui.selectable_value(&mut self.duckoption, "BUS".to_string(), "Blacklisted Users Sniped");
                                ui.selectable_value(&mut self.duckoption, "EGTC".to_string(), "Every Guard Training Complete");
                                ui.selectable_value(&mut self.duckoption, "B".to_string(), "Bonus");
                            });
                        let mut numb = 0;

                        match self.maps.duckmap.get(&self.user) {
                            Some(/*value*/_) => { /*println!("User found: {}, with {} points", self.user, value);*/ },
                            None => { self.maps.duckmap.insert(self.user.to_string().to_lowercase(), 0); }
                        };
                        match self.duckoption.to_lowercase().as_str() {
                            "r" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of raiders killed:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 4 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.errs);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "rb" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of rule breakers killed:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 2 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "cto" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of corpses thrown out:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            if num > 10 {numb = 10;}
                                            numb = 1 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "p" => {
                                egui::ComboBox::from_label("Time unit")
                                    .selected_text(&self.option2)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.option2, "Hours".to_string(), "Hours");
                                        ui.selectable_value(&mut self.option2, "Minutes".to_string(), "Minutes");
                                    });
                                ui.vertical(|ui| {
                                    if self.option2 == "Hours" {
                                        ui.label("Number of hours played on shift:");
                                        ui.text_edit_singleline(&mut self.option);
                                    } else if self.option2 == "Minutes" {
                                        ui.label("Number of minutes played on shift:");
                                        ui.text_edit_singleline(&mut self.number);

                                        if let Ok(mins) = self.number.trim().parse::<f32>() {
                                            let hours = mins.round() as i32;
                                            self.option = format!("{}", hours);
                                        } else {
                                            self.err = "Please enter a valid number.".to_string();
                                        }
                                    }
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: f64 = match self.option.trim().parse::<f64>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            if self.option2 == "Hours" {
                                                numb = (10f64 * num) as i32;
                                                *value += numb;
                                            } else if self.option2 == "Minutes" {
                                                numb = (0.1667 * num).round() as i32;
                                                *value += numb;
                                            }
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "peo" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of people experimented on:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 2 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }

                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "phd" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of people healed/defibrillated:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            if num > 10 {numb = 10;}
                                            numb = 1 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "ma" => {
                                egui::ComboBox::from_label("Time unit")
                                    .selected_text(&self.option2)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.option2, "Hours".to_string(), "Hours");
                                        ui.selectable_value(&mut self.option2, "Minutes".to_string(), "Minutes");
                                    });
                                ui.vertical(|ui| {
                                    if self.option2 == "Hours" {
                                        ui.label("Number of hours played on shift:");
                                        ui.text_edit_singleline(&mut self.option);
                                    } else if self.option2 == "Minutes" {
                                        ui.label("Number of minutes played on shift:");
                                        ui.text_edit_singleline(&mut self.number);

                                        if let Ok(mins) = self.number.trim().parse::<f32>() {
                                            let hours = mins.round() as i32;
                                            self.option = format!("{}", hours);
                                        } else {
                                            self.err = "Please enter a valid number.".to_string();
                                        }
                                    }
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: f64 = match self.option.trim().parse::<f64>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            if self.option2 == "Hours" {
                                                numb = (10.0 * num) as i32;
                                                *value += numb;
                                            } else if self.option2 == "Minutes" {
                                                numb = (0.25 * num).round() as i32;
                                                *value += numb;
                                            }
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "mmd" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of models made for the DSF:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 10 * num;
                                            *value -= numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "bus" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of blacklisted users sniped:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 6 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "egtc" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of guard trainings complete:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 20 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            //ui.label(&self.err);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            "b" => {
                                ui.vertical(|ui| {
                                    ui.label("Number of points for bonus:");
                                    ui.text_edit_singleline(&mut self.option);
                                });

                                if ui.button("Confirm").clicked() {
                                    let num: i32 = match self.option.trim().parse::<i32>() {
                                        Ok(n) => n,
                                        Err(_) => {
                                            self.err = "Invalid input; please enter a number.".to_string();
                                            return;
                                        }
                                    };

                                    let username = self.user.to_lowercase();

                                    match self.maps.duckmap.get_mut(&username) {
                                        Some(value) => {
                                            numb = 1 * num;
                                            *value += numb;
                                        }
                                        None => {
                                            self.err = format!("User '{}' not found; Something went wrong. Exiting...", username);
                                            std::process::exit(-1);
                                        }
                                    }
                                    self.given = format!("{} has been given {} points.", self.user, numb);
                                }
                            },
                            _ => {}
                        }

                        self.check = format!("Override {}", self.user);
                        ui.checkbox(&mut self.duckcheck, &self.check);
                        if self.duckcheck {
                            ui.label("Override options:");
                            ui.separator();

                            ui.vertical(|ui| {
                                ui.text_edit_singleline(&mut self.opt);
                                if ui.button("Add").clicked() {
                                    match self.opt.trim().parse::<i32>() {
                                        Ok(o) => {
                                            match self.maps.duckmap.get_mut(&self.user) {
                                                Some(value) => {
                                                    *value += o;
                                                }
                                                None => {
                                                    self.err = format!(
                                                        "User '{}' not found; Something went wrong.",
                                                        &self.user
                                                    );
                                                    //ui.label(&self.err);
                                                    std::process::exit(-1);
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            self.err = "Invalid input: Please enter a valid number.".to_string();
                                        }
                                    }
                                }
                            });

                            ui.vertical(|ui| {
                                ui.text_edit_singleline(&mut self.opt);
                                if ui.button("Subtract").clicked() {
                                    match self.opt.trim().parse::<i32>() {
                                        Ok(o) => {
                                            match self.maps.duckmap.get_mut(&self.user) {
                                                Some(value) => {
                                                    *value -= o;
                                                }
                                                None => {
                                                    self.err = format!(
                                                        "User '{}' not found; Something went wrong. Exiting...",
                                                        &self.user
                                                    );
                                                    //ui.label(&self.err);
                                                    std::process::exit(-1);
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            self.err = "Invalid input: Please enter a valid number.".to_string();
                                        }
                                    }
                                }
                            });

                            ui.vertical(|ui| {
                                ui.text_edit_singleline(&mut self.opt);
                                if ui.button("Set").clicked() {
                                    match self.opt.trim().parse::<i32>() {
                                        Ok(o) => {
                                            match self.maps.duckmap.get_mut(&self.user) {
                                                Some(value) => {
                                                    *value = o;
                                                }
                                                None => {
                                                    self.err = format!(
                                                        "User '{}' not found; Something went wrong. Exiting...",
                                                        &self.user
                                                    );
                                                    //ui.label(&self.err);
                                                    std::process::exit(-1);
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            self.err = "Invalid input: Please enter a valid number.".to_string();
                                        }
                                    }
                                }
                            });
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui.button("Close").clicked() {self.duck_pop = false; /*self.dg = String::new();*/}
                        });

                        ui.label(&self.pointsduck);
                        ui.label(&self.given);
                        ui.label(&self.errs);
                    });
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                if ui.button("Close App").clicked() {
                    std::process::exit(0);
                } else {}
            });
        });
    }
}