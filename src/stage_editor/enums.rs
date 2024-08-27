

#[derive(Default, Copy, Clone, Debug)]
pub enum EditorItem {
    #[default]
    Ground = 0,
    Spike = 1,
    Spawn = 2
}

impl EditorItem {
    pub fn cycle_next(&self) -> Self {
        match self {
            EditorItem::Ground => EditorItem::Spike,
            EditorItem::Spike => EditorItem::Spawn,
            EditorItem::Spawn => EditorItem::Ground,
        }
    }
    pub fn cycle_prev(&self) -> Self {
        match self {
            EditorItem::Ground => EditorItem::Spawn,
            EditorItem::Spawn => EditorItem::Spike,
            EditorItem::Spike => EditorItem::Ground,
        }
    }
}

