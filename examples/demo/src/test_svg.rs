use kano::{platform::Platform, View};

pub fn TestSvg<P: Platform>() -> impl View<P>
where
    P::Cursor: kano_svg::SvgCursor,
{
    // Taken from https://commons.wikimedia.org/wiki/File:Test.svg
    kano::svg_view!("resources/test.svg")
}
