use piston_window::*;
use sdl2_window::*;
use crate::globals;

pub struct Assets {
    pub px_plus_font: Glyphs,
}

const FONT_FOLDER: &str = "assets/fonts";
const PX_PLUS_FONT: &str = "PxPlus_IBM_VGA8.ttf";

impl Assets {
    pub fn new(_window: &mut PistonWindow<Sdl2Window>) -> Self {

        let font_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder(FONT_FOLDER)
            .unwrap();

        let px_plus_font: Glyphs = _window.load_font(font_folder.join(PX_PLUS_FONT)).unwrap();

        Assets {
            px_plus_font: px_plus_font,
        }
    }

    pub fn render_text(&mut self,
            _font_size: u32,
            _text: &str,
            _pos: [f64; 2],
            _c: &Context,
            _gl: &mut G2d,
        )
    {
        let _transform = _c.transform.trans(_pos[0], _pos[1]);
        let _font = &mut self.px_plus_font;
        match text::Text::new_color(globals::BLACK, _font_size).draw(
            _text,
            _font,
            &DrawState::default(),
            _transform,
            _gl,
        ) {
            Ok(()) => (),
            Err(_err) => (),
        }
    }
}