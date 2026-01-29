use crate::ui::components::page_view::PageView;
use gpui::*;
use gpui_component::StyledExt;
use gpui_component::button::ButtonVariants;
use gpui_component::{
    Disableable, Icon, Sizable, Theme, badge::Badge, button::Button, h_flex, v_flex,
};

#[derive(Clone)]
struct StoredCredential {
    credential_id: String,
    user_id: String,
    user_name: String,
    user_display_name: String,
    rp_id: String,
    rp_name: Option<String>,
}

#[derive(Clone)]
struct FidoOptions {
    client_pin: bool,
}

#[derive(Clone)]
struct FidoInfo {
    versions: Vec<String>,
    options: FidoOptions,
    min_pin_length: u8,
    resident_keys: bool,
}

#[derive(Clone)]
struct DeviceState {
    connected: bool,
    has_fido: bool,
    unlocked: bool,
    fido_info: Option<FidoInfo>,
    credentials: Vec<StoredCredential>,
}

pub struct PasskeysView;

impl PasskeysView {
    pub fn build(theme: &Theme) -> impl IntoElement {
        // Mock Data
        let device = DeviceState {
            connected: true,
            has_fido: true,
            unlocked: true,
            fido_info: Some(FidoInfo {
                versions: vec!["FIDO2_1".to_string()],
                options: FidoOptions { client_pin: true },
                min_pin_length: 4,
                resident_keys: true,
            }),
            credentials: vec![
                StoredCredential {
                    credential_id: "0000".to_string(),
                    user_id: "u123".to_string(),
                    user_name: "j.doe".to_string(),
                    user_display_name: "Work Key".to_string(),
                    rp_id: "example.com".to_string(),
                    rp_name: None,
                },
                StoredCredential {
                    credential_id: "1111".to_string(),
                    user_id: "u456".to_string(),
                    user_name: "developer".to_string(),
                    user_display_name: "Cloud Console".to_string(),
                    rp_id: "cloud.example.com".to_string(),
                    rp_name: None,
                },
            ],
        };

        PageView::build(
            "Passkeys",
            "Manage your security PIN and the FIDO credentials (passkeys) stored on your device.",
            if !device.connected {
                Self::render_no_device(theme).into_any_element()
            } else if !device.has_fido {
                Self::render_not_supported(theme).into_any_element()
            } else {
                v_flex()
                    .gap_6()
                    .child(Self::render_pin_management(&device, theme))
                    .child(Self::render_stored_passkeys(&device, theme))
                    .into_any_element()
            },
            theme,
        )
    }

    fn render_no_device(theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .h_64()
            .border_1()
            .border_color(theme.border)
            .rounded_xl()
            .child(
                div()
                    .text_color(theme.muted_foreground)
                    .child("Connect your pico-key to manage passkeys."),
            )
            .into_any_element()
    }

    fn render_not_supported(theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .h_64()
            .border_1()
            .border_color(theme.border)
            .rounded_xl()
            .child(
                div()
                    .text_color(theme.muted_foreground)
                    .child("FIDO Passkeys are not supported on this device."),
            )
            .into_any_element()
    }

    fn main_card(
        title: &str,
        icon_path: impl Into<SharedString>,
        description: &str,
        content: impl IntoElement,
        header_right: Option<impl IntoElement>,
        theme: &Theme,
    ) -> impl IntoElement {
        div()
            .w_full()
            .bg(rgb(0x18181b)) // Using the same dark bg as in HomeView
            .border_1()
            .border_color(theme.border)
            .rounded_xl()
            .p_6()
            .child(
                v_flex()
                    .gap_6()
                    .child(
                        h_flex()
                            .items_center()
                            .justify_between()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        h_flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                Icon::default()
                                                    .path(icon_path)
                                                    .size_5()
                                                    .text_color(theme.foreground),
                                            )
                                            .child(
                                                div()
                                                    .font_bold()
                                                    .text_color(theme.foreground)
                                                    .child(title.to_string()),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.muted_foreground)
                                            .child(description.to_string()),
                                    ),
                            )
                            .children(header_right),
                    )
                    .child(content),
            )
    }

    fn render_pin_management(device: &DeviceState, theme: &Theme) -> impl IntoElement {
        Self::main_card(
            "PIN Management",
            "icons/key.svg",
            "Configure FIDO2 PIN security",
            v_flex()
                .gap_4()
                .child(Self::render_pin_status_row(device, theme))
                .child(Self::render_min_pin_length_row(device, theme)),
            None::<AnyElement>,
            theme,
        )
    }

    fn render_pin_status_row(device: &DeviceState, theme: &Theme) -> impl IntoElement {
        let pin_set = device
            .fido_info
            .as_ref()
            .map_or(false, |f| f.options.client_pin);

        div()
            .flex()
            .items_center()
            .justify_between()
            .p_4()
            .border_1()
            .border_color(theme.border)
            .rounded_lg()
            .child(
                v_flex()
                    .child(div().font_medium().child("Current PIN Status"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.muted_foreground)
                            .child(if pin_set {
                                "PIN is set"
                            } else {
                                "No PIN configured"
                            }),
                    ),
            )
            .child(Button::new("change-pin-btn").outline().child(if pin_set {
                "Change PIN"
            } else {
                "Set PIN"
            }))
    }

    fn render_min_pin_length_row(device: &DeviceState, theme: &Theme) -> impl IntoElement {
        let min_len = device.fido_info.as_ref().map_or(4, |f| f.min_pin_length);
        let pin_set = device
            .fido_info
            .as_ref()
            .map_or(false, |f| f.options.client_pin);

        div()
            .flex()
            .items_center()
            .justify_between()
            .p_4()
            .border_1()
            .border_color(theme.border)
            .rounded_lg()
            .child(
                v_flex()
                    .child(div().font_medium().child("Minimum PIN Length"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.muted_foreground)
                            .child(format!("Current: {} characters", min_len)),
                    ),
            )
            .child(
                Button::new("update-pin-len-btn")
                    .outline()
                    .disabled(!pin_set)
                    .child("Update Minimum Length"),
            )
    }

    fn render_stored_passkeys(device: &DeviceState, theme: &Theme) -> impl IntoElement {
        Self::main_card(
            "Stored Passkeys",
            "icons/key-round.svg",
            "View and manage your resident credentials",
            if !device.unlocked {
                Self::render_locked_state(theme).into_any_element()
            } else {
                Self::render_unlocked_state(device, theme).into_any_element()
            },
            Some(
                div()
                    .child("View and manage your resident credentials")
                    .text_sm()
                    .text_color(theme.muted_foreground)
                    .invisible(),
            ),
            theme,
        )
    }

    fn render_locked_state(theme: &Theme) -> impl IntoElement {
        v_flex()
            .items_center()
            .justify_center()
            .gap_4()
            .py_8()
            .child(
                div().rounded_full().bg(theme.muted).p_4().child(
                    Icon::default()
                        .path("icons/shield.svg")
                        .size_8()
                        .text_color(theme.muted_foreground),
                ),
            )
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .child("Authentication Required"),
            )
            .child(
                div()
                    .text_color(theme.muted_foreground)
                    .text_sm()
                    .child("Unlock your device to view and manage passkeys."),
            )
            .child(
                Button::new("unlock-btn").child(
                    h_flex()
                        .gap_2()
                        .child(Icon::default().path("icons/lock-open.svg"))
                        .child("Unlock Storage"),
                ),
            )
    }

    fn render_unlocked_state(device: &DeviceState, theme: &Theme) -> impl IntoElement {
        v_flex()
            .gap_6()
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .child(
                        h_flex()
                            .gap_4()
                            .items_center()
                            .child(
                                Badge::new()
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .items_center()
                                            .child(
                                                Icon::default()
                                                    .path("icons/lock-open.svg")
                                                    .size_3p5(),
                                            )
                                            .child("Unlocked"),
                                    )
                                    .color(gpui::green()),
                            )
                            .child(div().w_px().h_4().bg(theme.border))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.muted_foreground)
                                    .child(format!(
                                        "{} credentials stored",
                                        device.credentials.len()
                                    )),
                            ),
                    )
                    .child(
                        Button::new("lock-storage-btn").outline().small().child(
                            h_flex()
                                .gap_2()
                                .child(Icon::default().path("icons/lock.svg").size_3p5())
                                .child("Lock Storage"),
                        ),
                    ),
            )
            .child(if device.credentials.is_empty() {
                Self::render_empty_credentials(theme).into_any_element()
            } else {
                div()
                    .grid()
                    .grid_cols(3)
                    .gap_4()
                    .children(
                        device
                            .credentials
                            .iter()
                            .map(|cred| Self::render_credential_card(cred, theme)),
                    )
                    .into_any_element()
            })
    }

    fn render_empty_credentials(theme: &Theme) -> impl IntoElement {
        v_flex()
            .items_center()
            .justify_center()
            .py_12()
            .border_1()
            .border_color(theme.border)
            .rounded_xl()
            .gap_4()
            .child(
                 div()
                    .rounded_full()
                    .bg(theme.muted)
                    .p_4()
                    .child(
                        Icon::default()
                            .path("icons/key-round.svg")
                            .size_8()
                            .text_color(theme.muted_foreground)
                    )
            )
             .child(div().text_lg().font_semibold().child("No Passkeys Found"))
            .child(
                div()
                    .text_color(theme.muted_foreground)
                    .text_sm()
                    .text_center()
                    .max_w(px(384.0))
                    .child("This device doesn't have any resident credentials stored yet. Create passkeys on websites to see them here.")
            )
    }

    fn render_credential_card(cred: &StoredCredential, theme: &Theme) -> impl IntoElement {
        div()
            .border_1()
            .border_color(theme.border)
            .rounded_xl()
            .p_4()
            .hover(|s| s.bg(theme.accent).border_color(theme.primary))
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .child(
                        h_flex()
                            .gap_3()
                            .items_center()
                            .child(
                                div()
                                    .size_10()
                                    .rounded_md()
                                    .bg(theme.secondary)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::default()
                                            .path("icons/key-round.svg")
                                            .text_color(theme.primary)
                                            .size_5(),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .child(
                                        div().font_semibold().child(
                                            cred.rp_name.clone().unwrap_or(cred.rp_id.clone()),
                                        ),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.muted_foreground)
                                            .child(cred.user_name.clone()),
                                    ),
                            ),
                    )
                    .child(
                        Button::new("delete-cred-btn").ghost().small().child(
                            Icon::default()
                                .path("icons/trash-2.svg")
                                .size_4()
                                .text_color(theme.muted_foreground),
                        ),
                    ),
            )
    }
}
