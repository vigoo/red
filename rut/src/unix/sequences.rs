
#[derive(Debug, Copy, Clone)]
pub struct FunctionSequences {
    pub enter: &'static str,
    pub exit: &'static str,
    pub show_cursor: &'static str,
    pub hide_cursor: &'static str,
    pub clear_screen: &'static str,
    pub reset_all_attributes: &'static str,
    pub underline: &'static str,
    pub bold: &'static str,
    pub blink: &'static str,
    pub reverse: &'static str,
    pub enter_keypad: &'static str,
    pub exit_keypad: &'static str,
    pub enter_mouse: &'static str,
    pub exit_mouse: &'static str,
}

const RXVT256COLOR: FunctionSequences = FunctionSequences {
    enter: "\x1b7\x1b[?47h",
    exit: "\x1b[2J\x1b[?47l\x1b8",
    show_cursor: "\x1b[?25h",
    hide_cursor: "\x1b[?25l",
    clear_screen: "\x1b[H\x1b[2J",
    reset_all_attributes: "\x1b[m",
    underline: "\x1b[4m",
    bold: "\x1b[1m",
    blink: "\x1b[5m",
    reverse: "\x1b[7m",
    enter_keypad: "\x1b=",
    exit_keypad: "\x1b>",
    enter_mouse: "\x1b[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h",
    exit_mouse: "\x1b[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l"
};

const ETERM: FunctionSequences = FunctionSequences {
    enter: "\x1b7\x1b[?47h",
    exit: "\x1b[2J\x1b[?47l\x1b8",
    show_cursor: "\x1b[?25h",
    hide_cursor: "\x1b[?25l",
    clear_screen: "\x1b[H\x1b[2J",
    reset_all_attributes: "\x1b[m",
    underline: "\x1b[4m",
    bold: "\x1b[1m",
    blink: "\x1b[5m",
    reverse: "\x1b[7m",
    enter_keypad: "",
    exit_keypad: "",
    enter_mouse: "",
    exit_mouse: ""
};

const SCREEN: FunctionSequences = FunctionSequences {
    enter: "\x1b[?1049h",
    exit: "\x1b[?1049l",
    show_cursor: "\x1b[34h\x1b[?25h",
    hide_cursor: "\x1b[?25l",
    clear_screen: "\x1b[H\x1b[2J",
    reset_all_attributes: "\x1b[m",
    underline: "\x1b[4m",
    bold: "\x1b[1m",
    blink: "\x1b[5m",
    reverse: "\x1b[7m",
    enter_keypad: "\x1b[?1h\x1b=",
    exit_keypad: "\x1b[?1l\x1b>",
    enter_mouse: "\x1b[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h",
    exit_mouse: "\x1b[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l"
};

const RXVTUNICODE: FunctionSequences = FunctionSequences {
    enter: "\x1b[?1049h",
    exit: "\x1b[r\x1b[?1049l",
    show_cursor: "\x1b[?25h",
    hide_cursor: "\x1b[?25l",
    clear_screen: "\x1b[H\x1b[2J",
    reset_all_attributes: "\x1b[m\x1b(B",
    underline: "\x1b[4m",
    bold: "\x1b[1m",
    blink: "\x1b[5m",
    reverse: "\x1b[7m",
    enter_keypad: "\x1b=",
    exit_keypad: "\x1b>",
    enter_mouse: "\x1b[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h",
    exit_mouse: "\x1b[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l"
};

const LINUX: FunctionSequences = FunctionSequences {
    enter: "",
    exit: "",
    show_cursor: "\x1b[?25h\x1b[?0c",
    hide_cursor: "\x1b[?25l\x1b[?1c",
    clear_screen: "\x1b[H\x1b[J",
    reset_all_attributes: "\x1b[0;10m",
    underline: "\x1b[4m",
    bold: "\x1b[1m",
    blink: "\x1b[5m",
    reverse: "\x1b[7m",
    enter_keypad: "",
    exit_keypad: "",
    enter_mouse: "",
    exit_mouse: ""
};

const XTERM: FunctionSequences = FunctionSequences {
    enter: "\x1b[?1049h",
    exit: "\x1b[?1049l",
    show_cursor: "\x1b[?12l\x1b[?25h",
    hide_cursor: "\x1b[?25l",
    clear_screen: "\x1b[H\x1b[2J",
    reset_all_attributes: "\x1b(B\x1b[m",
    underline: "\x1b[4m",
    bold: "\x1b[1m",
    blink: "\x1b[5m",
    reverse: "\x1b[7m",
    enter_keypad: "\x1b[?1h\x1b=",
    exit_keypad: "\x1b[?1l\x1b>",
    enter_mouse: "\x1b[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h",
    exit_mouse: "\x1b[?1006l\x1b[?1015l\x1b[?1002l\x1b[?1000l"
};

pub const TERM_MAPPING: [(&'static str, &'static FunctionSequences); 6] = [
    ("rxvt-256color", &RXVT256COLOR),
    ("Eterm", &ETERM),
    ("screen", &SCREEN),
    ("rxvt-unicode", &RXVTUNICODE),
    ("linux", &LINUX),
    ("xterm", &XTERM),
];

/*
  23   │
  24   │ // rxvt-256color
  25   │ static const char *rxvt_256color_keys[] = {
  26   │        "\x1b[11~","\x1b[12~","\x1b[13~","\x1b[14~","\x1b[15~","\x1b[17~","\x1b[18~","\x1b[19~","\x1b[20~","\x1b[21~","\x1b[23~","\x1b[24~","\x1b[2~","\x1b[3~","\x1b[7~","\x1b[8~","\x1b[5~","\x1b[6~","\x1b[A","\x1b[B","\x1b[D","\0
  31   │
  32   │ // Eterm
  33   │ static const char *eterm_keys[] = {
  34   │        "\x1b[11~","\x1b[12~","\x1b[13~","\x1b[14~","\x1b[15~","\x1b[17~","\x1b[18~","\x1b[19~","\x1b[20~","\x1b[21~","\x1b[23~","\x1b[24~","\x1b[2~","\x1b[3~","\x1b[7~","\x1b[8~","\x1b[5~","\x1b[6~","\x1b[A","\x1b[B","\x1b[D","\0
33[C",
       │  0
  35   │ };
  39   │
  40   │ // screen
  41   │ static const char *screen_keys[] = {
  42   │        "\x1bOP","\x1bOQ","\x1bOR","\x1bOS","\x1b[15~","\x1b[17~","\x1b[18~","\x1b[19~","\x1b[20~","\x1b[21~","\x1b[23~","\x1b[24~","\x1b[2~","\x1b[3~","\x1b[1~","\x1b[4~","\x1b[5~","\x1b[6~","\x1bOA","\x1bOB","\x1bOD","\x1bOC", 0
  43   │ };
  47   │
  48   │ // rxvt-unicode
  49   │ static const char *rxvt_unicode_keys[] = {
  50   │        "\x1b[11~","\x1b[12~","\x1b[13~","\x1b[14~","\x1b[15~","\x1b[17~","\x1b[18~","\x1b[19~","\x1b[20~","\x1b[21~","\x1b[23~","\x1b[24~","\x1b[2~","\x1b[3~","\x1b[7~","\x1b[8~","\x1b[5~","\x1b[6~","\x1b[A","\x1b[B","\x1b[D","\x1b[C",
       │  0
  51   │ };
  55   │
  56   │ // linux
  57   │ static const char *linux_keys[] = {
  58   │        "\x1b[[A","\x1b[[B","\x1b[[C","\x1b[[D","\x1b[[E","\x1b[17~","\x1b[18~","\x1b[19~","\x1b[20~","\x1b[21~","\x1b[23~","\x1b[24~","\x1b[2~","\x1b[3~","\x1b[1~","\x1b[4~","\x1b[5~","\x1b[6~","\x1b[A","\x1b[B","\x1b[D","\x1b[C", 0
  59   │ };
  63   │
  64   │ // xterm
  65   │ static const char *xterm_keys[] = {
  66   │        "\x1bOP","\x1bOQ","\x1bOR","\x1bOS","\x1b[15~","\x1b[17~","\x1b[18~","\x1b[19~","\x1b[20~","\x1b[21~","\x1b[23~","\x1b[24~","\x1b[2~","\x1b[3~","\x1bOH","\x1bOF","\x1b[5~","\x1b[6~","\x1bOA","\x1bOB","\x1bOD","\x1bOC", 0
  67   │ };
:*/