use crate::color::{ColoredString, Colors};
use crate::flags::{Block, Display, Flags, Layout};
use crate::icon::Icons;
use crate::meta::{DisplayOption, FileType, Meta};
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::HashMap;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use unicode_width::UnicodeWidthStr;

const EDGE: &str = "\u{251c}\u{2500}\u{2500}"; // "├──"
const LINE: &str = "\u{2502}  "; // "├  "
const CORNER: &str = "\u{2514}\u{2500}\u{2500}"; // "└──"
const BLANK: &str = "   ";

pub fn grid(metas: &[Meta], flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    inner_display_grid(
        &DisplayOption::None,
        metas,
        &flags,
        colors,
        icons,
        0,
        termize::dimensions().map(|(w, _)| w as usize),
    )
}

pub fn tree(metas: &[Meta], flags: &Flags, colors: &Colors, icons: &Icons) -> String {
    inner_display_tree(metas, &flags, colors, icons, 0, "")
}

fn inner_display_grid(
    display_option: &DisplayOption,
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
    term_width: Option<usize>,
) -> String {
    let mut output = String::new();

    let padding_rules = get_padding_rules(&metas, flags);
    let mut grid = match flags.layout {
        Layout::OneLine => Grid::new(GridOptions {
            filling: Filling::Spaces(1),
            direction: Direction::LeftToRight,
        }),
        _ => Grid::new(GridOptions {
            filling: Filling::Spaces(2),
            direction: Direction::TopToBottom,
        }),
    };

    // The first iteration (depth == 0) corresponds to the inputs given by the
    // user. We defer displaying directories given by the user unless we've been
    // asked to display the directory itself (rather than its contents).
    let skip_dirs = (depth == 0) && (flags.display != Display::DirectoryOnly);

    // print the files first.
    for meta in metas {
        // Maybe skip showing the directory meta now; show its contents later.
        if skip_dirs {
            match meta.file_type {
                FileType::Directory { .. } => continue,
                FileType::SymLink { is_dir: true } if flags.layout != Layout::OneLine => continue,
                _ => {}
            }
        }

        let blocks = get_output(
            &meta,
            &colors,
            &icons,
            &flags,
            &display_option,
            &padding_rules,
        );

        for block in blocks {
            grid.add(Cell {
                width: get_visible_width(&block),
                contents: block.to_string(),
            });
        }
    }

    output += if flags.layout == Layout::Grid {
        match term_width.and_then(|tw| grid.fit_into_width(tw)) {
            Some(gridded_output) => gridded_output,
            None => grid.fit_into_columns(1),
        }
    } else {
        grid.fit_into_columns(flags.blocks.0.len())
    }
    .to_string()
    .as_str();

    let should_display_folder_path = should_display_folder_path(depth, &metas, &flags);

    // print the folder content
    for meta in metas {
        if meta.content.is_some() {
            if should_display_folder_path {
                output += &display_folder_path(&meta);
            }

            let display_option = DisplayOption::Relative {
                base_path: &meta.path,
            };

            output += &inner_display_grid(
                &display_option,
                meta.content.as_ref().unwrap(),
                &flags,
                colors,
                icons,
                depth + 1,
                term_width,
            );
        }
    }

    output
}

fn inner_display_tree(
    metas: &[Meta],
    flags: &Flags,
    colors: &Colors,
    icons: &Icons,
    depth: usize,
    prefix: &str,
) -> String {
    let padding_rules = get_padding_rules(&metas, flags);

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    for meta in metas.iter() {
        for block in get_output(
            &meta,
            &colors,
            &icons,
            &flags,
            &DisplayOption::FileName,
            &padding_rules,
        ) {
            let block_str = block.to_string();

            grid.add(Cell {
                width: get_visible_width(&block_str),
                contents: block_str,
            });
        }
    }

    let content = grid.fit_into_columns(flags.blocks.0.len()).to_string();
    let mut output = String::with_capacity(metas.len());

    for (idx, (meta, line)) in metas.iter().zip(content.lines()).enumerate() {
        let is_last_folder_elem = idx + 1 != metas.len();

        if depth > 0 {
            output += prefix;
            if is_last_folder_elem {
                output += EDGE;
            } else {
                output += CORNER;
            }
            output += " ";
        }

        output += line;
        output += "\n";

        if meta.content.is_some() {
            let mut new_prefix = prefix.to_string();
            if depth > 0 {
                if is_last_folder_elem {
                    new_prefix += LINE;
                } else {
                    new_prefix += BLANK;
                }
            }

            output += &inner_display_tree(
                &meta.content.as_ref().unwrap(),
                &flags,
                colors,
                icons,
                depth + 1,
                &new_prefix,
            );
        }
    }

    output
}

fn should_display_folder_path(depth: usize, metas: &[Meta], flags: &Flags) -> bool {
    if depth > 0 {
        true
    } else {
        let folder_number = metas
            .iter()
            .filter(|x| match x.file_type {
                FileType::Directory { .. } => true,
                FileType::SymLink { is_dir: true } => flags.layout != Layout::OneLine,
                _ => false,
            })
            .count();

        folder_number > 1 || folder_number < metas.len()
    }
}

fn display_folder_path(meta: &Meta) -> String {
    String::new() + "\n" + &meta.path.to_string_lossy() + ":\n"
}

fn get_output<'a>(
    meta: &'a Meta,
    colors: &'a Colors,
    icons: &'a Icons,
    flags: &'a Flags,
    display_option: &DisplayOption,
    padding_rules: &HashMap<Block, usize>,
) -> Vec<ANSIString<'a>> {
    let mut strings: Vec<ANSIString> = Vec::new();
    for block in flags.blocks.0.iter() {
        match block {
            Block::INode => strings.push(meta.inode.render(colors)),
            Block::Links => strings.push(meta.links.render(colors)),
            Block::Permission => {
                let s: &[ColoredString] = &[
                    meta.file_type.render(colors),
                    meta.permissions.render(colors),
                ];
                let res = ANSIStrings(s).to_string();
                strings.push(ColoredString::from(res));
            }
            Block::User => strings.push(meta.owner.render_user(colors)),
            Block::Group => strings.push(meta.owner.render_group(colors)),
            Block::Size => strings.push(meta.size.render(
                colors,
                &flags,
                padding_rules[&Block::SizeValue],
            )),
            Block::SizeValue => strings.push(meta.size.render_value(colors, flags)),
            Block::Date => strings.push(meta.date.render(colors, &flags)),
            Block::Name => {
                let s: String =
                    if flags.no_symlink.0 || flags.dereference.0 || flags.layout == Layout::Grid {
                        ANSIStrings(&[
                            meta.name.render(
                                colors,
                                icons,
                                &display_option,
                                &meta.metadata,
                                &flags.icons.separator.0,
                            ),
                            meta.indicator.render(&flags),
                        ])
                        .to_string()
                    } else {
                        ANSIStrings(&[
                            meta.name.render(
                                colors,
                                icons,
                                &display_option,
                                &meta.metadata,
                                &flags.icons.separator.0,
                            ),
                            meta.indicator.render(&flags),
                            meta.get_symlink().render(colors, &flags),
                        ])
                        .to_string()
                    };

                strings.push(ColoredString::from(s));
            }
        };
    }

    strings
}

fn get_visible_width(input: &str) -> usize {
    let mut nb_invisible_char = 0;

    // If the input has color, do not compute the length contributed by the color to the actual length
    for (idx, _) in input.match_indices("\u{1b}[") {
        let (_, s) = input.split_at(idx);

        let m_pos = s.find('m');
        if let Some(len) = m_pos {
            nb_invisible_char += len
        }
    }

    UnicodeWidthStr::width(input) - nb_invisible_char
}

fn detect_size_lengths(metas: &[Meta], flags: &Flags) -> usize {
    let mut max_value_length: usize = 0;

    for meta in metas {
        let value_len = meta.size.value_string(flags).len();

        if value_len > max_value_length {
            max_value_length = value_len;
        }
    }

    max_value_length
}

fn get_padding_rules(metas: &[Meta], flags: &Flags) -> HashMap<Block, usize> {
    let mut padding_rules: HashMap<Block, usize> = HashMap::new();

    if flags.blocks.0.contains(&Block::Size) {
        let size_val = detect_size_lengths(&metas, &flags);

        padding_rules.insert(Block::SizeValue, size_val);
    }

    padding_rules
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;
    use crate::color::Colors;
    use crate::icon;
    use crate::icon::Icons;
    use crate::meta::{FileType, Name};
    use tempfile::tempdir;

    const FILES: [&str; 8] = [
        "Ｈｅｌｌｏ,ｗｏｒｌｄ!",
        "ASCII1234-_",
        "File with space",
        "制作样本。",
        "日本語",
        "샘플은 무료로 드리겠습니다",
        "👩🐩",
        "🔬",
    ];

    macro_rules! get_files {
        ($tmp:expr) => {
            FILES
                .iter()
                .map(|&f| {
                    let path = { &$tmp }.path().join(f);
                    std::fs::File::create(&path).unwrap();
                    path
                })
                // expected lengths
                .zip(&[22, 11, 15, 10, 6, 26, 4, 2])
        };
    }

    #[test]
    fn test_display_get_visible_width_without_icons() {
        let tmp = tempdir().expect("failed to create temp dir");
        for (path, &len) in get_files!(tmp) {
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name.render(
                &Colors::new(color::Theme::NoColor),
                &Icons::new(icon::Theme::NoIcon),
                &DisplayOption::FileName,
                &path.metadata().unwrap(),
                " ",
            );

            assert_eq!(get_visible_width(&output), len);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_icons() {
        let tmp = tempdir().expect("failed to create temp dir");
        for (path, &len) in get_files!(tmp) {
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::Theme::NoColor),
                    &Icons::new(icon::Theme::Fancy),
                    &DisplayOption::FileName,
                    &path.metadata().unwrap(),
                    " ",
                )
                .to_string();

            // Add 2 characters for the icons.
            assert_eq!(get_visible_width(&output), len + 2);
        }
    }

    #[test]
    fn test_display_get_visible_width_with_colors() {
        let tmp = tempdir().expect("failed to create temp dir");
        for (path, &len) in get_files!(tmp) {
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::Theme::NoLscolors),
                    &Icons::new(icon::Theme::NoIcon),
                    &DisplayOption::FileName,
                    &path.metadata().unwrap(),
                    " ",
                )
                .to_string();

            // check if the color is present.
            assert_eq!(true, output.starts_with("\u{1b}[38;5;"));
            assert_eq!(true, output.ends_with("[0m"));
            assert_eq!(get_visible_width(&output), len);
        }
    }

    #[test]
    fn test_display_get_visible_width_without_colors() {
        let tmp = tempdir().expect("failed to create temp dir");
        for (path, &len) in get_files!(tmp) {
            let name = Name::new(
                &path,
                FileType::File {
                    exec: false,
                    uid: false,
                },
            );
            let output = name
                .render(
                    &Colors::new(color::Theme::NoColor),
                    &Icons::new(icon::Theme::NoIcon),
                    &DisplayOption::FileName,
                    &path.metadata().unwrap(),
                    " ",
                )
                .to_string();

            // check if the color is present.
            assert_eq!(false, output.starts_with("\u{1b}[38;5;"));
            assert_eq!(false, output.ends_with("[0m"));

            assert_eq!(get_visible_width(&output), len);
        }
    }
}
