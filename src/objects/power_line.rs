pub struct PowerLine {
    pub status: bool,
}

impl PowerLine {
    pub fn swith_on(&mut self) -> bool {
        self.status = true;
        self.status
    }

    #[allow(dead_code)]
    pub fn swith_off(&mut self) -> bool {
        self.status = false;
        self.status
    }
}