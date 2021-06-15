use crate::context::AppContext;
use crate::ui::TuiBackend;

pub fn preview_with_script(context: &AppContext, backend: &mut TuiBackend) {
    let preview_options = context.config_ref().preview_options_ref();
    if let Some(script_path) = preview_options.preview_script.as_ref() {
        let file_full_path = 0;
        let preview_width = 0;
        let preview_height = 0;
        let image_cache = 0;
        let preview_image = if preview_options.preview_images { 1 } else { 0 };

        // spawn preview process
    }
}
