use crate::context::AppContext;
use crate::error::JoshutoResult;

use crate::ui::views::TuiWorkerView;
use crate::ui::TuiBackend;

pub fn show_workers(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    context.flush_event();

    let view = TuiWorkerView::new();
    view.display(context, backend);
    Ok(())
}
