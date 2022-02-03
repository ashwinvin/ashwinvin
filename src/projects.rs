use eframe::egui::{CtxRef, Grid, Layout, Ui, WidgetText, Window, CollapsingHeader};

#[derive(Default)]
pub struct Projects;

//     "Documentation:",
//     "Hosted on github pages",
//     "https://ashwinvin.github.io/Visionlib/",


//     "Source:",
//     "Available on Github",
//     "https://ashwinvin.github.io/Visionlib/",


//     "Package:",
//     "Pypi",
//     "https://pypi.org/project/visionlib/",


impl crate::Window for Projects {
    fn name(&self) -> &'static str {
        "Projects"
    }

    fn show(&self, ctx: &CtxRef, state: &mut bool) {
        Window::new(self.name())
            .open(state)
            .resizable(false)
            .show(ctx, |ui| {
                ui.group(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Visionlib");
                        ui.separator();
                        ui.label("A simple, easy to use, feature rich, customizable cv library");
                        // CollapsingHeader::new()
                    });
                });
            });
    }
}
