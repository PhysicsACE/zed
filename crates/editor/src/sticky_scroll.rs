use std::ops::Range;
use multi_buffer::MultiBufferRow
use crate::{DisplaySnapshot, Editor, Anchor};
use language::OutlineItem;
use text::Point;

#[derive(Default)]
pub struct StickyScrollManager {
    stack: Vec<OutlineItem<Anchor>>,
    max_size: Option<usize>,
}

pub struct StickyLine {
    pub line: MultiBufferRow,
}

impl StickyScrollManager {
    // pub fn needs_recompute(
    //     &self,
    //     position: Anchor,
    //     snapshot: &DisplaySnapshot,
    // ) -> bool {
    //     self.stack.is_empty() || !is_contained_in_scope(
    //         self.stack.last().unwrap().clone(), position, self)
    // }

    // pub fn size(&self) -> usize {
    //     self.stack.len()
    // }

    // pub fn set_size(&mut self, size: usize) {
    //     self.max_size = Some(size)
    // }

    // pub(crate) fn set_sticky_header_height(&mut self, height: usize, cx: &mut ViewContext<Editor>) {
    //     let opened_first_time = self.sticky_header_height.is_none();
    //     self.sticky_header_height = Some(height);
    //     if opened_first_time {
    //         cx.spawn(|editor, mut cx| async move {
    //             editor.update(&mut cx, |editor, cx| {
    //                 editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
    //             })
    //         })
    //         .detach();
    //     }
    // }

    // pub fn update(
    //     &mut self,
    //     position: Anchor,
    //     snapshot: &DisplaySnapshot,
    //     cx: &mut ViewContext<Editor>,
    // ) {
    //     if self.needs_recompute() {
    //         let mut start_boundary = snapshot
    //             .buffer_snapshot
    //             .anchor_before(Point::new(0, 0));
    //         let end_boundary = position.clone();
    //         let search_range = start_boundary..end_boundary;
    //         let search_results = snapshot
    //             .buffer_snapshot
    //             .symbols_containing_in_range(search_range.clone(), position.clone(), cx)
    //             .into_iter()
    //             .collect();
    //         for outline in search_results.iter() {
    //             self.stack.push(outline.clone());
    //         }
    //     }

    //     let position_offset = position.to_offset(&snapshot.buffer_snapshot);
    //     let mut contained = true;
    //     let mut prev_outline = None;
    //     while contained {
    //         match self.stack.last() {
    //             Some(outline_item) => {
    //                 if is_contained_in_scope(outline_item.clone(), position_offset, snapshot) {
    //                     if let Some(prev_item) = self.stack.pop() {
    //                         self.prev_item = Some(prev_item);
    //                     }
    //                 }
    //             }
    //             _ => contained = false,
    //         }
    //     }

    //     let mut start_boundary = snapshot
    //         .buffer_snapshot
    //         .anchor_before(Point::new(0, 0));
    //     if let Some(prev_item) = prev_outline {
    //         let prev_end = prev_item.range.end.to_offset(&snapshot.buffer_snapshot);
    //         let start_boundary = snapshot
    //             .buffer_snapshot
    //             .anchor_after(Point::new(prev_end, 0));
    //     }
    //     let end_boundary = position.clone();
    //     let search_range = start_boundary..end_boundary;
    //     let search_results = snapshot.
    //         buffer_snapshot
    //         .symbols_containing_in_range(search_range.clone(), position.clone(), cx)
    //         .into_iter()
    //         .collect();
    //     for outline in search_results.iter() {
    //         self.stack.push(outline.clone());
    //     }
    // }

    // pub fn sticky_snapshot(&self) -> Vec<OutlineItem<Anchor>> {
    //     self.stack.clone()
    // }

    pub fn symbols_containing(&self, position: Anchor, snapshot: &DisplaySnapshot, cx: &ViewContext<Editor>) -> Vec<StickyLine> {
        (_buffer_ids, symbols) = snapshot.buffer_snapshot.symbols_containing(position, cx)?;
        let mut sticky_lines: Vec<StickyLine> = Vec::new();
        for symbol in symbols {
            let start = symbol.range.start.to_offset(&snapshot.buffer_snapshot);
            sticky_lines.push(StickyLine {
                line: MultiBufferRow(start),
            })
        }

        sticky_lines
    }
}

impl Editor {
    // pub fn fetch_sticky_elements(
    //     &self,
    //     visible_buffer_range: Range<MultiBufferRow>,
    //     snapshot: &DisplaySnapshot,
    //     cx: &mut ViewContext<Editor>,
    // ) -> Option<Vec<OutlineItem<Anchor>>>> {
    //     let enable_sticky_scroll = self.should_show_sticky_elements().unwrap_or_default(|| {
    //         if let Some(buffer) = self.buffer().read(cx).as_singleton() {
    //             language_settings(buffer.read(cx).language(), buffer.read(cx).file(), cx)
    //                 .enable_sticky_scroll
    //                 .enabled
    //         } else {
    //             true
    //         }
    //     })

    //     if !enable_sticky_scroll {
    //         return None;
    //     }

    //     let visible_range_start = snapshot
    //         .buffer_snapshot
    //         .anchor_after(Point::new(visible_buffer_range.start.0, 0));

    //     self.sticky_scroll_manager.update(visible_range_start, snapshot, cx);
    //     Some(self.sticky_scroll_manager.sticky_snapshot())
    // }

    pub fn get_sticky_lines(
        &self,
        visible_buffer_range: Range<MultiBufferRow>,
        snapshot: &DisplaySnapshot,
        cx: &mut ViewContext<Editor>,
    ) -> Option<Vec<StickyLine>> {
        let visible_range_start = snapshot
            .buffer_snapshot
            .anchor_after(Point::new(visible_buffer_range.start.0, 0));
        self.sticky_scroll_manager
            .symbols_containing(visible_range_start, snapshot, cx)
    }
}

// fn is_contained_in_scope(outlin_item: OutlineElement<Anchor>, position_offset: u32, snapshot: &DisplaySnapshot) -> bool {
//     let scope_start_offset = outline_item.range.start.to_offset(&snapshot.buffer_snapshot);
//     if position_offset < scope_start_offset {
//         false 
//     }
//     let scope_end_offset = outline_item.range.end.to_offset(&snapshot.buffer_snapshot);
//     if position_offset > scope_end_offset {
//         false
//     }
//     true
// }