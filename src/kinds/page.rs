#[derive(PartialEq)]
pub enum Page {
    Due,
    Today,
    Future,
    History,
}

impl Page {
    pub const ALL: &'static [Page] = &[Page::Due, Page::Today, Page::Future, Page::History];

    pub fn label(&self) -> &str {
        match self {
            Page::Due => "Due [1]",
            Page::Today => "Today [2]",
            Page::Future => "Future [3]",
            Page::History => "History [4]",
        }
    }

    pub fn key(&self) -> char {
        match self {
            Page::Due => '1',
            Page::Today => '2',
            Page::Future => '3',
            Page::History => '4',
        }
    }

    pub fn index(&self) -> usize {
        Self::ALL.iter().position(|p| p == self).unwrap()
    }

    pub fn next(&self) -> Page {
        match self {
            Page::Due => Page::Today,
            Page::Today => Page::Future,
            Page::Future => Page::History,
            Page::History => Page::Due,
        }
    }

    pub fn prev(&self) -> Page {
        match self {
            Page::Due => Page::History,
            Page::Today => Page::Due,
            Page::Future => Page::Today,
            Page::History => Page::Future,
        }
    }
}
