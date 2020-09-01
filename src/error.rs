use crate::position;

pub struct Error {
    pub name: String,
    pub message: String,
    pub pos_start: position::Position,
    pub pos_end: position::Position
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.pos_start.ln == self.pos_end.ln {
            write!(f, "\n{}: {}\nFrom line no: {}, from coloumn no: {} to coulumn no: {}", self.name, self.message, self.pos_start.ln, self.pos_start.cn, self.pos_end.cn)
        } else {
            write!(f, "\n{}: {}\nFrom line no: {}, at coloumn no: {} to line no: {}, at coloumn no: {}", self.name, self.message, self.pos_start.ln, self.pos_start.cn, self.pos_end.ln, self.pos_end.cn)
        }

    }
}
