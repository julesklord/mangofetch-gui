//! MangoFetchApp core implementation conforming to the MonolithUI design system.
//! All visual tokens imported from theme.rs — zero hardcoded colors.

use crate::bridge::{CoreEvent, GuiCommand, MediaInfo, QueueItemInfo};
use crate::runtime::AppRuntime;
use crate::theme::*;
use crate::widgets::*;
use egui::{
    Align, Align2, Button, Color32, CornerRadius, FontFamily, FontId, Frame, Layout, Margin,
    ProgressBar, RichText, ScrollArea, Stroke, StrokeKind, Ui, Vec2,
};
use egui_extras::{Column, TableBuilder};

/// Active tabs in the orbital navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Home,
    Queue,
    Settings,
    Logs,
    About,
}

pub struct MangoFetchApp {
    runtime: AppRuntime,
    current_tab: Tab,
    theme: BrandPreset,

    // Core states
    items: Vec<QueueItemInfo>,
    logs: Vec<String>,
    ytdlp_installed: bool,
    ffmpeg_installed: bool,

    // Inputs & Forms
    input_url: String,
    output_dir: String,
    audio_only: bool,
    selected_quality: String,
    selected_video_format: String,
    selected_audio_format: String,
    selected_audio_quality: String,

    // Media Pre-fetch
    media_info_loading: bool,
    media_info: Option<MediaInfo>,
    media_info_error: Option<String>,
    thumbnail_texture: Option<egui::TextureHandle>,

    // Settings parameters
    concurrent_limit: usize,
    auto_retry: bool,
    show_persistent_logs: bool,

    // Telemetry
    sys: sysinfo::System,
    last_sys_refresh: std::time::Instant,

    // Layout
    top_nav_layout: bool,
}

impl MangoFetchApp {
    pub fn new(runtime: AppRuntime) -> Self {
        let default_output_dir = dirs::download_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "C:\\Downloads".to_string());

        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let app = Self {
            runtime,
            current_tab: Tab::Home,
            theme: BrandPreset::PlasmCore,
            items: Vec::new(),
            logs: Vec::new(),
            ytdlp_installed: false,
            ffmpeg_installed: false,
            input_url: String::new(),
            output_dir: default_output_dir,
            audio_only: false,
            selected_quality: "Best".to_string(),
            selected_video_format: "mp4".to_string(),
            selected_audio_format: "mp3".to_string(),
            selected_audio_quality: "320K".to_string(),
            media_info_loading: false,
            media_info: None,
            media_info_error: None,
            thumbnail_texture: None,
            concurrent_limit: 3,
            auto_retry: true,
            show_persistent_logs: false,
            sys,
            last_sys_refresh: std::time::Instant::now(),
            top_nav_layout: false,
        };

        let _ = app.runtime.send_command(GuiCommand::CheckDependencies);
        let _ = app.runtime.send_command(GuiCommand::RefreshQueue);

        app
    }

    fn drain_events(&mut self) {
        let events = self.runtime.drain_events();
        for event in events {
            match event {
                CoreEvent::QueueUpdated(queue_items) => {
                    self.items = queue_items;
                }
                CoreEvent::DownloadProgress {
                    id,
                    progress,
                    speed,
                    eta,
                } => {
                    if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
                        item.progress = progress;
                        item.speed = speed;
                        item.eta = eta;
                    }
                }
                CoreEvent::DownloadComplete { id, title } => {
                    if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
                        item.status = "Complete".to_string();
                        item.progress = 100.0;
                    }
                    self.logs
                        .push(format!("\u{2713} [{}] Completed successfully", title));
                }
                CoreEvent::DownloadError { id, error } => {
                    if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
                        item.status = "Error".to_string();
                        self.logs
                            .push(format!("\u{2717} [ID #{}] Error: {}", id, error));
                    }
                }
                CoreEvent::MediaInfoFetched(result) => {
                    self.media_info_loading = false;
                    self.thumbnail_texture = None;
                    match result {
                        Ok(info) => {
                            self.media_info = Some(info);
                            self.media_info_error = None;
                        }
                        Err(err) => {
                            self.media_info = None;
                            self.media_info_error = Some(err);
                        }
                    }
                }
                CoreEvent::DependencyStatus { ytdlp, ffmpeg } => {
                    self.ytdlp_installed = ytdlp;
                    self.ffmpeg_installed = ffmpeg;
                }
                CoreEvent::LogLine(line) => {
                    self.logs.push(line);
                    if self.logs.len() > 800 {
                        self.logs.remove(0);
                    }
                }
            }
        }
    }

    // ── Navigation ──────────────────────────────────────────────────────────

    fn render_top_nav(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_space(MonoSpace::LG);

            let nav_tabs = [
                (Tab::Home, "Home"),
                (Tab::Queue, "Queue"),
                (Tab::Settings, "Settings"),
                (Tab::Logs, "Logs"),
                (Tab::About, "About"),
            ];

            for (tab_enum, label) in nav_tabs {
                let is_active = self.current_tab == tab_enum;
                let text_color = if is_active {
                    self.theme.primary()
                } else {
                    MonoText::TERTIARY
                };
                let fill_color = if is_active {
                    MonolithSurfaces::TAB_ACTIVE
                } else {
                    Color32::TRANSPARENT
                };
                let button = egui::Button::new(
                    RichText::new(label)
                        .strong()
                        .color(text_color)
                        .font(FontId::new(MonoType::LABEL, FontFamily::Proportional)),
                )
                .fill(fill_color)
                .min_size(egui::vec2(0.0, MonoSpace::XXXL));
                let response = ui.add(button);
                if response.clicked() {
                    self.current_tab = tab_enum;
                    if tab_enum == Tab::Queue {
                        let _ = self.runtime.send_command(GuiCommand::RefreshQueue);
                    }
                }
                ui.add_space(MonoSpace::SM);
            }
        });
    }

    fn render_sidebar(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.add_space(MonoSpace::XXL);

            // Brand header
            ui.horizontal(|ui| {
                ui.add_space(MonoSpace::LG);
                ui.add(
                    egui::Image::new(egui::include_image!("../../docs/assets/logo.svg"))
                        .max_width(28.0)
                        .max_height(28.0),
                );
                ui.add_space(MonoSpace::SM);
                ui.vertical(|ui| {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("MANGOFETCH")
                            .font(FontId::new(MonoType::HEADING, FontFamily::Proportional))
                            .strong()
                            .color(MonoText::PRIMARY),
                    );
                });
            });

            ui.add_space(MonoSpace::XXL);
            ui.separator();
            ui.add_space(MonoSpace::LG);

            let nav_tabs = [
                (Tab::Home, "Home"),
                (Tab::Queue, "Queue"),
                (Tab::Settings, "Settings"),
                (Tab::Logs, "Logs"),
                (Tab::About, "About"),
            ];

            for (tab_enum, label) in nav_tabs {
                let is_active = self.current_tab == tab_enum;

                ui.horizontal(|ui| {
                    ui.add_space(MonoSpace::SM);

                    // Active indicator bar
                    if is_active {
                        let (rect, _) =
                            ui.allocate_exact_size(egui::vec2(3.0, 28.0), egui::Sense::hover());
                        ui.painter()
                            .rect_filled(rect, CornerRadius::same(2), self.theme.primary());
                        ui.add_space(MonoSpace::XS);
                    } else {
                        ui.add_space(MonoSpace::SM + 3.0 + MonoSpace::XS);
                    }

                    let text_color = if is_active {
                        self.theme.primary()
                    } else {
                        MonoText::TERTIARY
                    };

                    let fill_color = if is_active {
                        MonolithSurfaces::TAB_ACTIVE
                    } else {
                        Color32::TRANSPARENT
                    };

                    let button = egui::Button::new(
                        RichText::new(label)
                            .strong()
                            .color(text_color)
                            .font(FontId::new(MonoType::LABEL, FontFamily::Proportional)),
                    )
                    .fill(fill_color)
                    .min_size(egui::vec2(ui.available_width() - MonoSpace::LG, 28.0));

                    let response = ui.add(button);

                    if response.clicked() {
                        self.current_tab = tab_enum;
                        if tab_enum == Tab::Queue {
                            let _ = self.runtime.send_command(GuiCommand::RefreshQueue);
                        }
                    }
                });

                ui.add_space(MonoSpace::SM);
            }

            // Bottom radar scanner
            let remaining_h = ui.available_height();
            if remaining_h > 40.0 {
                ui.add_space(remaining_h - 36.0);
                ui.horizontal(|ui| {
                    ui.add_space(MonoSpace::LG);
                    let scan_chars = ["|", "/", "-", "\\"];
                    let idx = ((chrono::Local::now().timestamp_subsec_millis() / 250) % 4) as usize;
                    let spin = scan_chars[idx];
                    ui.label(
                        RichText::new(format!("{}  [RADAR: ACTIVE]", spin))
                            .font(FontId::monospace(MonoType::MONO_SMALL))
                            .color(self.theme.primary()),
                    );
                });
            }
        });
    }

    // ── Home Tab ────────────────────────────────────────────────────────────

    fn draw_home_tab(&mut self, ui: &mut Ui) {
        section_header(ui, "Command Center");
        ui.add_space(MonoSpace::SM);

        ui.horizontal(|ui| {
            let total_width = ui.available_width();
            let left_col_w = (total_width * 0.42).max(320.0);
            let right_col_w = total_width - left_col_w - MonoSpace::XL;

            // ── LEFT COLUMN: Controls ──
            ui.allocate_ui(Vec2::new(left_col_w, ui.available_height()), |ui| {
                ui.vertical(|ui| {
                    // URL Input Card
                    surface_card(ui, |ui| {
                        ui.label(
                            RichText::new("URL to Download")
                                .font(FontId::new(MonoType::LABEL, FontFamily::Proportional))
                                .color(MonoText::SECONDARY),
                        );
                        ui.add_space(MonoSpace::SM);

                        sunken_well(ui, |ui| {
                            ui.horizontal(|ui| {
                                let text_edit = ui.add_sized(
                                    Vec2::new(ui.available_width() - 95.0, 28.0),
                                    egui::TextEdit::singleline(&mut self.input_url)
                                        .hint_text("Paste YouTube, Twitch, TikTok or direct link...")
                                        .frame(false),
                                );

                                if text_edit.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                {
                                    self.fetch_preview();
                                }

                                if ui
                                    .add_sized(
                                        Vec2::new(80.0, 28.0),
                                        Button::new(
                                            RichText::new("Inspect")
                                                .font(FontId::new(
                                                    MonoType::LABEL,
                                                    FontFamily::Proportional,
                                                ))
                                                .color(self.theme.primary()),
                                        )
                                        .fill(MonolithSurfaces::SURFACE_5)
                                        .stroke(Stroke::new(1.0_f32, self.theme.primary_border())),
                                    )
                                    .clicked()
                                {
                                    self.fetch_preview();
                                }
                            });
                        });
                    });

                    ui.add_space(MonoSpace::MD);

                    // Options Card
                    surface_card(ui, |ui| {
                        ui.label(
                            RichText::new("Download Options")
                                .font(FontId::new(MonoType::LABEL, FontFamily::Proportional))
                                .strong()
                                .color(MonoText::SECONDARY),
                        );
                        ui.add_space(MonoSpace::MD);

                        ui.checkbox(&mut self.audio_only, "Extract Audio Only (MP3/M4A/FLAC)");
                        ui.add_space(MonoSpace::MD);

                        if !self.audio_only {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Video Quality:")
                                        .color(MonoText::SECONDARY),
                                );
                                egui::ComboBox::from_id_salt("quality_combo")
                                    .selected_text(&self.selected_quality)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut self.selected_quality,
                                            "Best".to_string(),
                                            "Best (Default)",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_quality,
                                            "1080p".to_string(),
                                            "1080p HD",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_quality,
                                            "720p".to_string(),
                                            "720p",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_quality,
                                            "480p".to_string(),
                                            "480p",
                                        );
                                    });
                            });
                            ui.add_space(MonoSpace::SM);
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Video Format:")
                                        .color(MonoText::SECONDARY),
                                );
                                egui::ComboBox::from_id_salt("video_format_combo")
                                    .selected_text(&self.selected_video_format)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut self.selected_video_format,
                                            "mp4".to_string(),
                                            "MP4",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_video_format,
                                            "mkv".to_string(),
                                            "MKV",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_video_format,
                                            "webm".to_string(),
                                            "WEBM",
                                        );
                                    });
                            });
                        } else {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Audio Format:")
                                        .color(MonoText::SECONDARY),
                                );
                                egui::ComboBox::from_id_salt("audio_format_combo")
                                    .selected_text(&self.selected_audio_format)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut self.selected_audio_format,
                                            "mp3".to_string(),
                                            "MP3",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_audio_format,
                                            "m4a".to_string(),
                                            "M4A",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_audio_format,
                                            "flac".to_string(),
                                            "FLAC",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_audio_format,
                                            "wav".to_string(),
                                            "WAV",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_audio_format,
                                            "opus".to_string(),
                                            "OPUS",
                                        );
                                    });
                            });
                            ui.add_space(MonoSpace::SM);
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Audio Quality:")
                                        .color(MonoText::SECONDARY),
                                );
                                egui::ComboBox::from_id_salt("audio_quality_combo")
                                    .selected_text(&self.selected_audio_quality)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut self.selected_audio_quality,
                                            "320K".to_string(),
                                            "320K (High)",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_audio_quality,
                                            "256K".to_string(),
                                            "256K",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_audio_quality,
                                            "192K".to_string(),
                                            "192K (Medium)",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_audio_quality,
                                            "128K".to_string(),
                                            "128K (Low)",
                                        );
                                        ui.selectable_value(
                                            &mut self.selected_audio_quality,
                                            "0".to_string(),
                                            "0 (Best possible)",
                                        );
                                    });
                            });
                        }

                        ui.add_space(MonoSpace::MD);
                        ui.label(
                            RichText::new("Output Directory:")
                                .color(MonoText::SECONDARY),
                        );
                        ui.add_space(MonoSpace::XS);

                        sunken_well(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized(
                                    Vec2::new(ui.available_width() - 85.0, 28.0),
                                    egui::TextEdit::singleline(&mut self.output_dir).frame(false),
                                );

                                if ui
                                    .add_sized(
                                        Vec2::new(75.0, 28.0),
                                        Button::new(
                                            RichText::new("Browse...")
                                                .font(FontId::new(
                                                    MonoType::LABEL,
                                                    FontFamily::Proportional,
                                                )),
                                        ),
                                    )
                                    .clicked()
                                {
                                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                        self.output_dir = path.to_string_lossy().to_string();
                                    }
                                }
                            });
                        });

                        ui.add_space(MonoSpace::XL);

                        // Primary CTA
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let btn_disabled = self.input_url.is_empty();
                            let btn_color = if btn_disabled {
                                MonolithSurfaces::SURFACE_5
                            } else {
                                self.theme.primary()
                            };
                            let text_color = if btn_disabled {
                                MonoText::MUTED
                            } else {
                                Color32::BLACK
                            };

                            let start_btn = ui.add_sized(
                                Vec2::new(180.0, 36.0),
                                Button::new(
                                    RichText::new("Enqueue Download")
                                        .strong()
                                        .font(FontId::new(MonoType::LABEL, FontFamily::Proportional))
                                        .color(text_color),
                                )
                                .fill(btn_color),
                            );

                            if start_btn.clicked() && !self.input_url.is_empty() {
                                let cmd = GuiCommand::StartDownload {
                                    url: self.input_url.clone(),
                                    output_dir: self.output_dir.clone(),
                                    quality: Some(self.selected_quality.clone()),
                                    video_format: Some(self.selected_video_format.clone()),
                                    audio_format: Some(self.selected_audio_format.clone()),
                                    audio_quality: Some(self.selected_audio_quality.clone()),
                                    audio_only: self.audio_only,
                                };
                                let _ = self.runtime.send_command(cmd);

                                self.logs
                                    .push(format!("Enqueued download: {}", self.input_url));
                                self.input_url.clear();
                                self.media_info = None;
                                self.thumbnail_texture = None;
                                self.current_tab = Tab::Queue;
                            }
                        });
                    });
                });
            });

            ui.add_space(MonoSpace::XL);

            // ── RIGHT COLUMN: Preview / Guide ──
            ui.allocate_ui(Vec2::new(right_col_w, ui.available_height()), |ui| {
                ui.vertical(|ui| {
                    if self.media_info_loading {
                        // Loading state — skeleton cards
                        surface_card(ui, |ui| {
                            ui.label(
                                RichText::new("Analyzing stream metadata...")
                                    .font(FontId::new(MonoType::LABEL, FontFamily::Proportional))
                                    .italics()
                                    .color(MonoText::TERTIARY),
                            );
                            ui.add_space(MonoSpace::MD);
                            loading_skeleton(ui, ui.available_width(), 16.0);
                            ui.add_space(MonoSpace::SM);
                            loading_skeleton(ui, ui.available_width() * 0.6, 16.0);
                            ui.add_space(MonoSpace::SM);
                            loading_skeleton(ui, ui.available_width() * 0.4, 16.0);
                        });
                    } else if let Some(ref info) = self.media_info {
                        let has_thumb = info
                            .thumbnail_url
                            .as_ref()
                            .map(|u| !u.is_empty())
                            .unwrap_or(false);

                        if has_thumb {
                            // ── THUMBNAIL CARD: image + layered blur overlays ──
                            let card_w = ui.available_width();
                            let thumb_url = info.thumbnail_url.as_deref().unwrap_or("");

                            // 1. Image widget in normal flow (handles async load)
                            let img_response = ui.add(
                                egui::Image::from_uri(thumb_url)
                                    .max_width(card_w)
                                    .max_height(280.0),
                            );
                            let img_rect = img_response.rect;

                            // 2. Painter draws overlays ON TOP of the image
                            let painter = ui.painter();
                            let rounding = CornerRadius::same(MonoLayout::CORNER_RADIUS_MD);

                            // Card border
                            painter.rect_stroke(
                                img_rect,
                                rounding,
                                Stroke::new(1.0_f32, MonolithSurfaces::SURFACE_6),
                                StrokeKind::Inside,
                            );

                            // ── BLUR LAYER 1: subtle full-image tint ──
                            painter.rect_filled(
                                img_rect,
                                rounding,
                                with_alpha(Color32::BLACK, 0.15),
                            );

                            // ── BLUR LAYER 2: mid-section darkening ──
                            let mid_y = img_rect.min.y + img_rect.height() * 0.50;
                            let mid_rect = egui::Rect::from_min_max(
                                egui::pos2(img_rect.min.x, mid_y),
                                img_rect.max,
                            );
                            painter.rect_filled(
                                mid_rect,
                                CornerRadius::ZERO,
                                with_alpha(Color32::BLACK, 0.30),
                            );

                            // ── BLUR LAYER 3: info area (heaviest) ──
                            let info_y = img_rect.min.y + img_rect.height() * 0.72;
                            let info_area = egui::Rect::from_min_max(
                                egui::pos2(img_rect.min.x, info_y),
                                img_rect.max,
                            );
                            painter.rect_filled(
                                info_area,
                                CornerRadius::ZERO,
                                with_alpha(Color32::BLACK, 0.50),
                            );

                            // ── TEXT on top of everything ──
                            let text_x = img_rect.min.x + MonoSpace::LG;
                            let text_y = info_y + MonoSpace::SM;

                            // Title
                            let title_text = if info.title.len() > 50 {
                                format!("{}...", &info.title[..47])
                            } else {
                                info.title.clone()
                            };
                            painter.text(
                                egui::pos2(text_x, text_y),
                                Align2::LEFT_TOP,
                                &title_text,
                                FontId::new(MonoType::SUBHEADING, FontFamily::Proportional),
                                Color32::WHITE,
                            );

                            // Duration + Platform row
                            let duration_text = if let Some(sec) = info.duration {
                                let min = sec / 60;
                                let s = sec % 60;
                                format!("{:02}:{:02}", min, s)
                            } else {
                                "Live".to_string()
                            };
                            let row_y = text_y + 22.0;
                            painter.text(
                                egui::pos2(text_x, row_y),
                                Align2::LEFT_TOP,
                                &duration_text,
                                FontId::monospace(MonoType::MONO_DATA),
                                Color32::from_gray(200),
                            );

                            // Platform pill text
                            let platform_label =
                                format!("  {}  ", info.platform.to_uppercase());
                            let duration_w = painter
                                .layout_no_wrap(
                                    duration_text.clone(),
                                    FontId::monospace(MonoType::MONO_DATA),
                                    Color32::from_gray(200),
                                )
                                .size()
                                .x;
                            painter.text(
                                egui::pos2(text_x + duration_w + 8.0, row_y),
                                Align2::LEFT_TOP,
                                &platform_label,
                                FontId::monospace(MonoType::MICRO),
                                self.theme.primary(),
                            );
                        } else {
                            // ── NO THUMBNAIL: standard metadata card ──
                            surface_card(ui, |ui| {
                                ui.label(
                                    RichText::new("Media Metadata Inspector")
                                        .font(FontId::new(MonoType::LABEL, FontFamily::Proportional))
                                        .strong()
                                        .color(self.theme.primary()),
                                );
                                ui.add_space(MonoSpace::MD);

                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Title:").color(MonoText::TERTIARY));
                                    ui.label(
                                        RichText::new(&info.title)
                                            .strong()
                                            .color(MonoText::PRIMARY),
                                    );
                                });
                                ui.add_space(MonoSpace::SM);

                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Duration:").color(MonoText::TERTIARY));
                                    if let Some(sec) = info.duration {
                                        let min = sec / 60;
                                        let s = sec % 60;
                                        ui.label(
                                            RichText::new(format!("{:02}:{:02}", min, s))
                                                .font(FontId::monospace(MonoType::MONO_DATA))
                                                .color(MonoText::PRIMARY),
                                        );
                                    } else {
                                        ui.label(
                                            RichText::new("Live Stream / Unknown")
                                                .color(MonoText::TERTIARY),
                                        );
                                    }
                                });
                                ui.add_space(MonoSpace::SM);

                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Platform:").color(MonoText::TERTIARY));
                                    platform_pill(ui, &info.platform);
                                });
                            });
                        }
                    } else if let Some(ref err) = self.media_info_error {
                        error_banner(ui, &format!("Metadata check failed: {}", err));
                    } else {
                        // Quick Start Guide
                        surface_card(ui, |ui| {
                            ui.label(
                                RichText::new("QUICK START")
                                    .font(FontId::new(MonoType::LABEL, FontFamily::Proportional))
                                    .strong()
                                    .color(self.theme.primary()),
                            );
                            ui.add_space(MonoSpace::LG);

                            ui.label(
                                RichText::new(
                                    "MangoFetch is a fast, multi-source download manager built for efficiency.",
                                )
                                .color(MonoText::SECONDARY),
                            );
                            ui.add_space(MonoSpace::MD);

                            ui.label(
                                RichText::new("GETTING STARTED")
                                    .strong()
                                    .color(MonoText::PRIMARY),
                            );
                            ui.add_space(MonoSpace::XS);
                            ui.label(
                                RichText::new("1. Paste a media link inside the URL well.")
                                    .color(MonoText::SECONDARY),
                            );
                            ui.label(
                                RichText::new("2. Click Inspect or press Enter to analyze metadata.")
                                    .color(MonoText::SECONDARY),
                            );
                            ui.label(
                                RichText::new("3. Choose quality and format options.")
                                    .color(MonoText::SECONDARY),
                            );
                            ui.label(
                                RichText::new("4. Click Enqueue Download to start.")
                                    .color(MonoText::SECONDARY),
                            );

                            ui.add_space(MonoSpace::LG);
                            ui.separator();
                            ui.add_space(MonoSpace::MD);

                            ui.label(
                                RichText::new("INTEGRATED PIPELINES")
                                    .strong()
                                    .color(self.theme.secondary()),
                            );
                            ui.add_space(MonoSpace::SM);

                            ui.horizontal_wrapped(|ui| {
                                platform_pill(ui, "YouTube");
                                ui.add_space(MonoSpace::XS);
                                platform_pill(ui, "Instagram");
                                ui.add_space(MonoSpace::XS);
                                platform_pill(ui, "TikTok");
                                ui.add_space(MonoSpace::XS);
                                platform_pill(ui, "Twitch");
                                ui.add_space(MonoSpace::XS);
                                platform_pill(ui, "Torrent");
                                ui.add_space(MonoSpace::XS);
                                platform_pill(ui, "Bluesky");
                                ui.add_space(MonoSpace::XS);
                                platform_pill(ui, "Reddit");
                            });
                        });
                    }
                });
            });
        });
    }

    // ── Queue Tab ───────────────────────────────────────────────────────────

    fn draw_queue_tab(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            section_header(ui, "Active Download Queue");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ghost_button(ui, "Refresh", self.theme.primary()) {
                    let _ = self.runtime.send_command(GuiCommand::RefreshQueue);
                }
            });
        });
        ui.add_space(MonoSpace::SM);

        if self.items.is_empty() {
            sunken_well(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new("No active or completed downloads in the queue.")
                            .font(FontId::new(MonoType::BODY, FontFamily::Proportional))
                            .color(MonoText::TERTIARY),
                    );
                });
            });
            return;
        }

        ScrollArea::vertical().show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(Layout::left_to_right(Align::Center))
                .column(Column::exact(40.0))
                .column(Column::exact(110.0))
                .column(Column::remainder())
                .column(Column::exact(110.0))
                .column(Column::exact(160.0))
                .column(Column::exact(80.0))
                .header(28.0, |mut header| {
                    let col_label = |ui: &mut Ui, text: &str| {
                        ui.label(
                            RichText::new(text)
                                .font(FontId::new(MonoType::MICRO, FontFamily::Monospace))
                                .strong()
                                .color(self.theme.primary()),
                        );
                    };
                    header.col(|ui| col_label(ui, "# ID"));
                    header.col(|ui| col_label(ui, "PLATFORM"));
                    header.col(|ui| col_label(ui, "MEDIA TITLE"));
                    header.col(|ui| col_label(ui, "STATUS"));
                    header.col(|ui| col_label(ui, "PROGRESS"));
                    header.col(|ui| col_label(ui, "ACTIONS"));
                })
                .body(|body| {
                    let items_clone = self.items.clone();
                    body.rows(38.0, items_clone.len(), |mut row| {
                        let item = &items_clone[row.index()];

                        // ID
                        row.col(|ui| {
                            ui.label(
                                RichText::new(format!("{:02}", item.id))
                                    .font(FontId::monospace(MonoType::MONO_DATA))
                                    .color(MonoText::MUTED),
                            );
                        });

                        // Platform
                        row.col(|ui| {
                            platform_pill(ui, &item.platform);
                        });

                        // Title
                        row.col(|ui| {
                            ui.label(RichText::new(&item.title).strong().color(MonoText::PRIMARY));
                        });

                        // Status & Dot
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                status_dot(ui, &item.status);
                                ui.add_space(2.0);
                                ui.label(
                                    RichText::new(&item.status)
                                        .font(FontId::new(
                                            MonoType::CAPTION,
                                            FontFamily::Proportional,
                                        ))
                                        .color(MonoText::SECONDARY),
                                );
                            });
                        });

                        // Progress Bar & Speed
                        row.col(|ui| {
                            ui.vertical(|ui| {
                                ui.add_space(2.0);
                                let p = item.progress / 100.0;
                                ui.add(
                                    ProgressBar::new(p)
                                        .show_percentage()
                                        .fill(self.theme.primary()),
                                );

                                if item.status == "Active" && item.speed > 0.0 {
                                    let speed_str = format!("{:.1} MB/s", item.speed / 1_048_576.0);
                                    ui.label(
                                        RichText::new(speed_str)
                                            .font(FontId::monospace(MonoType::MONO_MICRO))
                                            .color(self.theme.secondary()),
                                    );
                                }
                            });
                        });

                        // Action buttons — using text icons instead of emojis
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                if item.status == "Active" {
                                    if icon_button(
                                        ui,
                                        "\u{23F8}",
                                        "Pause download",
                                        self.theme.primary(),
                                    ) {
                                        let _ =
                                            self.runtime.send_command(GuiCommand::PauseDownload {
                                                id: item.id,
                                            });
                                    }
                                } else if item.status == "Paused"
                                    && icon_button(
                                        ui,
                                        "\u{25B6}",
                                        "Resume download",
                                        MonoSemantics::SUCCESS,
                                    )
                                {
                                    let _ = self
                                        .runtime
                                        .send_command(GuiCommand::ResumeDownload { id: item.id });
                                }

                                if icon_button(
                                    ui,
                                    "\u{2715}",
                                    "Remove from queue",
                                    MonoSemantics::DANGER,
                                ) {
                                    let _ = self
                                        .runtime
                                        .send_command(GuiCommand::RemoveDownload { id: item.id });
                                }
                            });
                        });
                    });
                });
        });
    }

    // ── Settings Tab ────────────────────────────────────────────────────────

    fn draw_settings_tab(&mut self, ui: &mut Ui) {
        section_header(ui, "Preferences");
        ui.add_space(MonoSpace::SM);

        ScrollArea::vertical().show(ui, |ui| {
            // Layout & Behavior
            surface_card(ui, |ui| {
                sub_header(ui, "Application Layout & Behavior");
                ui.add_space(MonoSpace::MD);

                ui.checkbox(
                    &mut self.top_nav_layout,
                    "Hide sidebar, show tabs in top bar (Hotkey: L)",
                );
                ui.add_space(MonoSpace::SM);

                ui.checkbox(
                    &mut self.show_persistent_logs,
                    "Show Persistent Engine Output Terminal",
                );
            });

            ui.add_space(MonoSpace::LG);

            // Concurrency
            surface_card(ui, |ui| {
                sub_header(ui, "Concurrency & Limits");
                ui.add_space(MonoSpace::MD);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Max Concurrent Downloads:").color(MonoText::SECONDARY));
                    ui.add(egui::Slider::new(&mut self.concurrent_limit, 1..=8));
                });

                ui.add_space(MonoSpace::SM);
                ui.checkbox(&mut self.auto_retry, "Automatically retry failed downloads");
            });

            ui.add_space(MonoSpace::LG);

            // Theme Selector
            surface_card(ui, |ui| {
                sub_header(ui, "Graphical Customization");
                ui.add_space(MonoSpace::MD);

                ui.label(RichText::new("Active Brand Preset Theme:").color(MonoText::SECONDARY));
                ui.add_space(MonoSpace::SM);

                let presets = [
                    BrandPreset::PlasmCore,
                    BrandPreset::OxidizedGold,
                    BrandPreset::VioletReaction,
                    BrandPreset::CoolantLiquid,
                    BrandPreset::CriticalMass,
                ];

                ui.horizontal_wrapped(|ui| {
                    for preset in presets {
                        let active = self.theme == preset;

                        let fill = if active {
                            MonolithSurfaces::SURFACE_5
                        } else {
                            MonolithSurfaces::SURFACE_3
                        };
                        let border_stroke = if active {
                            Stroke::new(1.5_f32, preset.primary())
                        } else {
                            Stroke::new(1.0_f32, MonolithSurfaces::SURFACE_6)
                        };

                        let response = Frame::NONE
                            .fill(fill)
                            .stroke(border_stroke)
                            .corner_radius(CornerRadius::same(MonoLayout::CORNER_RADIUS_MD))
                            .inner_margin(Margin::symmetric(10, 6))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Dual-color indicator
                                    let (rect, _) = ui.allocate_exact_size(
                                        Vec2::new(18.0, 10.0),
                                        egui::Sense::hover(),
                                    );

                                    ui.painter().rect_filled(
                                        egui::Rect::from_min_max(
                                            rect.min,
                                            egui::pos2(rect.center().x, rect.max.y),
                                        ),
                                        CornerRadius::same(2),
                                        preset.primary(),
                                    );
                                    ui.painter().rect_filled(
                                        egui::Rect::from_min_max(
                                            egui::pos2(rect.center().x, rect.min.y),
                                            rect.max,
                                        ),
                                        CornerRadius::same(2),
                                        preset.secondary(),
                                    );
                                    ui.add_space(MonoSpace::XS);

                                    let text_color = if active {
                                        MonoText::PRIMARY
                                    } else {
                                        MonoText::TERTIARY
                                    };
                                    ui.label(
                                        RichText::new(preset.name())
                                            .font(FontId::new(
                                                MonoType::CAPTION,
                                                FontFamily::Proportional,
                                            ))
                                            .strong()
                                            .color(text_color),
                                    );
                                });
                            })
                            .response;

                        let response =
                            ui.interact(response.rect, response.id, egui::Sense::click());
                        if response.clicked() {
                            self.theme = preset;
                            crate::theme::apply_monolith_dark(ui.ctx(), preset);
                        }

                        ui.add_space(MonoSpace::SM);
                    }
                });
            });

            ui.add_space(MonoSpace::LG);

            // Dependencies
            surface_card(ui, |ui| {
                sub_header(ui, "External Dependencies");
                ui.add_space(MonoSpace::MD);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("yt-dlp Core Downloader:").color(MonoText::SECONDARY));
                    if self.ytdlp_installed {
                        brand_pill(ui, "INSTALLED", MonoSemantics::SUCCESS);
                    } else {
                        brand_pill(ui, "MISSING", MonoSemantics::DANGER);
                    }
                });

                ui.add_space(MonoSpace::SM);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("ffmpeg Converter Suite:").color(MonoText::SECONDARY));
                    if self.ffmpeg_installed {
                        brand_pill(ui, "INSTALLED", MonoSemantics::SUCCESS);
                    } else {
                        brand_pill(ui, "MISSING", MonoSemantics::DANGER);
                    }
                });

                ui.add_space(MonoSpace::MD);
                if ghost_button(ui, "Force Re-Check Dependencies", self.theme.primary()) {
                    let _ = self.runtime.send_command(GuiCommand::CheckDependencies);
                }
            });
        });
    }

    // ── Logs Tab ────────────────────────────────────────────────────────────

    fn draw_logs_content(&self, ui: &mut Ui) {
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                if self.logs.is_empty() {
                    ui.label(
                        RichText::new("[SYSTEM] Idle - Listening for download tasks...")
                            .font(FontId::monospace(MonoType::MONO_DATA))
                            .color(MonoText::TERTIARY),
                    );
                } else {
                    for line in &self.logs {
                        let text_color = if line.starts_with('\u{2713}') {
                            MonoSemantics::SUCCESS
                        } else if line.starts_with('\u{2717}') {
                            MonoSemantics::DANGER
                        } else if line.starts_with('\u{2699}') {
                            self.theme.primary()
                        } else {
                            MonoText::SECONDARY
                        };

                        ui.label(
                            RichText::new(line)
                                .font(FontId::monospace(MonoType::MONO_DATA))
                                .color(text_color),
                        );
                    }
                }
            });
    }

    fn draw_logs_tab(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            section_header(ui, "Engine Activity Shell");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ghost_button(ui, "Clear Buffer", self.theme.primary()) {
                    self.logs.clear();
                }
            });
        });
        ui.add_space(MonoSpace::SM);

        sunken_well(ui, |ui| {
            self.draw_logs_content(ui);
        });
    }

    // ── About Tab ───────────────────────────────────────────────────────────

    fn draw_about_tab(&mut self, ui: &mut Ui) {
        section_header(ui, "About MangoFetch");
        ui.add_space(MonoSpace::SM);

        ScrollArea::vertical().show(ui, |ui| {
            surface_card(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new("\u{1F96D}")
                            .font(FontId::new(48.0, FontFamily::Proportional)),
                    );
                });
                ui.add_space(MonoSpace::MD);

                ui.label(
                    RichText::new(format!("MangoFetch v{}", env!("CARGO_PKG_VERSION")))
                        .font(FontId::new(MonoType::DISPLAY, FontFamily::Proportional))
                        .strong()
                        .color(self.theme.primary()),
                );

                ui.label(
                    RichText::new("Concurrent media downloading utility.")
                        .color(MonoText::SECONDARY),
                );
                ui.add_space(MonoSpace::MD);

                ui.label(
                    RichText::new("Credits & Contributors")
                        .strong()
                        .color(MonoText::PRIMARY),
                );
                ui.label(
                    RichText::new("Core Architecture & GUI: Jules Martins")
                        .strong()
                        .color(MonoText::PRIMARY),
                );
                ui.label(
                    RichText::new("Framework: egui + eframe (Immediate mode Desktop Suite)")
                        .color(MonoText::SECONDARY),
                );
                ui.label(
                    RichText::new("Async Engine: Tokio multi-threaded runtime")
                        .color(MonoText::SECONDARY),
                );

                ui.add_space(MonoSpace::LG);
                ui.separator();
                ui.add_space(MonoSpace::SM);

                ui.label(
                    RichText::new("LICENSE AND LEGAL")
                        .font(FontId::monospace(MonoType::MONO_SMALL))
                        .strong()
                        .color(self.theme.secondary()),
                );
                ui.add_space(MonoSpace::XS);
                ui.label(
                    RichText::new("This software is licensed under the GPL-3.0-or-later License.")
                        .color(MonoText::SECONDARY),
                );
            });
        });
    }

    fn fetch_preview(&mut self) {
        if !self.input_url.is_empty() {
            self.media_info_loading = true;
            self.media_info = None;
            self.media_info_error = None;
            let _ = self.runtime.send_command(GuiCommand::FetchMediaInfo {
                url: self.input_url.clone(),
            });
        }
    }
}

impl eframe::App for MangoFetchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Drain events
        self.drain_events();

        // Refresh telemetry every 2s
        if self.last_sys_refresh.elapsed() >= std::time::Duration::from_secs(2) {
            self.sys.refresh_cpu();
            self.sys.refresh_memory();
            self.last_sys_refresh = std::time::Instant::now();
        }

        // Hotkey: toggle layout
        if ctx.input(|i| i.key_pressed(egui::Key::L)) {
            self.top_nav_layout = !self.top_nav_layout;
        }

        // Separator painter
        let sep = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("panel_separators"),
        ));
        let sep_stroke = MonoSemantics::separator_stroke();

        // 2. Top Toolbar — compact 48px
        let top = egui::TopBottomPanel::top("command_bar")
            .exact_height(MonoLayout::TOOLBAR_HEIGHT)
            .frame(Frame::NONE.fill(MonolithSurfaces::SURFACE_1))
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.add_space(MonoSpace::LG);
                    ui.label(
                        RichText::new("mangofetch")
                            .font(FontId::monospace(MonoType::MONO_SMALL))
                            .color(MonoText::GHOST),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(MonoSpace::LG);

                        // Pulsing connection indicator
                        let is_even = (chrono::Local::now().timestamp_subsec_millis() / 500)
                            .is_multiple_of(2);
                        let status_lbl = if is_even { "Active" } else { "Online" };
                        status_dot(ui, status_lbl);

                        ui.label(
                            RichText::new("CONNECTED")
                                .font(FontId::monospace(MonoType::MONO_MICRO))
                                .color(MonoSemantics::SUCCESS),
                        );
                        ui.add_space(MonoSpace::SM);
                        ui.label(RichText::new("\u{2502}").color(MonoSemantics::separator()));
                        ui.add_space(MonoSpace::SM);

                        let active_cnt = self.items.iter().filter(|i| i.status == "Active").count();
                        ui.label(
                            RichText::new(format!("THREAD POOL: {} ACTIVE", active_cnt))
                                .font(FontId::monospace(MonoType::MONO_MICRO))
                                .color(self.theme.primary()),
                        );
                    });
                });
            });
        sep.hline(
            top.response.rect.left()..=top.response.rect.right(),
            top.response.rect.bottom(),
            sep_stroke,
        );

        // 3. Bottom Status Bar
        let bottom = egui::TopBottomPanel::bottom("status_bar")
            .exact_height(MonoLayout::STATUS_BAR_HEIGHT)
            .frame(Frame::NONE.fill(MonolithSurfaces::SURFACE_1))
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.add_space(MonoSpace::LG);

                    // CPU telemetry
                    let cpu_usage = self.sys.global_cpu_info().cpu_usage();
                    let cpu_bar_width = 8usize;
                    let cpu_filled = (((cpu_usage / 100.0) * cpu_bar_width as f32).round()
                        as usize)
                        .min(cpu_bar_width);
                    let cpu_bar = format!(
                        "{}{}",
                        "\u{2588}".repeat(cpu_filled),
                        "\u{2591}".repeat(cpu_bar_width.saturating_sub(cpu_filled))
                    );

                    // RAM telemetry
                    let total_mem = self.sys.total_memory() / 1_048_576;
                    let used_mem = self.sys.used_memory() / 1_048_576;
                    let ram_pct = (used_mem as f32 / total_mem as f32 * 100.0).clamp(0.0, 100.0);
                    let ram_bar_width = 8usize;
                    let ram_filled = (((ram_pct / 100.0) * ram_bar_width as f32).round() as usize)
                        .min(ram_bar_width);
                    let ram_bar = format!(
                        "{}{}",
                        "\u{2588}".repeat(ram_filled),
                        "\u{2591}".repeat(ram_bar_width.saturating_sub(ram_filled))
                    );

                    ui.label(
                        RichText::new(format!(
                            "CPU [{}] {:.1}%    RAM [{}] {}/{} MB",
                            cpu_bar, cpu_usage, ram_bar, used_mem, total_mem
                        ))
                        .font(FontId::monospace(MonoType::MONO_SMALL))
                        .color(MonoText::CHROME),
                    );

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(MonoSpace::LG);
                        ui.label(
                            RichText::new(format!(
                                "{}  \u{00B7}  v{}",
                                self.theme.name().to_uppercase(),
                                env!("CARGO_PKG_VERSION")
                            ))
                            .font(FontId::monospace(MonoType::MONO_SMALL))
                            .color(MonoText::GHOST),
                        );
                    });
                });
            });
        sep.hline(
            bottom.response.rect.left()..=bottom.response.rect.right(),
            bottom.response.rect.top(),
            sep_stroke,
        );

        // Persistent log panel
        if self.show_persistent_logs {
            let log_panel = egui::TopBottomPanel::bottom("persistent_logs_panel")
                .resizable(true)
                .min_height(120.0)
                .frame(Frame::NONE.fill(MonolithSurfaces::SURFACE_2))
                .show(ctx, |ui| {
                    Frame::NONE
                        .inner_margin(Margin::same(MonoSpace::SM as i8))
                        .show(ui, |ui| {
                            self.draw_logs_content(ui);
                        });
                });
            sep.hline(
                log_panel.response.rect.left()..=log_panel.response.rect.right(),
                log_panel.response.rect.top(),
                sep_stroke,
            );
        }

        // Navigation panel
        if self.top_nav_layout {
            // Top-nav mode: logo + tabs in top bar
            let top_nav = egui::TopBottomPanel::top("top_nav_panel")
                .exact_height(48.0)
                .frame(Frame::NONE.fill(MonolithSurfaces::SURFACE_1))
                .show(ctx, |ui| {
                    self.render_top_nav(ui);
                });
            sep.hline(
                top_nav.response.rect.left()..=top_nav.response.rect.right(),
                top_nav.response.rect.bottom(),
                sep_stroke,
            );
        } else {
            // Sidebar mode: full sidebar with logo + tabs + radar
            let sidebar = egui::SidePanel::left("left_sidebar")
                .resizable(false)
                .exact_width(MonoLayout::SIDEBAR_WIDTH)
                .frame(Frame::NONE.fill(MonolithSurfaces::SURFACE_1))
                .show(ctx, |ui| {
                    self.render_sidebar(ui);
                });
            sep.vline(
                sidebar.response.rect.right(),
                sidebar.response.rect.top()..=sidebar.response.rect.bottom(),
                sep_stroke,
            );
        }

        // Central content
        egui::CentralPanel::default()
            .frame(Frame::NONE.fill(MonolithSurfaces::SURFACE_3))
            .show(ctx, |ui| {
                Frame::NONE
                    .inner_margin(Margin::same(MonoSpace::XL as i8))
                    .show(ui, |ui| match self.current_tab {
                        Tab::Home => self.draw_home_tab(ui),
                        Tab::Queue => self.draw_queue_tab(ui),
                        Tab::Settings => self.draw_settings_tab(ui),
                        Tab::Logs => self.draw_logs_tab(ui),
                        Tab::About => self.draw_about_tab(ui),
                    });
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }
}
