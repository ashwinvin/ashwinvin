use std::collections::HashMap;

use eframe::egui::{CollapsingHeader, CtxRef, Ui, Window};

pub struct Project {
    name: String,
    details: HashMap<String, (String, String)>,
}

pub struct ProjectWindow {
    projects: Vec<Project>,
}

impl Default for ProjectWindow {
    fn default() -> Self {
        let projects = vec![Project {
            name: "Visionlib".to_owned(),
            details: HashMap::from([
                (
                    "Documentation:".to_owned(),
                    (
                        "Hosted on github pages".to_owned(),
                        "https://ashwinvin.github.io/Visionlib/".to_owned(),
                    ),
                ),
                (
                    "Source:".to_owned(),
                    (
                        "Available on Github".to_owned(),
                        "https://ashwinvin.github.io/Visionlib/".to_owned(),
                    ),
                ),
                (
                    "Package:".to_owned(),
                    (
                        "Pypi".to_owned(),
                        "https://pypi.org/project/visionlib/".to_owned(),
                    ),
                ),
            ]),
        }];

        Self { projects }
    }
}

impl ProjectWindow {
    fn add_projects(&self, ui: &mut Ui) {
        for project in &self.projects {
            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(&project.name);
                    ui.separator();
                    ui.label("A simple, easy to use, feature rich, customizable cv library");
                    CollapsingHeader::new("Stats").show(ui, |ui| {
                        ui.label("Stars: ");
                        ui.label("Forks: ");
                        ui.label("Watchers: ");
                    });
                    CollapsingHeader::new("Details").show(ui, |ui| {
                        for (heading, details) in project.details.iter() {
                            ui.horizontal(|ui| {
                                ui.label(heading);
                                ui.hyperlink_to(&details.0, &details.1)
                            });
                        }
                    });
                });
            });
        }
    }
}

impl crate::Window for ProjectWindow {
    fn name(&self) -> &'static str {
        "Projects"
    }

    fn show(&self, ctx: &CtxRef, state: &mut bool) {
        Window::new(self.name())
            .open(state)
            .resizable(false)
            .show(ctx, |ui| {
                self.add_projects(ui);
                ui.small("More coming soon!");
            });
    }
}
