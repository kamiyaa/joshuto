use std::{
    error::Error,
    io,
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
};

use allmytoes::{ThumbSize, AMT};
use ratatui_image::{picker::Picker, Resize};

use crate::types::event::{AppEvent, PreviewData};

use super::preview_worker::PreviewWorkerRequest;

#[derive(Debug)]
pub struct PreviewImageWorker {
    pub response_tx: Sender<AppEvent>,
    pub request_rx: Receiver<PreviewWorkerRequest>,
    pub picker: Option<Picker>,
    pub allmytoes: Option<AMT>,
    pub xdg_thumb_size: ThumbSize,
}

impl PreviewImageWorker {
    pub fn listen_for_events(&self) {
        loop {
            if let Ok(req) = self.request_rx.recv() {
                self.spawn_command(req);
            }
        }
    }

    fn spawn_command(&self, req: PreviewWorkerRequest) {
        let thumb_path = if let Some(amt) = &self.allmytoes {
            let thumb_result = amt.get(&req.path, self.xdg_thumb_size);
            if let Ok(thumb) = thumb_result {
                PathBuf::from(thumb.path)
            } else {
                req.path.clone()
            }
        } else {
            req.path.clone()
        };
        let proto = image::ImageReader::open(thumb_path.as_path())
            .and_then(|reader| reader.decode().map_err(Self::map_io_err))
            .and_then(|dyn_img| match self.picker.as_ref() {
                Some(picker) => picker
                    .new_protocol(dyn_img, req.area, Resize::Fit(None))
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{err}"))),
                None => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Preview protocol disabled".to_string(),
                )),
            });
        if let Ok(proto) = proto {
            let resp = AppEvent::PreviewFile {
                path: req.path,
                res: Ok(PreviewData::Image(Box::new(proto))),
            };
            let _ = self.response_tx.send(resp);
        }
    }

    #[inline]
    fn map_io_err(err: impl Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, format!("{err}"))
    }
}
