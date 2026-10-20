use std::ffi::c_void;
use std::fmt;
use std::sync::atomic::{AtomicU8, Ordering};

pub const OUT_W: u32 = 720;
pub const OUT_H: u32 = 480;

/// How the 720x480 composite is placed on the physical panel.
/// `Original` keeps the upstream integer/bezel placement (desktop and device
/// builds). The Android build selects one of the others every frame:
/// `Fill` stretches to the whole panel, `Fit` keeps 3:2 with black bars, and
/// `Integer` uses the largest whole multiple of the native 240x160 picture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelLayout {
    Original,
    Fill,
    Fit,
    Integer,
}

static PANEL_LAYOUT: AtomicU8 = AtomicU8::new(0);

pub fn set_panel_layout(layout: PanelLayout) {
    PANEL_LAYOUT.store(layout as u8, Ordering::Relaxed);
}

pub fn panel_layout() -> PanelLayout {
    match PANEL_LAYOUT.load(Ordering::Relaxed) {
        1 => PanelLayout::Fill,
        2 => PanelLayout::Fit,
        3 => PanelLayout::Integer,
        _ => PanelLayout::Original,
    }
}

#[derive(Debug)]
pub enum GfxError {
    Context(String),
    Shader(String),
    Framebuffer(u32),
}

impl fmt::Display for GfxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GfxError::Context(m) => write!(f, "gl context: {m}"),
            GfxError::Shader(m) => write!(f, "shader: {m}"),
            GfxError::Framebuffer(s) => write!(f, "framebuffer incomplete: 0x{s:x}"),
        }
    }
}

impl std::error::Error for GfxError {}

pub trait Surface {
    fn make_current(&mut self) -> Result<(), GfxError>;
    fn window_size(&self) -> (u32, u32);
    fn swap(&mut self) -> Result<(), GfxError>;
    fn proc_address(&self, name: &str) -> *const c_void;
}

pub fn fit_scale(win_w: u32, win_h: u32) -> u32 {
    (win_w / OUT_W).min(win_h / OUT_H).max(1)
}

pub fn fit_rect(win_w: u32, win_h: u32) -> (i32, i32, i32, i32) {
    let s = fit_scale(win_w, win_h) as i32;
    let (w, h) = (OUT_W as i32 * s, OUT_H as i32 * s);
    ((win_w as i32 - w) / 2, (win_h as i32 - h) / 2, w, h)
}

pub fn blit_rect_fit(panel: (u32, u32), shake: f32) -> (i32, i32, i32, i32) {
    let scale = (panel.0 as f32 / OUT_W as f32).min(panel.1 as f32 / OUT_H as f32);
    let w = (OUT_W as f32 * scale).round() as i32;
    let h = (OUT_H as f32 * scale).round() as i32;
    let dx = (shake * scale).round() as i32;
    (
        (panel.0 as i32 - w) / 2 + dx,
        (panel.1 as i32 - h) / 2,
        w,
        h,
    )
}

pub fn blit_rect(window: (u32, u32), shake: f32) -> (i32, i32, i32, i32) {
    if window.0 > 0 && window.1 > 0 {
        match panel_layout() {
            PanelLayout::Original => {}
            PanelLayout::Fill => {
                let dx = shake * window.0 as f32 / OUT_W as f32;
                return (dx.round() as i32, 0, window.0 as i32, window.1 as i32);
            }
            PanelLayout::Fit => return blit_rect_fit(window, shake),
            PanelLayout::Integer => {
                let k = (window.0 / 240).min(window.1 / 160).max(1) as i32;
                let (w, h) = (240 * k, 160 * k);
                let dx = (shake * k as f32 / 3.0).round() as i32;
                return (
                    (window.0 as i32 - w) / 2 + dx,
                    (window.1 as i32 - h) / 2,
                    w,
                    h,
                );
            }
        }
    }
    if crate::bezel::framed(window) {
        let (x, y, w, h) = crate::bezel::screen_rect(window);
        let dx = shake * w as f32 / OUT_W as f32;
        return (x + dx.round() as i32, window.1 as i32 - y - h, w, h);
    }
    // Fractional 3:2 scaling fills KONKR's 960x640 panel instead of
    // leaving a 720x480 1x viewport. Preserve behavior on other ratios.
    if window.0.saturating_mul(OUT_H) == window.1.saturating_mul(OUT_W)
        || window.0 < OUT_W
        || window.1 < OUT_H
    {
        return blit_rect_fit(window, shake);
    }
    let (x, y, w, h) = fit_rect(window.0, window.1);
    let dx = shake * fit_scale(window.0, window.1) as f32;
    (x + dx.round() as i32, y, w, h)
}
