use struct_zero_egui::{EguiDisplay, FieldParams, TitleParams};

fn main() {}
pub struct Node {
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
impl EguiDisplay for Node {
    fn title(&self) -> Option<TitleParams<'static>> {
        Some(TitleParams {
            value: "[Node]",
            hover_enabled: Some("The Node is online"),
            hover_disabled: Some("The Node is offline"),
        })
    }
    fn fields(&self) -> impl Iterator<Item = FieldParams<'_>> {
        [
            FieldParams {
                name: "Uptime",
                value: &self.uptime,
                hover: None,
                spacing: Some(1.2f32),
            },
            FieldParams {
                name: "Blockheight",
                value: &self.blockheight,
                hover: None,
                spacing: None,
            },
            FieldParams {
                name: "Difficulty",
                value: &self.difficulty,
                hover: None,
                spacing: None,
            },
            FieldParams {
                name: "Database Size",
                value: &self.database_size,
                hover: None,
                spacing: None,
            },
            FieldParams {
                name: "Free Space",
                value: &self.free_space,
                hover: None,
                spacing: None,
            },
            FieldParams {
                name: "Nettype",
                value: &self.nettype,
                hover: None,
                spacing: None,
            },
            FieldParams {
                name: "Outgoing Connections",
                value: &self.outgoing_connections,
                hover: None,
                spacing: None,
            },
            FieldParams {
                name: "Incoming Connections",
                value: &self.incoming_connections,
                hover: None,
                spacing: None,
            },
            FieldParams {
                name: "Status",
                value: &self.status,
                hover: None,
                spacing: None,
            },
            FieldParams {
                name: "Synchronized",
                value: &self.synchronized,
                hover: None,
                spacing: None,
            },
        ]
        .into_iter()
    }
}
