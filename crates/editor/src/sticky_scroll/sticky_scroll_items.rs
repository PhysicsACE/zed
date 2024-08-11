pub struct PinnedItem {
    start: usize,
    end: usize,
    depth: usize,
    text: String,
}

impl PinnedItem {
    pub fn contains(&self, candidate: usize) -> bool {
        return (self.start <= candidate && candidate <= self.end)
    }
}

pub struct StickyScrollManager {
    // A stack of the current scopes that contains the contents of 
    // the visible buffer. Will be sorted by the starting offset
    // of its PinnedItems
    stack: Vec<PinnedItem>,
    max_size: Option<usize>,
}

impl StickyScrollManager {
    pub fn needs_recompute(
        &self,
        visible_buffer_range: Range<MultiBufferRow>,
        snapshot: &DisplaySnapshot,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        let visible_start_anchor = snapshot
            .buffer_snapshot
            .anchor_before(Point::new(visible_buffer_range.start.0, 0));
        if let Some((_, _, snapshot)) = snapshot.buffer_snapshot.as_singleton() {
            
        }
    }
}