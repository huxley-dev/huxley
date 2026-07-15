use uuid::Uuid;

const PAGE_LIMIT_DEFAULT: i32 = 50;
const PAGE_LIMIT_MIN: i32 = 5;
const PAGE_LIMIT_MAX: i32 = 100;

pub struct Page<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<Uuid>,
}

#[derive(Clone)]
pub enum PageSort {
    Asc,
    Desc,
}

impl Default for PageSort {
    fn default() -> Self {
        Self::Asc
    }
}
pub struct PageQuery {
    pub limit: Option<i32>,
    pub next_cursor: Option<Uuid>,
    pub sort: Option<PageSort>,
}

impl Default for PageQuery {
    fn default() -> Self {
        Self {
            limit: Some(PAGE_LIMIT_DEFAULT),
            next_cursor: None,
            sort: Some(PageSort::Asc),
        }
    }
}

impl PageQuery {
    pub fn resolved_limit(&self) -> i32 {
        self.limit
            .unwrap_or(PAGE_LIMIT_DEFAULT)
            .clamp(PAGE_LIMIT_MIN, PAGE_LIMIT_MAX)
    }

    pub fn resolved_sort(&self) -> PageSort {
        self.sort.clone().unwrap_or(PageSort::default())
    }
}

pub enum Field<T> {
    Keep,
    Set(T),
    SetNull,
}

impl<T> Default for Field<T> {
    fn default() -> Self {
        Field::Keep
    }
}

impl<T> Field<T> {
    pub fn into_parts(self) -> (bool, Option<T>) {
        match self {
            Field::Keep => (false, None),
            Field::Set(v) => (true, Some(v)),
            Field::SetNull => (true, None),
        }
    }
}

pub fn to_field<T>(v: Option<Option<T>>) -> Field<T> {
    match v {
        None => Field::Keep,
        Some(None) => Field::SetNull,
        Some(Some(val)) => Field::Set(val),
    }
}
