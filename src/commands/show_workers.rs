use crate::context::JoshutoContext;
use crate::error::JoshutoResult;

use crate::ui::views::TuiWorkerView;
use crate::ui::TuiBackend;

pub fn show_workers(context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let view = TuiWorkerView::new();
    view.display(context, backend);
    Ok(())
}
