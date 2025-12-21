#![allow(dead_code)]

mod ext;
mod hangul;
mod hangul_parser;
mod jamo;
mod syllable;
mod terminal;

use std::io;

use crossterm::event::KeyEventKind;
pub use crossterm::{
    Command, cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self as cross_terminal, ClearType},
};

use crate::terminal::{Terminal, TerminalResult};

const MENU: &str = r#"Crossterm interactive test

Controls:

 - 'q' - quit interactive test (or return to this menu)
 - any other key - continue with next step

Available tests:

1. cursor
2. color (foreground, background)
3. attributes (bold, italic, ...)
4. input
5. synchronized output

Select test to run ('1', '2', ...) or hit 'q' to quit.
"#;

fn run<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    execute!(w, cross_terminal::EnterAlternateScreen)?;

    cross_terminal::enable_raw_mode()?;

    loop {
        queue!(
            w,
            style::ResetColor,
            cross_terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;

        for line in MENU.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        w.flush()?;

        match read_char()? {
            // '1' => test::cursor::run(w)?,
            // '2' => test::color::run(w)?,
            // '3' => test::attribute::run(w)?,
            // '4' => test::event::run(w)?,
            // '5' => test::synchronized_output::run(w)?,
            'q' => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                break;
            }
            _ => {}
        };
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        cross_terminal::LeaveAlternateScreen
    )?;

    cross_terminal::disable_raw_mode()
}

pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

pub fn buffer_size() -> io::Result<(u16, u16)> {
    cross_terminal::size()
}

fn main() -> TerminalResult<()> {
    pretty_env_logger::init();
    let mut term = Terminal::default();
    term.run()
    // let mut stdout = io::stdout();
    // run(&mut stdout)
}

/*https://www.lennyfacecopypaste.com/text-symbols/line.html
Glyph                       ACS            Ascii     acsc     acsc
Name                        Name           Default   Char     Value
────────────────────────────────────────────────────────────────────
arrow pointing right        ACS_RARROW     >         +        0x2b
arrow pointing left         ACS_LARROW     <         ,        0x2c
arrow pointing up           ACS_UARROW     ^         -        0x2d
arrow pointing down         ACS_DARROW     v         .        0x2e
solid square block          ACS_BLOCK      #         0        0x30
diamond                     ACS_DIAMOND    +         `        0x60
checker board (stipple)     ACS_CKBOARD    :         a        0x61
degree symbol               ACS_DEGREE     \         f        0x66
plus/minus                  ACS_PLMINUS    #         g        0x67
board of squares            ACS_BOARD      #         h        0x68
lantern symbol              ACS_LANTERN    #         i        0x69
lower right corner          ACS_LRCORNER   +         j        0x6a
upper right corner          ACS_URCORNER   +         k        0x6b
upper left corner           ACS_ULCORNER   +         l        0x6c
lower left corner           ACS_LLCORNER   +         m        0x6d
large plus or crossover     ACS_PLUS       +         n        0x6e
scan line 1                 ACS_S1         ~         o        0x6f
scan line 3                 ACS_S3         -         p        0x70
horizontal line             ACS_HLINE      -         q        0x71
scan line 7                 ACS_S7         -         r        0x72
scan line 9                 ACS_S9         _         s        0x73
tee pointing right          ACS_LTEE       +         t        0x74
tee pointing left           ACS_RTEE       +         u        0x75
tee pointing up             ACS_BTEE       +         v        0x76
tee pointing down           ACS_TTEE       +         w        0x77
vertical line               ACS_VLINE      |         x        0x78
less-than-or-equal-to       ACS_LEQUAL     <         y        0x79
greater-than-or-equal-to    ACS_GEQUAL     >         z        0x7a
greek pi                    ACS_PI         *         {        0x7b
not-equal                   ACS_NEQUAL     !         |        0x7c
UK pound sign               ACS_STERLING   f         }        0x7d
bullet                      ACS_BULLET     o         ~        0x7e
*/
