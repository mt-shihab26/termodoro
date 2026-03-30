#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Due,
    Today,
    Future,
}

impl Page {
    pub const ALL: &'static [Page] = &[Page::Due, Page::Today, Page::Future];

    pub fn label(&self) -> &str {
        match self {
            Page::Due => "Due [1]",
            Page::Today => "Today [2]",
            Page::Future => "Future [3]",
        }
    }

    pub fn key(&self) -> char {
        match self {
            Page::Due => '1',
            Page::Today => '2',
            Page::Future => '3',
        }
    }

    pub fn index(&self) -> usize {
        Self::ALL.iter().position(|p| p == self).unwrap()
    }

    pub fn next(&self) -> Page {
        let i = (self.index() + 1) % Self::ALL.len();
        Self::ALL[i]
    }

    pub fn prev(&self) -> Page {
        let i = (self.index() + Self::ALL.len() - 1) % Self::ALL.len();
        Self::ALL[i]
    }
}
