#![windows_subsystem = "windows"]

fn main() -> iced::Result {
    uad_gui::setup_logger().expect("setup logging");
    uad_gui::gui::UadGui::start()
}
