use crate::module::Keyword;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

impl Visibility {
    pub fn fromKeyword(keyword: Keyword) -> Option<Self> {
        return match keyword {
            Keyword::Public => Some(Visibility::Public),
            Keyword::Private => Some(Visibility::Private),
            _ => None,
        };
    }
}
