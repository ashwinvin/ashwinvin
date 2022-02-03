use eframe::egui::{CtxRef, Window};

#[derive(Default)]
pub struct UserInfo;

impl crate::Window for UserInfo {
    fn name(&self) -> &'static str {
        "📚 About Me"
    }

    fn show(&self, ctx: &CtxRef, state: &mut bool) {
        Window::new(self.name())
            .open(state)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label(
                            "Hey I'm Ashwin Vinod, a teenager with \
                     a passion for computers and an ability to learn things quickly. I have been \
                     in the development scene for the past 3 years and I spend most of the time \
                     expanding and improving my current skillset.",
                    );
                    ui.separator();
                    ui.label(
                            "I'm proficient in python and is currently \
                     learning rust. I also have experience in administrating linux systems, \
                     postgresql, mysql, building discord bots and REST APIs.",
                     );
                });
            });
    }
}
