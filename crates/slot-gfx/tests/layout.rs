use slot_gfx::{blit_rect, canvas_size, set_panel_layout, PanelLayout};

// One test function: the layout is process-global, so keep it serial.
#[test]
fn android_layouts_place_the_composite_on_a_4x3_panel() {
    let panel = (1280, 960);

    set_panel_layout(PanelLayout::Fill);
    assert_eq!(blit_rect(panel, 0.0), (0, 0, 1280, 960));
    assert_eq!(canvas_size(panel), (1280, 960));

    set_panel_layout(PanelLayout::Fit);
    let (x, y, w, h) = blit_rect(panel, 0.0);
    assert_eq!((x, w), (0, 1280));
    assert_eq!(h, 853);
    assert_eq!(y, (960 - 853) / 2);

    set_panel_layout(PanelLayout::Integer);
    assert_eq!(blit_rect(panel, 0.0), (40, 80, 1200, 800));

    set_panel_layout(PanelLayout::Original);
    assert_eq!(blit_rect((960, 640), 0.0), (0, 0, 960, 640));
}
