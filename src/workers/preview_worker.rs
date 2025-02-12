use std::{
    io,
    path::PathBuf,
    process::{Command, Stdio},
    sync::mpsc::{Receiver, Sender},
};

use ratatui::layout::Rect;

use crate::{
    preview::preview_file::FilePreview,
    types::event::{AppEvent, PreviewData},
};

#[derive(Clone, Debug)]
pub struct PreviewWorkerRequest {
    pub path: PathBuf,
    pub area: Rect,
}

#[derive(Debug)]
pub struct PreviewWorker {
    pub preview_script: PathBuf,
    pub response_tx: Sender<AppEvent>,
    pub request_rx: Receiver<PreviewWorkerRequest>,
}

impl PreviewWorker {
    pub fn listen_for_events(&self) {
        loop {
            if let Ok(req) = self.request_rx.recv() {
                self.spawn_command(req);
            }
        }
    }

    fn spawn_command(&self, req: PreviewWorkerRequest) {
        let output = Command::new(&self.preview_script)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .arg("--path")
            .arg(&req.path)
            .arg("--preview-width")
            .arg(req.area.width.to_string())
            .arg("--preview-height")
            .arg(req.area.height.to_string())
            .output();

        let resp = match output {
            Ok(output) => {
                if output.status.success() {
                    let preview = FilePreview::from(output);
                    AppEvent::PreviewFile {
                        path: req.path,
                        res: Ok(PreviewData::Script(Box::new(preview))),
                    }
                } else {
                    AppEvent::PreviewFile {
                        path: req.path,
                        res: Err(io::Error::new(io::ErrorKind::Other, "nonzero status")),
                    }
                }
            }
            Err(err) => AppEvent::PreviewFile {
                path: req.path,
                res: Err(io::Error::new(io::ErrorKind::Other, format!("{err}"))),
            },
        };
        let _ = self.response_tx.send(resp);
    }
}
