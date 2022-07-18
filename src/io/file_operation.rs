#[derive(Clone, Copy, Debug)]
pub enum FileOperation {
    Cut,
    Copy,
    Delete,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FileOperationOptions {
    // cut, copy
    pub overwrite: bool,
    pub skip_exist: bool,

    // delete
    pub permanently: bool,
}

impl std::fmt::Display for FileOperationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "overwrite={} skip_exist={}",
            self.overwrite, self.skip_exist
        )
    }
}

#[derive(Clone, Debug)]
pub struct FileOperationProgress {
    _kind: FileOperation,
    _files_processed: usize,
    _total_files: usize,
    _bytes_processed: u64,
    _total_bytes: u64,
}

impl FileOperationProgress {
    pub fn new(
        _kind: FileOperation,
        _files_processed: usize,
        _total_files: usize,
        _bytes_processed: u64,
        _total_bytes: u64,
    ) -> Self {
        Self {
            _kind,
            _files_processed,
            _total_files,
            _bytes_processed,
            _total_bytes,
        }
    }

    pub fn kind(&self) -> FileOperation {
        self._kind
    }

    pub fn files_processed(&self) -> usize {
        self._files_processed
    }

    pub fn set_files_processed(&mut self, files_processed: usize) {
        self._files_processed = files_processed;
    }

    pub fn total_files(&self) -> usize {
        self._total_files
    }

    pub fn bytes_processed(&self) -> u64 {
        self._bytes_processed
    }

    pub fn set_bytes_processed(&mut self, _bytes_processed: u64) {
        self._bytes_processed = _bytes_processed;
    }

    pub fn total_bytes(&self) -> u64 {
        self._total_bytes
    }
}
