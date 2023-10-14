use iced::widget::Button;
use iced::widget::MouseArea;
use iced::widget::ProgressBar;
use iced::widget::Text;
use iced::window::{close, Settings};

use tracing;

const APPLICATION_ID: &str = "org.uncomfy.twentyfiveandfive";

#[cfg(target_os = "linux")]
pub fn settings() -> Settings {
    use iced::window;

    Settings {
        platform_specific: window::PlatformSpecific {
            application_id: APPLICATION_ID.to_string(),
        },
        size: (540, 540),
        max_size: Some((720, 720)),
        min_size: Some((300, 300)),
        ..Default::default()
    }
}
