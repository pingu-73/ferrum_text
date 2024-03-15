pub struct Size{
    pub width: u16,
    pub height: u16,
}

pub struct Terminal{
    size: Size,     // using size fn to return size to avoid changing of size form outside.
}

impl Terminal{
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size { width: size.0, height: size.1 }
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}