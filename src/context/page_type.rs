pub enum PageType {
    Normal,
    // This u8 indicates offset (How much the help page is scrolled)
    Help(u8),
}

impl PageType {
    pub fn set_offset(&mut self, new_offset: u8) -> Result<(), ()> {
        match self {
            PageType::Normal => Err(()),
            PageType::Help(ref mut offset) => {
                *offset = new_offset;
                Ok(())
            }
        }
    }

    pub fn get_offset(&self) -> Result<u8, ()> {
        match self {
            PageType::Normal => Err(()),
            PageType::Help(offset) => Ok(*offset),
        }
    }
    pub fn offset_up(&mut self) {
        if let PageType::Help(ref mut offset) = self {
            if *offset > 0 {
                *offset -= 1;
            }
        }
    }
    pub fn offset_down(&mut self) {
        if let PageType::Help(ref mut offset) = self {
            *offset += 1;
        }
    }
}
