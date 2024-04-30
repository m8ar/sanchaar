use iced::advanced::widget::{operation, tree, Operation};
use iced::advanced::{layout, overlay, renderer, widget, Clipboard, Layout, Shell, Widget};
use iced::widget::{button, column, container, text};
use iced::{
    event, mouse, Command, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme, Vector,
};

use super::double_pass::double_pass;

#[derive(Debug, Clone)]
pub struct MenuButton<'a, M: Clone + 'a> {
    content: &'a str,
    message: M,
}

pub fn menu_item<'a, M: Clone + 'a>(content: &'a str, message: M) -> MenuButton<'a, M> {
    MenuButton { content, message }
}

fn menu_button<'a, M: Clone + 'a>(entry: MenuButton<'a, M>, length: Length) -> Element<'a, M> {
    button(text(entry.content).size(15))
        .padding([2, 4])
        .width(length)
        .style(|theme, status| match status {
            button::Status::Pressed | button::Status::Hovered => button::secondary(theme, status),
            _ => button::text(theme, status),
        })
        .on_press(entry.message)
        .into()
}

pub fn context_menu<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    entries: Vec<MenuButton<'a, Message>>,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    let build_menu = |length: Length| {
        container(
            column(
                entries
                    .iter()
                    .cloned()
                    .map(|entry| menu_button(entry, length)),
            )
            .spacing(2),
        )
        .padding(4)
        .style(container::rounded_box)
    };

    let menu = double_pass(build_menu(Length::Shrink), build_menu(Length::Fill));

    ContextMenu {
        base: base.into(),
        menu,
    }
    .into()
}

struct ContextMenu<'a, Message> {
    base: Element<'a, Message>,
    menu: Element<'a, Message>,
}

#[derive(Debug, Clone, Copy)]
enum State {
    Closed,
    Open(Point),
}

impl State {
    fn open(self) -> Option<Point> {
        match self {
            State::Closed => None,
            State::Open(point) => Some(point),
        }
    }
}

impl<'a, Message> Widget<Message, Theme, Renderer> for ContextMenu<'a, Message> {
    fn size(&self) -> Size<Length> {
        self.base.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.base.as_widget().size_hint()
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.base
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.base.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::Closed)
    }

    fn children(&self) -> Vec<widget::Tree> {
        vec![widget::Tree::new(&self.base), widget::Tree::new(&self.menu)]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&[&self.base, &self.menu]);
    }

    fn operate(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        let state = tree.state.downcast_mut::<State>();

        operation.custom(state, None);

        self.base
            .as_widget()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<State>();

        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) = &event {
            if let Some(position) = cursor.position_over(layout.bounds()) {
                *state = State::Open(position);
            }
        }

        self.base.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        _tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        cursor
            .is_over(layout.bounds())
            .then_some(mouse::Interaction::Pointer)
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<State>();

        let (first, second) = tree.children.split_at_mut(1);

        let base = self
            .base
            .as_widget_mut()
            .overlay(&mut first[0], layout, renderer, translation);

        let overlay = state.open().map(|position| {
            overlay::Element::new(Box::new(Overlay {
                content: &mut self.menu,
                tree: &mut second[0],
                state,
                position: position + translation,
            }))
        });

        Some(overlay::Group::with_children(base.into_iter().chain(overlay).collect()).overlay())
    }
}

pub fn close<Message: 'static>(f: fn(bool) -> Message) -> Command<Message> {
    struct Close<T> {
        any_closed: bool,
        f: fn(bool) -> T,
    }

    impl<T> Operation<T> for Close<T> {
        fn container(
            &mut self,
            _id: Option<&widget::Id>,
            _bounds: Rectangle,
            operate_on_children: &mut dyn FnMut(&mut dyn Operation<T>),
        ) {
            operate_on_children(self)
        }

        fn custom(&mut self, state: &mut dyn std::any::Any, _id: Option<&widget::Id>) {
            if let Some(state) = state.downcast_mut::<State>() {
                if let State::Open(_) = *state {
                    *state = State::Closed;
                    self.any_closed = true;
                }
            }
        }

        fn finish(&self) -> operation::Outcome<T> {
            operation::Outcome::Some((self.f)(self.any_closed))
        }
    }

    Command::widget(Close {
        any_closed: false,
        f,
    })
}

impl<'a, Message> From<ContextMenu<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(context_menu: ContextMenu<'a, Message>) -> Self {
        Element::new(context_menu)
    }
}

struct Overlay<'a, 'b, Message> {
    content: &'b mut Element<'a, Message>,
    tree: &'b mut widget::Tree,
    state: &'b mut State,
    position: Point,
}

impl<'a, 'b, Message> overlay::Overlay<Message, Theme, Renderer> for Overlay<'a, 'b, Message> {
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds)
            .width(Length::Fill)
            .height(Length::Fill);

        let node = self
            .content
            .as_widget()
            .layout(self.tree, renderer, &limits);

        let viewport = Rectangle::new(Point::ORIGIN, bounds);
        let mut bounds = Rectangle::new(self.position, node.size());

        if bounds.x < viewport.x {
            bounds.x = viewport.x;
        } else if viewport.x + viewport.width < bounds.x + bounds.width {
            bounds.x = viewport.x + viewport.width - bounds.width;
        }

        if bounds.y < viewport.y {
            bounds.y = viewport.y;
        } else if viewport.y + viewport.height < bounds.y + bounds.height {
            bounds.y = viewport.y + viewport.height - bounds.height;
        }

        node.move_to(bounds.position())
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            &layout.bounds(),
        );
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.content
            .as_widget_mut()
            .operate(self.tree, layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        if let Event::Mouse(mouse::Event::ButtonPressed(_)) = &event {
            if cursor.position_over(layout.bounds()).is_none() {
                *self.state = State::Closed;
            }
        }

        if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) = &event {
            if cursor.position_over(layout.bounds()).is_some() {
                *self.state = State::Closed;
            }
        }

        self.content.as_widget_mut().on_event(
            self.tree,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        self.content
            .as_widget()
            .mouse_interaction(self.tree, layout, cursor, viewport, renderer)
    }

    fn is_over(&self, layout: Layout<'_>, _renderer: &Renderer, cursor_position: Point) -> bool {
        layout.bounds().contains(cursor_position)
    }
}
