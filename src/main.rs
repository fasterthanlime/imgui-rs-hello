extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;

use imgui::*;

mod support;

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

fn main() {
    support::run("imgui-rs-hello".to_owned(), CLEAR_COLOR, hello_world);
}

fn hello_world<'a>(ui: &Ui<'a>) -> bool {
    ui.window(im_str!("itch-lite"))
        .size((300.0, 100.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text(im_str!("Electron haters unite!"));
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos.0,
                mouse_pos.1
            ));
        });
    true
}
