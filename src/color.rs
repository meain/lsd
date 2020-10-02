use crate::flags::color::ColorTheme;
use ansi_term::{ANSIString, Colour, Style};
use lscolors::{Indicator, LsColors};
use std::collections::HashMap;
use std::path::Path;

#[allow(dead_code)]
#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum Elem {
    /// Node type
    File {
        exec: bool,
        uid: bool,
    },
    SymLink,
    BrokenSymLink,
    Dir {
        uid: bool,
    },
    Pipe,
    BlockDevice,
    CharDevice,
    Socket,
    Special,

    /// Permissions
    Read,
    Write,
    Exec,
    ExecSticky,
    NoAccess,

    /// Last Time Modified
    DayOld,
    HourOld,
    Older,

    /// User / Group Name
    User,
    Group,

    /// File Size
    NonFile,
    FileLarge,
    FileMedium,
    FileSmall,

    /// INode
    INode {
        valid: bool,
    },
}

impl Elem {
    pub fn has_suid(&self) -> bool {
        matches!(self, Elem::Dir { uid: true } | Elem::File { uid: true, .. })
    }
}

pub type ColoredString<'a> = ANSIString<'a>;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum ColorMode {
    NoColor,
    Default,
    NoLscolors,
}

pub struct Colors {
    colors: Option<HashMap<Elem, Option<Colour>>>,
    lscolors: Option<LsColors>,
}

impl Colors {
    pub fn new(mode: ColorMode, theme: ColorTheme) -> Self {
        let colors = match mode {
            ColorMode::NoColor => None,
            ColorMode::Default | ColorMode::NoLscolors => Some(match theme {
                ColorTheme::Dark => Self::get_dark_theme_colour_map(),
                ColorTheme::Light => Self::get_light_theme_colour_map(),
                ColorTheme::Minimal => Self::get_minimal_theme_colour_map(),
            }),
        };
        let lscolors = match mode {
            ColorMode::NoColor => None,
            ColorMode::Default => Some(LsColors::from_env().unwrap_or_default()),
            ColorMode::NoLscolors => None,
        };

        Self { colors, lscolors }
    }

    pub fn colorize<'a>(&self, input: String, elem: &Elem) -> ColoredString<'a> {
        self.style(elem).paint(input)
    }

    pub fn colorize_using_path<'a>(
        &self,
        input: String,
        path: &Path,
        elem: &Elem,
    ) -> ColoredString<'a> {
        let style_from_path = self.style_from_path(path);
        match style_from_path {
            Some(style_from_path) => style_from_path.paint(input),
            None => self.colorize(input, elem),
        }
    }

    fn style_from_path(&self, path: &Path) -> Option<Style> {
        match &self.lscolors {
            Some(lscolors) => lscolors
                .style_for_path(path)
                .map(lscolors::Style::to_ansi_term_style),
            None => None,
        }
    }

    fn style(&self, elem: &Elem) -> Style {
        match &self.lscolors {
            Some(lscolors) => match self.get_indicator_from_elem(elem) {
                Some(style) => {
                    let style = lscolors.style_for_indicator(style);
                    style
                        .map(lscolors::Style::to_ansi_term_style)
                        .unwrap_or_default()
                }
                None => self.style_default(elem),
            },
            None => self.style_default(elem),
        }
    }

    fn style_default(&self, elem: &Elem) -> Style {
        if let Some(ref colors) = self.colors {
            let style_fg = if let Some(color) = colors[elem] {
                Style::default().fg(color)
            } else {
                Style::default()
            };
            if elem.has_suid() {
                style_fg.on(Colour::Fixed(124)) // Red3
            } else {
                style_fg
            }
        } else {
            Style::default()
        }
    }

    fn get_indicator_from_elem(&self, elem: &Elem) -> Option<Indicator> {
        let indicator_string = match elem {
            Elem::File { exec, uid } => match (exec, uid) {
                (_, true) => None,
                (true, false) => Some("ex"),
                (false, false) => Some("fi"),
            },
            Elem::Dir { uid } => {
                if *uid {
                    None
                } else {
                    Some("di")
                }
            }
            Elem::SymLink => Some("ln"),
            Elem::Pipe => Some("pi"),
            Elem::Socket => Some("so"),
            Elem::BlockDevice => Some("bd"),
            Elem::CharDevice => Some("cd"),
            Elem::BrokenSymLink => Some("or"),
            Elem::INode { valid } => match valid {
                true => Some("so"),
                false => Some("no"),
            },
            _ => None,
        };

        match indicator_string {
            Some(ids) => Indicator::from(ids),
            None => None,
        }
    }

    // You can find the table for each color, code, and display at:
    //
    //https://jonasjacek.github.io/colors/
    fn get_dark_theme_colour_map() -> HashMap<Elem, Option<Colour>> {
        let mut m = HashMap::new();
        // User / Group
        m.insert(Elem::User, Some(Colour::Fixed(230))); // Cornsilk1
        m.insert(Elem::Group, Some(Colour::Fixed(187))); // LightYellow3

        // Permissions
        m.insert(Elem::Read, Some(Colour::Green));
        m.insert(Elem::Write, Some(Colour::Yellow));
        m.insert(Elem::Exec, Some(Colour::Red));
        m.insert(Elem::ExecSticky, Some(Colour::Purple));
        m.insert(Elem::NoAccess, Some(Colour::Fixed(245))); // Grey

        // File Types
        m.insert(
            Elem::File {
                exec: false,
                uid: false,
            },
            Some(Colour::Fixed(184)),
        ); // Yellow3
        m.insert(
            Elem::File {
                exec: false,
                uid: true,
            },
            Some(Colour::Fixed(184)),
        ); // Yellow3
        m.insert(
            Elem::File {
                exec: true,
                uid: false,
            },
            Some(Colour::Fixed(40)),
        ); // Green3
        m.insert(
            Elem::File {
                exec: true,
                uid: true,
            },
            Some(Colour::Fixed(40)),
        ); // Green3
        m.insert(Elem::Dir { uid: true }, Some(Colour::Fixed(33))); // DodgerBlue1
        m.insert(Elem::Dir { uid: false }, Some(Colour::Fixed(33))); // DodgerBlue1
        m.insert(Elem::Pipe, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::SymLink, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::BrokenSymLink, Some(Colour::Fixed(124))); // Red3
        m.insert(Elem::BlockDevice, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::CharDevice, Some(Colour::Fixed(172))); // Orange3
        m.insert(Elem::Socket, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::Special, Some(Colour::Fixed(44))); // DarkTurquoise

        // Last Time Modified
        m.insert(Elem::HourOld, Some(Colour::Fixed(40))); // Green3
        m.insert(Elem::DayOld, Some(Colour::Fixed(42))); // SpringGreen2
        m.insert(Elem::Older, Some(Colour::Fixed(36))); // DarkCyan

        // Last Time Modified
        m.insert(Elem::NonFile, Some(Colour::Fixed(245))); // Grey
        m.insert(Elem::FileSmall, Some(Colour::Fixed(229))); // Wheat1
        m.insert(Elem::FileMedium, Some(Colour::Fixed(216))); // LightSalmon1
        m.insert(Elem::FileLarge, Some(Colour::Fixed(172))); // Orange3

        // INode
        m.insert(Elem::INode { valid: true }, Some(Colour::Fixed(13))); // Pink
        m.insert(Elem::INode { valid: false }, Some(Colour::Fixed(245))); // Grey

        m
    }

    fn get_light_theme_colour_map() -> HashMap<Elem, Option<Colour>> {
        let mut m = HashMap::new();
        // User / Group
        m.insert(Elem::User, Some(Colour::Fixed(111)));
        m.insert(Elem::Group, Some(Colour::Fixed(168)));

        // Permissions
        m.insert(Elem::Read, Some(Colour::Green));
        m.insert(Elem::Write, Some(Colour::Yellow));
        m.insert(Elem::Exec, Some(Colour::Red));
        m.insert(Elem::ExecSticky, Some(Colour::Purple));
        m.insert(Elem::NoAccess, Some(Colour::Fixed(245))); // Grey

        // File Types
        m.insert(
            Elem::File {
                exec: false,
                uid: false,
            },
            Some(Colour::Fixed(184)),
        ); // Yellow3
        m.insert(
            Elem::File {
                exec: false,
                uid: true,
            },
            Some(Colour::Fixed(184)),
        ); // Yellow3
        m.insert(
            Elem::File {
                exec: true,
                uid: false,
            },
            Some(Colour::Fixed(40)),
        ); // Green3
        m.insert(
            Elem::File {
                exec: true,
                uid: true,
            },
            Some(Colour::Fixed(40)),
        ); // Green3
        m.insert(Elem::Dir { uid: true }, Some(Colour::Fixed(33))); // DodgerBlue1
        m.insert(Elem::Dir { uid: false }, Some(Colour::Fixed(33))); // DodgerBlue1
        m.insert(Elem::Pipe, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::SymLink, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::BrokenSymLink, Some(Colour::Fixed(124))); // Red3
        m.insert(Elem::BlockDevice, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::CharDevice, Some(Colour::Fixed(172))); // Orange3
        m.insert(Elem::Socket, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::Special, Some(Colour::Fixed(44))); // DarkTurquoise

        // Last Time Modified
        m.insert(Elem::HourOld, Some(Colour::Fixed(40))); // Green3
        m.insert(Elem::DayOld, Some(Colour::Fixed(42))); // SpringGreen2
        m.insert(Elem::Older, Some(Colour::Fixed(36))); // DarkCyan

        // Last Time Modified
        m.insert(Elem::NonFile, Some(Colour::Fixed(245))); // Grey
        m.insert(Elem::FileSmall, Some(Colour::Fixed(180)));
        m.insert(Elem::FileMedium, Some(Colour::Fixed(137))); // LightSalmon1
        m.insert(Elem::FileLarge, Some(Colour::Fixed(130))); // Orange3

        // INode
        m.insert(Elem::INode { valid: true }, Some(Colour::Fixed(13))); // Pink
        m.insert(Elem::INode { valid: false }, Some(Colour::Fixed(245))); // Grey

        m
    }

    fn get_minimal_theme_colour_map() -> HashMap<Elem, Option<Colour>> {
        let mut m = HashMap::new();
        // User / Group
        m.insert(Elem::User, None);
        m.insert(Elem::Group, None);

        // Permissions
        m.insert(Elem::Read, None);
        m.insert(Elem::Write, None);
        m.insert(Elem::Exec, None);
        m.insert(Elem::ExecSticky, None);
        m.insert(Elem::NoAccess, Some(Colour::Fixed(8))); // Grey

        // File Types
        m.insert(
            Elem::File {
                exec: false,
                uid: false,
            },
            None,
        );
        m.insert(
            Elem::File {
                exec: false,
                uid: true,
            },
            Some(Colour::Fixed(8)),
        );
        m.insert(
            Elem::File {
                exec: true,
                uid: false,
            },
            Some(Colour::Fixed(8)),
        );
        m.insert(
            Elem::File {
                exec: true,
                uid: true,
            },
            Some(Colour::Fixed(8)),
        );
        m.insert(Elem::Dir { uid: true }, Some(Colour::Fixed(3))); // DodgerBlue1
        m.insert(Elem::Dir { uid: false }, Some(Colour::Fixed(26))); // DodgerBlue1
        m.insert(Elem::Pipe, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::SymLink, Some(Colour::Fixed(37)));
        m.insert(Elem::BrokenSymLink, Some(Colour::Fixed(124))); // Red3
        m.insert(Elem::BlockDevice, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::CharDevice, Some(Colour::Fixed(172))); // Orange3
        m.insert(Elem::Socket, Some(Colour::Fixed(44))); // DarkTurquoise
        m.insert(Elem::Special, Some(Colour::Fixed(44))); // DarkTurquoise

        // Last Time Modified
        m.insert(Elem::HourOld, None);
        m.insert(Elem::DayOld, None);
        m.insert(Elem::Older, Some(Colour::Fixed(245)));

        // Last Time Modified
        m.insert(Elem::NonFile, Some(Colour::Fixed(250))); // Grey
        m.insert(Elem::FileSmall, Some(Colour::Fixed(245)));
        m.insert(Elem::FileMedium, None);
        m.insert(Elem::FileLarge, None);

        // INode
        m.insert(Elem::INode { valid: true }, None);
        m.insert(Elem::INode { valid: false }, Some(Colour::Fixed(245))); // Grey

        m
    }
}
