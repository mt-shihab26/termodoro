#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Due,
    Today,
    Index,
    History,
}

impl Page {
    pub const ALL: &'static [Page] = &[Page::Due, Page::Today, Page::Index, Page::History];

    pub fn label(&self) -> &str {
        match self {
            Page::Due => "Due [1]",
            Page::Today => "Today [2]",
            Page::Index => "Index [3]",
            Page::History => "History [4]",
        }
    }

    pub fn index(&self) -> usize {
        Self::ALL.iter().position(|p| p == self).unwrap()
    }

    pub fn next(&self) -> Page {
        match self {
            Page::Due => Page::Today,
            Page::Today => Page::Index,
            Page::Index => Page::History,
            Page::History => Page::Due,
        }
    }

    pub fn prev(&self) -> Page {
        match self {
            Page::Due => Page::History,
            Page::Today => Page::Due,
            Page::Index => Page::Today,
            Page::History => Page::Index,
        }
    }
}
