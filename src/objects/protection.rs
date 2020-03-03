pub struct Protection {
    pub status: bool,
}

impl Protection {
    pub fn put_on(&mut self) -> bool {
        self.status = true;
        self.status
    }
    pub fn put_off(&mut self) -> bool {
        self.status = false;
        self.status
    }
}