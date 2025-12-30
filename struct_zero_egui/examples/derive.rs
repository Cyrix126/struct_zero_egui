use struct_zero_egui_derive::EguiDisplay;

fn main() {}
#[derive(EguiDisplay)]
#[egui_display(title = "[Node]")]
#[egui_display(hover_enabled = "The Node is online")]
#[egui_display(hover_disabled = "The Node is offline")]
pub struct GuiNodeApi {
    #[egui(spacing = 1.2)]
    #[egui(hover = HOVER_UPTIME)]
    pub uptime: String,
    pub blockheight: String,
    pub difficulty: String,
    pub database_size: String,
    pub free_space: String,
    pub nettype: String,
    pub outgoing_connections: String,
    pub incoming_connections: String,
    pub status: String,
    pub synchronized: String,
}
const HOVER_UPTIME: &str = "Since when the service was started";
