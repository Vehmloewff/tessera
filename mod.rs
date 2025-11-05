mod element;
mod event_listeners;
mod htmx_attributes;
mod internal_buffer;
mod standard_atrributes;
mod tag;
mod template;

use internal_buffer::InternalBuffer;

pub use element::Element;
pub use event_listeners::EventListeners;
pub use htmx_attributes::*;
pub use standard_atrributes::*;
pub use tag::*;
pub use template::Template;

pub struct HtmlBuffer {
    internal_buffer: InternalBuffer,
}

impl HtmlBuffer {
    pub fn new() -> Self {
        Self {
            internal_buffer: InternalBuffer::new(),
        }
    }

    pub fn template(&mut self) -> Template<'_> {
        Template::new(&mut self.internal_buffer)
    }

    pub fn as_str(&mut self) -> &str {
        self.internal_buffer.as_str()
    }

    pub fn into_string(self) -> String {
        self.internal_buffer.into_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_borrowing() {
        let mut buffer = HtmlBuffer::new();
        let mut t = buffer.template();

        t.div().id("wrapper").children(|t| {
            t.button().text("Click me!");
            t.span().text("Click me!");
            t.input().oninput("alert('dude! you typed!')");
        });

        assert_eq!(
            buffer.as_str(),
            "<div id=\"wrapper\"><button>Click me!</button><span>Click me!</span><input oninput=\"alert('dude! you typed!')\"></div>"
        );
    }

    #[test]
    fn test_svg_support() {
        let mut buffer = HtmlBuffer::new();
        let mut t = buffer.template();

        t.svg()
            .view_box("0 0 100 100")
            .xmlns("http://www.w3.org/2000/svg")
            .width("100")
            .height("100")
            .children(|t| {
                t.circle()
                    .cx("50")
                    .cy("50")
                    .r("40")
                    .fill("red")
                    .stroke("black")
                    .stroke_width("3");

                t.rect()
                    .x("10")
                    .y("10")
                    .width("30")
                    .height("30")
                    .fill("blue");

                t.path()
                    .d("M 10,30 A 20,20 0,0,1 50,30")
                    .stroke("green")
                    .fill("none");
            });

        assert_eq!(
            buffer.as_str(),
            "<svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"100\"><circle cx=\"50\" cy=\"50\" r=\"40\" fill=\"red\" stroke=\"black\" stroke-width=\"3\"></circle><rect x=\"10\" y=\"10\" width=\"30\" height=\"30\" fill=\"blue\"></rect><path d=\"M 10,30 A 20,20 0,0,1 50,30\" stroke=\"green\" fill=\"none\"></path></svg>"
        );
    }
}
