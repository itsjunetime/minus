use crate::{
    input::{InputEvent, InputHandler},
    LineNumbers, Pager, SearchMode,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

// Just a transparent function to fix incompatiblity issues between
// versions
// TODO: Remove this later in favour of how handle_event should actually be called
fn handle_input(ev: Event, p: &Pager) -> Option<InputEvent> {
    p.input_handler
        .handle_input(ev, p.upper_mark, p.search_mode, p.line_numbers, p.rows)
}

#[test]
#[allow(clippy::too_many_lines)]
fn input_handling() {
    let mut pager = Pager::new();
    pager.upper_mark = 12;
    pager.set_line_numbers(LineNumbers::Enabled);
    pager.rows = 5;
    pager.running = true;

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(pager.upper_mark + 1)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(pager.upper_mark - 1)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        });
        // Pager for local use
        let mut pager = Pager::new();
        pager.upper_mark = usize::MAX;
        pager.set_line_numbers(LineNumbers::Enabled);
        pager.rows = 5;
        pager.running = true;
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(usize::MAX)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        });
        // Pager for local use
        let mut pager = Pager::new();
        pager.upper_mark = usize::MIN;
        pager.set_line_numbers(LineNumbers::Enabled);
        pager.rows = 5;
        pager.running = true;
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(usize::MIN)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Mouse(MouseEvent {
            kind: MouseEventKind::ScrollDown,
            row: 0,
            column: 0,
            modifiers: KeyModifiers::NONE,
        });

        assert_eq!(
            Some(InputEvent::UpdateUpperMark(pager.upper_mark + 5)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Mouse(MouseEvent {
            kind: MouseEventKind::ScrollUp,
            row: 0,
            column: 0,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(pager.upper_mark - 5)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('g'),
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(0)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::PageUp,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            // rows is 5, therefore upper_mark = upper_mark - rows -1
            Some(InputEvent::UpdateUpperMark(8)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('g'),
            modifiers: KeyModifiers::SHIFT,
        });
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(usize::MAX)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('G'),
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(usize::MAX)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('G'),
            modifiers: KeyModifiers::SHIFT,
        });
        assert_eq!(
            Some(InputEvent::UpdateUpperMark(usize::MAX)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::PageDown,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            // rows is 5, therefore upper_mark = upper_mark - rows -1
            Some(InputEvent::UpdateUpperMark(16)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Resize(42, 35);
        assert_eq!(
            Some(InputEvent::UpdateTermArea(42, 35)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::CONTROL,
        });
        assert_eq!(
            Some(InputEvent::UpdateLineNumber(!pager.line_numbers)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(Some(InputEvent::Exit), handle_input(ev, &pager));
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        });
        assert_eq!(Some(InputEvent::Exit), handle_input(ev, &pager));
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(None, handle_input(ev, &pager));
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('/'),
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            Some(InputEvent::Search(SearchMode::Forward)),
            handle_input(ev, &pager)
        );
    }

    {
        let ev = Event::Key(KeyEvent {
            code: KeyCode::Char('?'),
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            Some(InputEvent::Search(SearchMode::Reverse)),
            handle_input(ev, &pager)
        );
    }
}
