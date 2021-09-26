use termion::event::Key;

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::views::TuiWorkerView;
use crate::ui::TuiBackend;

pub fn show_workers(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    exit_key: &Key,
) -> JoshutoResult<()> {
    context.flush_event();

    let view = TuiWorkerView::new(*exit_key);
    view.display(context, backend);
    Ok(())
}
