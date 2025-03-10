use std::sync::Arc;

use tiny_skia::Pixmap;

use crate::components::background::Background;
use crate::components::breadcrumbs::Breadcrumbs;
use crate::components::container::Container;
use crate::components::editor::code::Code;
use crate::components::editor::mac_title_bar::MacTitleBar;
use crate::components::interface::component::ComponentContext;
use crate::components::interface::render_error;
use crate::components::rect::Rect;
use crate::components::watermark::Watermark;
use crate::config::TakeSnapshotParams;

// Scale the screenshot to 3 times its size
const SCALE_FACTOR: f32 = 3.;

// The params is come from neovim instance
pub fn take_snapshot(params: TakeSnapshotParams) -> render_error::Result<Pixmap> {
    let context = ComponentContext {
        scale_factor: SCALE_FACTOR,
        take_snapshot_params: Arc::new(params.clone()),
    };
    let pixmap = Container::from_children(vec![Box::new(Background::from_children(vec![
        Box::new(Rect::new(
            16.,
            vec![
                Box::new(MacTitleBar::from_radius(8.)),
                Box::new(Breadcrumbs::from_path(
                    params.file_path,
                    15.,
                    params.breadcrumbs_separator,
                    params.has_breadcrumbs,
                )),
                Box::new(Code::new(params.code, 20., 15.)),
            ],
        )),
        Box::new(Watermark::new(params.watermark)),
    ]))])
    .draw_root(&context)?;

    Ok(pixmap)
}
