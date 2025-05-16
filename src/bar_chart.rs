use iced::advanced::renderer;
use iced::Theme;

pub struct BarChart<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog
{

}

pub trait Catalog {
    ///Style for the trait to use.
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self, Style>;

    fn default<'a>() -> Self::Class<'a> {
        todo!();
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

enum Status {

}

pub struct Style {

}

pub type StyleFn<'a, Theme, Style> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;