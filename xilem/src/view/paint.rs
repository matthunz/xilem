use accesskit::Role;
use masonry::{
    AccessCtx, BoxConstraints, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, StatusChange,
    Widget, WidgetId,
};
use smallvec::SmallVec;
use vello::Scene;
use xilem_core::{View, ViewElement, ViewId, ViewMarker, ViewPathTracker};

pub struct Paint<V, E> {
    view: V,
    paint_fn: fn(&mut E, &mut PaintCtx, &mut Scene),
}

impl<V, E> Paint<V, E> {
    pub(crate) fn new(view: V, paint_fn: fn(&mut E, &mut PaintCtx, &mut Scene)) -> Self {
        Self { view, paint_fn }
    }
}

impl<V, E> ViewMarker for Paint<V, E> {}

impl<State, Action, Context, Message, V> View<State, Action, Context, Message>
    for Paint<V, V::Element>
where
    Context: ViewPathTracker,
    V: View<State, Action, Context, Message>,
    V::Element: Widget,
{
    type Element = PaintElement<V::Element>;

    type ViewState = V::ViewState;

    fn build(&self, ctx: &mut Context) -> (Self::Element, Self::ViewState) {
        let (element, state) = self.view.build(ctx);
        (
            PaintElement {
                element,
                paint_fn: self.paint_fn,
            },
            state,
        )
    }

    fn rebuild<'el>(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut Context,
        element: xilem_core::Mut<'el, Self::Element>,
    ) -> xilem_core::Mut<'el, Self::Element> {
        todo!()
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut Context,
        element: xilem_core::Mut<'_, Self::Element>,
    ) {
        todo!()
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        message: Message,
        app_state: &mut State,
    ) -> xilem_core::MessageResult<Action, Message> {
        self.view.message(view_state, id_path, message, app_state)
    }
}

pub struct PaintElement<E> {
    element: E,
    paint_fn: fn(&mut E, &mut PaintCtx, &mut Scene),
}

impl<E> ViewElement for PaintElement<E>
where
    E: ViewElement + 'static,
{
    type Mut<'a> = &'a mut PaintElement<E>;
}

impl<E: Widget> Widget for PaintElement<E> {
    fn on_status_change(&mut self, ctx: &mut LifeCycleCtx, event: &StatusChange) {
        self.element.on_status_change(ctx, event)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        self.element.lifecycle(ctx, event);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        self.element.layout(ctx, bc)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        (self.paint_fn)(&mut self.element, ctx, scene);
    }

    fn accessibility_role(&self) -> Role {
        self.element.accessibility_role()
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx) {
        self.element.accessibility(ctx)
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        self.element.children_ids()
    }

    fn on_pointer_event(&mut self, ctx: &mut masonry::EventCtx, event: &masonry::PointerEvent) {
        self.element.on_pointer_event(ctx, event)
    }

    fn on_text_event(&mut self, ctx: &mut masonry::EventCtx, event: &masonry::TextEvent) {
        self.element.on_text_event(ctx, event)
    }

    fn on_access_event(&mut self, ctx: &mut masonry::EventCtx, event: &masonry::AccessEvent) {
        self.element.on_access_event(ctx, event)
    }

    fn compose(&mut self, ctx: &mut masonry::ComposeCtx) {
        self.element.compose(ctx)
    }

    fn skip_pointer(&self) -> bool {
        self.element.skip_pointer()
    }

    fn get_debug_text(&self) -> Option<String> {
        self.element.get_debug_text()
    }

    fn get_cursor(&self) -> masonry::CursorIcon {
        self.element.get_cursor()
    }
}
