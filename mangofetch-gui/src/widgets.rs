//! Custom reusable MonolithUI widgets for mangofetch-gui
//! All visual tokens come from theme.rs — no hardcoded colors here.

use crate::theme::{
    with_alpha, MonoLayout, MonoSemantics, MonoSpace, MonoText, MonoType, MonolithSurfaces,
};
use egui::{Color32, CornerRadius, FontFamily, FontId, Frame, Margin, RichText, Stroke, Ui, Vec2};

/// Renders a MonolithUI surface card with elevation and subtle border.
pub fn surface_card<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::NONE
        .fill(MonolithSurfaces::SURFACE_4)
        .inner_margin(Margin::same(MonoSpace::LG as i8))
        .corner_radius(CornerRadius::same(MonoLayout::CORNER_RADIUS_MD))
        .stroke(Stroke::new(1.0_f32, MonolithSurfaces::SURFACE_6))
        .show(ui, add_contents)
        .inner
}

/// Renders a sunken "well" for recessed inputs, terminals, or data areas.
pub fn sunken_well<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::NONE
        .fill(MonolithSurfaces::SURFACE_0)
        .inner_margin(Margin::same(MonoSpace::MD as i8))
        .corner_radius(CornerRadius::same(MonoLayout::CORNER_RADIUS_SM))
        .stroke(Stroke::new(1.0_f32, MonolithSurfaces::SURFACE_3))
        .show(ui, add_contents)
        .inner
}

/// Status indicator dot with semantic coloring.
/// Active/Complete/Online: green with soft glow
/// Queued/Warning/Paused: amber
/// Error/Default: danger red
pub fn status_dot(ui: &mut Ui, status: &str) {
    let (color, has_glow) = match status {
        "Active" | "Complete" | "Online" => (MonoSemantics::SUCCESS, true),
        "Queued" | "Warning" | "Paused" => (MonoSemantics::WARNING, false),
        _ => (MonoSemantics::DANGER, false),
    };

    let (rect, _) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), egui::Sense::hover());
    let painter = ui.painter();

    // Glow ring for active states
    if has_glow {
        painter.circle_filled(rect.center(), 6.5, with_alpha(color, 0.20));
    }

    // Core dot
    painter.circle_filled(rect.center(), 4.0, color);
}

/// MonolithUI pill badge with tinted background and brand-colored text.
pub fn brand_pill(ui: &mut Ui, text: &str, color: Color32) {
    Frame::NONE
        .fill(with_alpha(color, 0.08))
        .stroke(Stroke::new(1.0_f32, with_alpha(color, 0.22)))
        .inner_margin(Margin::symmetric(8, 3))
        .corner_radius(CornerRadius::same(MonoLayout::CORNER_RADIUS_SM))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text)
                    .color(color)
                    .font(FontId::new(MonoType::MICRO, FontFamily::Monospace)),
            );
        });
}

/// Platform-branded tag with official color and Nerd Font glyph.
pub fn platform_pill(ui: &mut Ui, platform: &str) {
    let color = match platform.to_lowercase().as_str() {
        "youtube" => Color32::from_rgb(255, 85, 85),
        "instagram" => Color32::from_rgb(255, 120, 200),
        "tiktok" => Color32::from_rgb(85, 255, 255),
        "twitch" => Color32::from_rgb(180, 100, 255),
        "torrent" => Color32::from_rgb(85, 255, 120),
        "bluesky" => Color32::from_rgb(96, 165, 250),
        "reddit" => Color32::from_rgb(255, 69, 0),
        "pinterest" => Color32::from_rgb(230, 0, 35),
        "vimeo" => Color32::from_rgb(26, 180, 255),
        "bilibili" => Color32::from_rgb(0, 161, 214),
        "twitter" | "x" => Color32::from_rgb(29, 161, 242),
        _ => Color32::from_rgb(168, 85, 247),
    };

    let glyph = match platform.to_lowercase().as_str() {
        "youtube" => "󰗃",
        "instagram" => "󰅟",
        "tiktok" => "󰓳",
        "twitch" => "󰓓",
        "torrent" => "󰄗",
        "bluesky" => "󰖟",
        "reddit" => "󰑙",
        "pinterest" => "󰏖",
        "vimeo" => "󰨈",
        "bilibili" => "󰅜",
        "twitter" | "x" => "󰇩",
        _ => "󰈚",
    };

    let text = format!("{}  {}", glyph, platform.to_uppercase());
    brand_pill(ui, &text, color);
}

/// Section header title with MonolithUI typography scale.
pub fn section_header(ui: &mut Ui, title: &str) {
    ui.vertical(|ui| {
        ui.add_space(MonoSpace::XS);
        ui.label(
            RichText::new(title)
                .font(FontId::new(MonoType::HEADING, FontFamily::Proportional))
                .strong()
                .color(MonoText::PRIMARY),
        );
        ui.add_space(MonoSpace::SM);
    });
}

/// Compact section header for sub-sections within cards.
pub fn sub_header(ui: &mut Ui, title: &str) {
    ui.label(
        RichText::new(title)
            .font(FontId::new(MonoType::SUBHEADING, FontFamily::Proportional))
            .strong()
            .color(MonoText::PRIMARY),
    );
}

/// Styled action button with brand color fill.
pub fn primary_button(
    ui: &mut Ui,
    label: &str,
    width: f32,
    height: f32,
    brand_color: Color32,
) -> bool {
    let btn = egui::Button::new(
        RichText::new(label)
            .strong()
            .color(Color32::BLACK)
            .font(FontId::new(MonoType::LABEL, FontFamily::Proportional)),
    )
    .fill(brand_color)
    .min_size(egui::vec2(width, height));

    ui.add(btn).clicked()
}

/// Ghost/secondary button with subtle border.
pub fn ghost_button(ui: &mut Ui, label: &str, brand_color: Color32) -> bool {
    let btn = egui::Button::new(
        RichText::new(label)
            .strong()
            .color(brand_color)
            .font(FontId::new(MonoType::LABEL, FontFamily::Proportional)),
    )
    .fill(Color32::TRANSPARENT)
    .stroke(Stroke::new(1.0_f32, with_alpha(brand_color, 0.40)));

    ui.add(btn).clicked()
}

/// Icon button (for toolbar and table actions) — renders a single glyph.
pub fn icon_button(ui: &mut Ui, icon: &str, tooltip: &str, color: Color32) -> bool {
    let btn = egui::Button::new(
        RichText::new(icon)
            .font(FontId::new(MonoType::BODY, FontFamily::Proportional))
            .color(color),
    )
    .fill(Color32::TRANSPARENT);

    ui.add(btn).on_hover_text(tooltip).clicked()
}

/// Error banner — inline error message with danger styling.
pub fn error_banner(ui: &mut Ui, message: &str) {
    Frame::NONE
        .fill(MonoSemantics::danger_bg())
        .stroke(Stroke::new(1.0_f32, MonoSemantics::danger_border()))
        .inner_margin(Margin::same(MonoSpace::MD as i8))
        .corner_radius(CornerRadius::same(MonoLayout::CORNER_RADIUS_MD))
        .show(ui, |ui| {
            ui.label(
                RichText::new(message)
                    .color(MonoSemantics::DANGER)
                    .font(FontId::new(MonoType::BODY, FontFamily::Proportional)),
            );
        });
}

/// Info banner — neutral information with subtle styling.
pub fn info_banner(ui: &mut Ui, message: &str) {
    Frame::NONE
        .fill(with_alpha(MonolithSurfaces::SURFACE_5, 0.5))
        .stroke(Stroke::new(1.0_f32, MonolithSurfaces::SURFACE_6))
        .inner_margin(Margin::same(MonoSpace::MD as i8))
        .corner_radius(CornerRadius::same(MonoLayout::CORNER_RADIUS_MD))
        .show(ui, |ui| {
            ui.label(
                RichText::new(message)
                    .color(MonoText::SECONDARY)
                    .font(FontId::new(MonoType::BODY, FontFamily::Proportional)),
            );
        });
}

/// Loading skeleton placeholder — animated shimmer for content loading.
pub fn loading_skeleton(ui: &mut Ui, width: f32, height: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
    let painter = ui.painter();

    // Base fill
    painter.rect_filled(
        rect,
        CornerRadius::same(MonoLayout::CORNER_RADIUS_SM),
        MonolithSurfaces::SURFACE_5,
    );

    // Shimmer highlight (subtle diagonal gradient effect using 2 overlapping rects)
    let shimmer_alpha = 0.06;
    let mid = rect.center().x;
    let shimmer_rect = egui::Rect::from_min_max(
        egui::pos2(mid - rect.width() * 0.3, rect.min.y),
        egui::pos2(mid + rect.width() * 0.3, rect.max.y),
    );
    painter.rect_filled(
        shimmer_rect,
        CornerRadius::same(MonoLayout::CORNER_RADIUS_SM),
        with_alpha(Color32::WHITE, shimmer_alpha),
    );
}
