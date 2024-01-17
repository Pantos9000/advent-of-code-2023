use crate::map::Shape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn follow(&self, shape: Shape) -> (Self, Option<Self>) {
        match shape {
            Shape::Empty => self.straight(),
            Shape::MirrorForward => self.follow_mirror_forward(),
            Shape::MirrorBackward => self.follow_mirror_backward(),
            Shape::SplitterHorizontal => self.follow_splitter_horizontal(),
            Shape::SplitterVertical => self.follow_splitter_vertical(),
        }
    }

    fn follow_splitter_horizontal(&self) -> (Self, Option<Self>) {
        match self {
            Self::Up | Self::Down => (Self::Left, Some(Self::Right)),
            Self::Left | Self::Right => self.straight(),
        }
    }

    fn follow_splitter_vertical(&self) -> (Self, Option<Self>) {
        match self {
            Self::Up | Self::Down => self.straight(),
            Self::Left | Self::Right => (Self::Up, Some(Self::Down)),
        }
    }

    fn straight(&self) -> (Self, Option<Self>) {
        (*self, None)
    }

    fn follow_mirror_forward(&self) -> (Self, Option<Self>) {
        let next = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Up,
            Self::Down => Self::Left,
            Self::Left => Self::Down,
        };
        (next, None)
    }

    fn follow_mirror_backward(&self) -> (Self, Option<Self>) {
        let next = match self {
            Self::Up => Self::Left,
            Self::Left => Self::Up,
            Self::Down => Self::Right,
            Self::Right => Self::Down,
        };
        (next, None)
    }
}
