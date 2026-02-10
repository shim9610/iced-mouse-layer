//! A widget that displays content following the mouse cursor.
//!
//! # Example
//!
//! ```no_run
//! use iced::widget::mouse_layer;
//!
//! let layer = mouse_layer(some_content)
//!     .offset(10.0, 10.0)
//!     .scale(0.9);
//! ```
use iced::advanced::layout::{self, Layout};
use iced::advanced::mouse;
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::{Element, Length, Point, Rectangle, Size, Vector, Theme, Renderer,Event};


/// A widget that displays content following the mouse cursor.
pub struct MouseLayer<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    offset: Vector,
}

impl<'a, Message, Theme, Renderer> MouseLayer<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    /// Creates a new [`MouseLayer`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            offset: Vector::ZERO,
        }
    }

    /// Sets the offset from the cursor position.
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.offset = Vector::new(x, y);
        self
    }
}

#[derive(Default)]
struct State {
    cursor_position: Option<Point>,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for MouseLayer<'_, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::default())
    }

    fn children(&self) -> Vec<widget::Tree> {
        vec![widget::Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&[&self.content]);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink) 
    }

    fn layout(
        &mut self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::ZERO)
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
       
    }
    fn update(
        &mut self,
        tree: &mut widget::Tree,
        _event: &Event,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let new_pos = cursor.position();        // Option<Point>
        if state.cursor_position != new_pos{
            state.cursor_position = new_pos;
            shell.request_redraw();
        }
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_ref::<State>();

        Some(overlay::Element::new(Box::new(Overlay {
            content: &mut self.content,
            tree: &mut tree.children[0],
            cursor_position: state.cursor_position,
            offset: self.offset,
            translation,
            viewport: *viewport,
        })))
    }
}

impl<'a, Message, Theme, Renderer> From<MouseLayer<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(layer: MouseLayer<'a, Message, Theme, Renderer>) -> Self {
        Element::new(layer)
    }
}

// --- Overlay ---

struct Overlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    content: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut widget::Tree,
    cursor_position: Option<Point>,
    offset: Vector,
    translation: Vector,
    viewport: Rectangle,
}

impl<Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for Overlay<'_, '_, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, _bounds: Size) -> layout::Node {
        let cursor_pos = self.cursor_position;
        
        let Some(cursor) = cursor_pos else {
            return layout::Node::new(Size::ZERO);
        };

        let limits = layout::Limits::new(Size::ZERO, self.viewport.size());
        
        let content_layout = self.content.as_widget_mut().layout(
            self.tree,
            renderer,
            &limits,
        );

        let position = Point::new(
            cursor.x + self.offset.x + self.translation.x,
            cursor.y + self.offset.y + self.translation.y,
        );

        layout::Node::with_children(
            content_layout.bounds().size(),
            vec![content_layout],
        )
        .move_to(position)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        if self.cursor_position.is_none() {
            return;
        }

        let Some(content_layout) = layout.children().next() else {
            return;
        };

        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            &Rectangle::with_size(Size::INFINITE),
        );
    }

    fn update(
        &mut self,
        _event: &Event,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let new_pos = cursor.position();
        if self.cursor_position != new_pos {
            self.cursor_position = new_pos;
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}

/// Creates a new [`MouseLayer`].
pub fn mouse_layer<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> MouseLayer<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    MouseLayer::new(content)
}