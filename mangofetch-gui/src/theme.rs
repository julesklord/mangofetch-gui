//! Theming: MonolithUI → egui visuals
//! Centralized design token system for the entire application.
//! ALL colors, font sizes, and spacing values live here. Never hardcode elsewhere.

use egui::{Color32, Stroke};

// ══════════════════════════════════════════════════════════════════════════════
// DESIGN TOKENS — Single source of truth
// ══════════════════════════════════════════════════════════════════════════════

/// Text color tokens
pub struct MonoText;
impl MonoText {
    /// Primary text — white for headings and important content
    pub const PRIMARY: Color32 = Color32::from_rgb(0xf0, 0xf0, 0xf2);
    /// Secondary text — body copy, descriptions
    pub const SECONDARY: Color32 = Color32::from_rgb(0xd1, 0xd5, 0xdb);
    /// Tertiary text — labels, captions, subtle info
    pub const TERTIARY: Color32 = Color32::from_rgb(0x9c, 0xa3, 0xaf);
    /// Muted text — placeholders, disabled states
    pub const MUTED: Color32 = Color32::from_rgb(0x6b, 0x72, 0x80);
    /// Ghost text — toolbar labels, very subtle metadata
    pub const GHOST: Color32 = Color32::from_rgb(0x4a, 0x54, 0x68);
    /// Chrome text — status bar, system info
    pub const CHROME: Color32 = Color32::from_rgb(0x55, 0x5f, 0x72);
}

/// Semantic color tokens
pub struct MonoSemantics;
impl MonoSemantics {
    /// Success — active downloads, completions, online status
    pub const SUCCESS: Color32 = Color32::from_rgb(0x34, 0xa8, 0x53);
    /// Success muted — success backgrounds at low opacity
    pub fn success_bg() -> Color32 {
        with_alpha(Self::SUCCESS, 0.12)
    }
    /// Success border
    pub fn success_border() -> Color32 {
        with_alpha(Self::SUCCESS, 0.30)
    }

    /// Warning — paused, queued, attention needed
    pub const WARNING: Color32 = Color32::from_rgb(0xfb, 0xbf, 0x24);
    /// Warning muted — warning backgrounds at low opacity
    pub fn warning_bg() -> Color32 {
        with_alpha(Self::WARNING, 0.12)
    }

    /// Danger — errors, failed downloads, destructive actions
    pub const DANGER: Color32 = Color32::from_rgb(0xf2, 0x8b, 0x82);
    /// Danger muted — error backgrounds at low opacity
    pub fn danger_bg() -> Color32 {
        with_alpha(Self::DANGER, 0.08)
    }
    /// Danger border
    pub fn danger_border() -> Color32 {
        with_alpha(Self::DANGER, 0.25)
    }

    /// Separator — 1px panel dividers (white at 7% opacity)
    pub fn separator() -> Color32 {
        Color32::from_rgba_unmultiplied(255, 255, 255, 18)
    }
    /// Separator stroke
    pub fn separator_stroke() -> Stroke {
        Stroke::new(1.0_f32, Self::separator())
    }
}

/// Surface ramp — 8 levels from deepest to lightest.
/// Tinted with a very subtle cool undertone for visual richness.
pub struct MonolithSurfaces;
impl MonolithSurfaces {
    pub const SURFACE_0: Color32 = Color32::from_rgb(0x08, 0x08, 0x0a); // deepest — sunken wells
    pub const SURFACE_1: Color32 = Color32::from_rgb(0x10, 0x10, 0x12); // sidebar, nav, toolbar
    pub const SURFACE_2: Color32 = Color32::from_rgb(0x18, 0x18, 0x1a); // panels, log terminal bg
    pub const SURFACE_3: Color32 = Color32::from_rgb(0x22, 0x22, 0x24); // root canvas, input bg
    pub const SURFACE_4: Color32 = Color32::from_rgb(0x2a, 0x2a, 0x2d); // cards, controls
    pub const SURFACE_5: Color32 = Color32::from_rgb(0x34, 0x34, 0x37); // hover states
    pub const SURFACE_6: Color32 = Color32::from_rgb(0x40, 0x40, 0x44); // active hover, borders
    pub const SURFACE_7: Color32 = Color32::from_rgb(0x4c, 0x4c, 0x50); // elevated elements

    /// Active tab fill — slightly elevated from base
    pub const TAB_ACTIVE: Color32 = Color32::from_rgb(0x26, 0x26, 0x28);
}

/// Typography scale tokens (in points, for egui FontId)
pub struct MonoType;
impl MonoType {
    // Font sizes — proportional (Outfit)
    pub const DISPLAY: f32 = 22.0; // About title, big headings
    pub const HEADING: f32 = 17.0; // Section headers, sidebar brand
    pub const SUBHEADING: f32 = 15.0; // Card titles, nav items
    pub const BODY: f32 = 13.5; // Default body text
    pub const LABEL: f32 = 12.0; // Form labels, descriptions
    pub const CAPTION: f32 = 11.0; // Small labels, metadata
    pub const MICRO: f32 = 10.0; // Toolbar, status bar, pills

    // Font sizes — monospace (DM Mono)
    pub const MONO_DATA: f32 = 11.0; // Table data, IDs, logs
    pub const MONO_SMALL: f32 = 10.0; // Status bar, toolbar labels
    pub const MONO_MICRO: f32 = 9.5; // Speed indicators, tiny data
}

/// Spacing tokens
pub struct MonoSpace;
impl MonoSpace {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 20.0;
    pub const XXL: f32 = 24.0;
    pub const XXXL: f32 = 32.0;
}

/// Layout dimension tokens
pub struct MonoLayout;
impl MonoLayout {
    pub const SIDEBAR_WIDTH: f32 = 200.0;
    pub const TOOLBAR_HEIGHT: f32 = 48.0;
    pub const STATUS_BAR_HEIGHT: f32 = 26.0;
    pub const CORNER_RADIUS_SM: u8 = 4;
    pub const CORNER_RADIUS_MD: u8 = 6;
    pub const CORNER_RADIUS_LG: u8 = 8;
    pub const SEPARATOR_THICKNESS: f32 = 1.0;
}

// ══════════════════════════════════════════════════════════════════════════════
// BRAND PRESETS — Switchable accent color themes
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrandPreset {
    PlasmCore,      // Cyan + Violet
    OxidizedGold,   // Amber
    VioletReaction, // Purple + Crimson
    CoolantLiquid,  // Cyan + Teal
    CriticalMass,   // Scarlet + Dark Red
}

impl BrandPreset {
    pub fn name(&self) -> &'static str {
        match self {
            BrandPreset::PlasmCore => "Plasma Core",
            BrandPreset::OxidizedGold => "Oxidized Gold",
            BrandPreset::VioletReaction => "Violet Reaction",
            BrandPreset::CoolantLiquid => "Coolant Liquid",
            BrandPreset::CriticalMass => "Critical Mass",
        }
    }

    pub fn primary(&self) -> Color32 {
        match self {
            BrandPreset::PlasmCore => hex_to_color32("#22d3ee"),
            BrandPreset::OxidizedGold => hex_to_color32("#f59e0b"),
            BrandPreset::VioletReaction => hex_to_color32("#a855f7"),
            BrandPreset::CoolantLiquid => hex_to_color32("#06b6d4"),
            BrandPreset::CriticalMass => hex_to_color32("#ef4444"),
        }
    }

    pub fn secondary(&self) -> Color32 {
        match self {
            BrandPreset::PlasmCore => hex_to_color32("#a855f7"),
            BrandPreset::OxidizedGold => hex_to_color32("#fbbf24"),
            BrandPreset::VioletReaction => hex_to_color32("#e11d48"),
            BrandPreset::CoolantLiquid => hex_to_color32("#2dd4bf"),
            BrandPreset::CriticalMass => hex_to_color32("#991b1b"),
        }
    }

    /// Brand primary at 12% opacity for subtle backgrounds
    pub fn primary_bg(&self) -> Color32 {
        with_alpha(self.primary(), 0.12)
    }
    /// Brand primary at 20% opacity for active states
    pub fn primary_active(&self) -> Color32 {
        with_alpha(self.primary(), 0.20)
    }
    /// Brand primary at 30% opacity for borders
    pub fn primary_border(&self) -> Color32 {
        with_alpha(self.primary(), 0.30)
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ══════════════════════════════════════════════════════════════════════════════

/// Converts hex string to Color32 (supports #RRGGBB)
pub fn hex_to_color32(hex: &str) -> Color32 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Color32::from_rgb(r, g, b)
}

/// Applies alpha to a color (0.0 = transparent, 1.0 = opaque)
pub fn with_alpha(color: Color32, alpha: f32) -> Color32 {
    let arr = color.to_array();
    Color32::from_rgba_unmultiplied(arr[0], arr[1], arr[2], (alpha * 255.0) as u8)
}

/// Interpolates between two colors by a factor (0.0 = color_a, 1.0 = color_b)
pub fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let ar = a.to_array();
    let br = b.to_array();
    Color32::from_rgb(
        (ar[0] as f32 + (br[0] as f32 - ar[0] as f32) * t) as u8,
        (ar[1] as f32 + (br[1] as f32 - ar[1] as f32) * t) as u8,
        (ar[2] as f32 + (br[2] as f32 - ar[2] as f32) * t) as u8,
    )
}

// ══════════════════════════════════════════════════════════════════════════════
// THEME APPLICATION
// ══════════════════════════════════════════════════════════════════════════════

/// Applies the MonolithUI dark theme to the egui context
pub fn apply_monolith_dark(ctx: &egui::Context, brand: BrandPreset) {
    let mut visuals = egui::Visuals::dark();

    // Surface fills
    visuals.window_fill = MonolithSurfaces::SURFACE_1;
    visuals.panel_fill = MonolithSurfaces::SURFACE_3;
    visuals.faint_bg_color = MonolithSurfaces::SURFACE_2;
    visuals.extreme_bg_color = MonolithSurfaces::SURFACE_0;

    let primary = brand.primary();

    // Widget: inactive
    visuals.widgets.inactive.bg_fill = MonolithSurfaces::SURFACE_4;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, MonoText::SECONDARY);
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(MonoLayout::CORNER_RADIUS_MD);
    visuals.widgets.inactive.weak_bg_fill = MonolithSurfaces::SURFACE_3;

    // Widget: hovered
    visuals.widgets.hovered.bg_fill = MonolithSurfaces::SURFACE_5;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, primary);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(MonoLayout::CORNER_RADIUS_MD);
    visuals.widgets.hovered.weak_bg_fill = MonolithSurfaces::SURFACE_5;

    // Widget: active (pressed)
    visuals.widgets.active.bg_fill = brand.primary_active();
    visuals.widgets.active.fg_stroke = Stroke::new(1.5_f32, primary);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(MonoLayout::CORNER_RADIUS_MD);
    visuals.widgets.active.weak_bg_fill = brand.primary_active();

    // Widget: noninteractive (labels, static text)
    visuals.widgets.noninteractive.bg_fill = MonolithSurfaces::SURFACE_3;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, MonoText::SECONDARY);
    visuals.widgets.noninteractive.corner_radius =
        egui::CornerRadius::same(MonoLayout::CORNER_RADIUS_MD);

    // Selection
    visuals.selection.bg_fill = brand.primary_bg();
    visuals.selection.stroke = Stroke::new(1.0_f32, primary);

    // Hyperlinks
    visuals.hyperlink_color = primary;

    // Separator
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, MonoSemantics::separator());

    ctx.set_visuals(visuals);
}

/// Loads system fonts based on the user's graphical environment.
/// Falls back to egui's built-in defaults if no suitable system font is found.
pub fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Try to find a system proportional (sans-serif) font
    let proportional_candidates = [
        "sans-serif",
        "Arial",
        "Helvetica",
        "Liberation Sans",
        "Noto Sans",
        "DejaVu Sans",
        "Ubuntu",
        "Cantarell",
        "Inter",
        "Segoe UI",
        "San Francisco",
        "PingFang SC",
    ];

    // Try to find a system monospace font
    let monospace_candidates = [
        "monospace",
        "Courier New",
        "Liberation Mono",
        "Noto Sans Mono",
        "DejaVu Sans Mono",
        "Ubuntu Mono",
        "Roboto Mono",
        "Cascadia Code",
        "JetBrains Mono",
        "Fira Code",
        "SF Mono",
        "Menlo",
    ];

    let mut loaded_proportional = false;
    let mut loaded_monospace = false;

    for family in &proportional_candidates {
        if loaded_proportional {
            break;
        }
        let property = font_loader::system_fonts::FontPropertyBuilder::new()
            .family(family)
            .build();
        if let Some((data, _index)) = font_loader::system_fonts::get(&property) {
            fonts.font_data.insert(
                "system_proportional".to_owned(),
                std::sync::Arc::new(egui::FontData::from_owned(data)),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "system_proportional".to_owned());
            loaded_proportional = true;
            tracing::info!("Loaded system proportional font: {}", family);
        }
    }

    for family in &monospace_candidates {
        if loaded_monospace {
            break;
        }
        let property = font_loader::system_fonts::FontPropertyBuilder::new()
            .family(family)
            .build();
        if let Some((data, _index)) = font_loader::system_fonts::get(&property) {
            fonts.font_data.insert(
                "system_monospace".to_owned(),
                std::sync::Arc::new(egui::FontData::from_owned(data)),
            );
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, "system_monospace".to_owned());
            loaded_monospace = true;
            tracing::info!("Loaded system monospace font: {}", family);
        }
    }

    if !loaded_proportional {
        tracing::warn!("No system proportional font found, using egui defaults");
    }
    if !loaded_monospace {
        tracing::warn!("No system monospace font found, using egui defaults");
    }

    ctx.set_fonts(fonts);
}
