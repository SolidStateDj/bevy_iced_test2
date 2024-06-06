// use bevy_iced::iced::advanced::layout::{self, Layout};
// use bevy_iced::iced::advanced::renderer;
// use bevy_iced::iced::advanced::widget::{self, Widget};
use bevy_iced::advanced::layout::{self, Layout};
use bevy_iced::advanced::renderer;
use bevy_iced::advanced::widget::{self, Widget};
use bevy_iced::iced::mouse;
// use bevy_iced::mouse;
use bevy_iced::iced::{Border, Color, Element, Length, Rectangle, Shadow, Size};

pub struct CustomQuad {
    color: Color,
    size: f32,
    radius: [f32; 4],
    shadow: Shadow,
}

impl CustomQuad {
    pub fn new(color: Color, size: f32, radius: [f32; 4], shadow: Shadow) -> Self {
        Self {
            color,
            size,
            radius,
            shadow,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for CustomQuad
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.size, self.size))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    radius: self.radius.into(),
                    width: 0.0,
                    color: Color::from_rgb(1.0, 0.0, 0.0),
                },
                shadow: self.shadow,
            },
            self.color,
        );
    }
}

impl<'a, Message> From<CustomQuad> for Element<'a, Message> {
    fn from(circle: CustomQuad) -> Self {
        Self::new(circle)
    }
}
