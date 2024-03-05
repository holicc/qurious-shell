mod completer;
mod highlighter;

use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{config, Cmd, CompletionType, EditMode, Editor, KeyEvent};
use rustyline::{Completer, Helper, Hinter, Validator};
use std::borrow::Cow::{self, Borrowed, Owned};

use self::highlighter::ShellHighlighter;

pub const BANNER: &str = r#" 
     ██████   ██    ██  ██  ██████    ██████   ██    ██  ███████ 
    ██    ██  ██    ██  ██  ██   ██  ██    ██  ██    ██  ██      
    ██    ██  ██    ██  ██  ██████   ██    ██  ██    ██  ███████ 
    ██ ▄▄ ██  ██    ██  ██  ██   ██  ██    ██  ██    ██       ██ 
     ██████    ██████   ██  ██   ██   ██████    ██████   ███████ 
       ▀▀                                                 

    Welcome to Qurious Shell
    Type 'help' to get started
    Version 0.1.0
"#;

const HELP_MESSAGE: &'static str = r#"
    Qurious Command Line Help
    Qurious is a SQL query engine built in Rust.

    Usage
        qurious [options] [query]
    Options
        -h or --help: Display this help information.
        -v or --version: Show the version number of Qurious.
        -f or --file: Specify the SQL query file to execute.
        -c or --connect: Specify the database to connect to.
        -u or --username: Define the username for the database.
        -p or --password: Set the password for the database.
"#;

#[derive(Helper, Completer, Hinter, Validator)]
pub struct ReplHelper<'a> {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    highlighter: ShellHighlighter<'a>,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl<'a> Highlighter for ReplHelper<'a> {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

pub fn run() {
    let prompt = "Qurious> ";
    let cfg = config::Builder::new()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Vi)
        .auto_add_history(true)
        .history_ignore_space(true)
        .build();

    let mut repl = Editor::with_config(cfg).expect("Failed to create editor");

    repl.set_helper(Some(ReplHelper {
        completer: FilenameCompleter::new(),
        highlighter: ShellHighlighter::new(),
        hinter: HistoryHinter::new(),
        colored_prompt: format!("\x1b[1;32m{prompt}\x1b[0m"),
        validator: MatchingBracketValidator::new(),
    }));

    repl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    repl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);

    println!("{}", BANNER);

    loop {
        let readline = repl.readline(&prompt);
        match readline {
            Ok(line) => {
                println!("Line: {line}");
            }
            Err(ReadlineError::Interrupted) => {
                println!("Bye 👋!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Encountered Eof");
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }
}
