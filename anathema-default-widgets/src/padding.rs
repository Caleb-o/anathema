use std::ops::ControlFlow;

use anathema::CommonVal;
use anathema_geometry::Size;
use anathema_widgets::layout::{Constraints, LayoutCtx, PositionCtx};
use anathema_widgets::{AttributeStorage, LayoutChildren, PositionChildren, Widget, WidgetId};

#[derive(Default)]
struct PaddingValues {
    top: u16,
    right: u16,
    bottom: u16,
    left: u16,
}

impl PaddingValues {
    fn size(&self) -> Size {
        Size {
            height: (self.top + self.bottom) as usize,
            width: (self.left + self.right) as usize,
        }
    }
}

#[derive(Default)]
pub struct Padding(PaddingValues);

impl Widget for Padding {
    fn layout<'bp>(
        &mut self,
        mut children: LayoutChildren<'_, '_, 'bp>,
        mut constraints: Constraints,
        id: WidgetId,
        ctx: &mut LayoutCtx<'_, '_, 'bp>,
    ) -> Size {
        let attributes = ctx.attribs.get(id);
        let mut size = Size::ZERO;
        let padding = attributes.get("padding").unwrap_or(0);
        let padding_right = attributes.get("padding-right").unwrap_or(0);
        let padding_bottom = attributes.get("padding-bottom").unwrap_or(0);
        let padding_left = attributes.get("padding-left").unwrap_or(0);

        self.0.top = attributes.get("padding-top").unwrap_or(padding);
        self.0.right = attributes.get("padding-right").unwrap_or(padding);
        self.0.bottom = attributes.get("padding-bottom").unwrap_or(padding);
        self.0.left = attributes.get("padding-left").unwrap_or(padding);

        let padding_size = self.0.size();

        children.for_each(|child, children| {
            constraints.sub_max_width(padding_size.width);
            constraints.sub_max_height(padding_size.height);
            let mut child_size = child.layout(children, constraints, ctx);
            child_size += padding_size;
            size.width = child_size.width.max(size.width);
            size.height = child_size.height.max(size.height);

            ControlFlow::Break(())
        });

        size.width = constraints.min_width.max(size.width).min(constraints.max_width());
        size.height = constraints.min_height.max(size.height).min(constraints.max_height());

        size
    }

    fn position<'bp>(
        &mut self,
        mut children: PositionChildren<'_, '_, 'bp>,
        id: WidgetId,
        attribute_storage: &AttributeStorage<'bp>,
        mut ctx: PositionCtx,
    ) {
        children.for_each(|child, children| {
            ctx.pos.y += self.0.top as i32;

            ctx.pos.x += self.0.left as i32;

            child.position(children, ctx.pos, attribute_storage);
            ControlFlow::Break(())
        });
    }
}