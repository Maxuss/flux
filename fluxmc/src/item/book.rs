use lobsterchat::component::Component;
use serde::Serialize;

use crate::text::StrComponent;

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct BookAndQuillMeta {
    pages: Vec<String>,
}

impl BookAndQuillMeta {
    pub fn new(page_count: Option<usize>) -> Self {
        Self {
            pages: Vec::with_capacity(if let Some(page) = page_count { page } else { 1 }),
        }
    }

    pub fn add_page<S: Into<String>>(&mut self, page: S) {
        self.pages.push(page.into())
    }

    pub fn new_full(pages: Vec<String>) -> Self {
        Self {
            pages: pages.iter().map(|page| page.replace('\n', "\\n")).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum BookGeneration {
    Original = 0,
    CopyOfOriginal = 1,
    CopyOfCopy = 2,
    Tattered = 3,
}

impl Serialize for BookGeneration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
pub struct WrittenBookMeta {
    resolved: Option<bool>,
    generation: Option<BookGeneration>,
    author: String,
    title: String,
    pages: Vec<StrComponent>,
}

impl WrittenBookMeta {
    pub fn new<A: Into<String>, T: Into<String>>(
        author: A,
        title: T,
        page_count: Option<usize>,
    ) -> Self {
        Self {
            resolved: None,
            generation: None,
            author: author.into(),
            title: title.into(),
            pages: Vec::with_capacity(if let Some(pages) = page_count {
                pages
            } else {
                1
            }),
        }
    }

    pub fn new_full<A: Into<String>, T: Into<String>>(
        author: A,
        title: T,
        pages: Vec<Component>,
    ) -> Self {
        Self {
            resolved: None,
            generation: None,
            author: author.into(),
            title: title.into(),
            pages: pages.into_iter().map(|page| page.into()).collect(),
        }
    }

    pub fn resolved(&self) -> Option<bool> {
        self.resolved
    }

    pub fn generation(&self) -> Option<BookGeneration> {
        self.generation
    }

    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    #[must_use = "This is an expensive operation that requires iterating over all pages in the book"]
    pub fn pages(&self) -> Vec<Component> {
        self.pages
            .iter()
            .map(|page| page.value().to_owned())
            .collect()
    }

    pub fn set_resolved(&mut self, resolved: bool) {
        self.resolved = Some(resolved)
    }

    pub fn set_generation(&mut self, generation: BookGeneration) {
        self.generation = Some(generation)
    }

    pub fn set_author<S: Into<String>>(&mut self, author: S) {
        self.author = author.into()
    }

    pub fn set_title<S: Into<String>>(&mut self, title: S) {
        self.title = title.into()
    }

    pub fn add_page(&mut self, page: Component) {
        self.pages.push(page.into())
    }
}
