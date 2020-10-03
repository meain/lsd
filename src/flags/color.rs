//! This module defines the [Color]. To set it up from [ArgMatches], a [Yaml] and its [Default]
//! value, use its [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;
use yaml_rust::Yaml;

/// A collection of flags on how to use colors.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Color {
    /// When to use color.
    pub when: ColorOption,
    pub theme: ColorTheme,
}

impl Color {
    /// Get a `Color` struct from [ArgMatches], a [Config] or the [Default] values.
    ///
    /// The [ColorOption] is configured with their respective [Configurable] implementation.
    pub fn configure_from(matches: &ArgMatches, config: &Config) -> Self {
        let when = ColorOption::configure_from(matches, config);
        let theme = ColorTheme::configure_from(matches, config);
        Self { when, theme }
    }
}

/// The flag showing when to use colors in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ColorOption {
    Always,
    Auto,
    Never,
}

impl ColorOption {
    /// Get a Color value from a [Yaml] string. The [Config] is used to log warnings about wrong
    /// values in a Yaml.
    fn from_yaml_string(value: &str, config: &Config) -> Option<Self> {
        match value {
            "always" => Some(Self::Always),
            "auto" => Some(Self::Auto),
            "never" => Some(Self::Never),
            _ => {
                config.print_invalid_value_warning("color->when", &value);
                None
            }
        }
    }
}

impl Configurable<Self> for ColorOption {
    /// Get a potential `ColorOption` variant from [ArgMatches].
    ///
    /// If the "classic" argument is passed, then this returns the [ColorOption::Never] variant in
    /// a [Some]. Otherwise if the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("classic") {
            Some(Self::Never)
        } else if matches.occurrences_of("color") > 0 {
            match matches.value_of("color") {
                Some("always") => Some(Self::Always),
                Some("auto") => Some(Self::Auto),
                Some("never") => Some(Self::Never),
                _ => panic!("This should not be reachable!"),
            }
        } else {
            None
        }
    }

    /// Get a potential `ColorOption` variant from a [Config].
    ///
    /// If the Config's [Yaml] contains a [Boolean](Yaml::Boolean) value pointed to by "classic"
    /// and its value is `true`, then this returns the [ColorOption::Never] variant in a [Some].
    /// Otherwise if the Yaml contains a [String](Yaml::String) value pointed to by "color" ->
    /// "when" and it is one of "always", "auto" or "never", this returns its corresponding variant
    /// in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            if let Yaml::Boolean(true) = &yaml["classic"] {
                Some(Self::Never)
            } else {
                match &yaml["color"]["when"] {
                    Yaml::BadValue => None,
                    Yaml::String(value) => Self::from_yaml_string(&value, &config),
                    _ => {
                        config.print_wrong_type_warning("color->when", "string");
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `ColorOption` is [ColorOption::Auto].
impl Default for ColorOption {
    fn default() -> Self {
        Self::Auto
    }
}

/// The flag showing when to use colors in the output.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ColorTheme {
    Light,
    Dark,
    Minimal,
}

impl ColorTheme {
    /// Get a Color value from a [Yaml] string. The [Config] is used to log warnings about wrong
    /// values in a Yaml.
    fn from_yaml_string(value: &str, config: &Config) -> Option<Self> {
        match value {
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            "minimal" => Some(Self::Minimal),
            _ => {
                config.print_invalid_value_warning("color->theme", &value);
                None
            }
        }
    }
}

impl Configurable<Self> for ColorTheme {
    /// Get a potential `ColorTheme` variant from [ArgMatches].
    ///
    /// If the argument is passed, this returns the variant corresponding to
    /// its parameter in a [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.occurrences_of("color-theme") > 0 {
            match matches.value_of("color-theme") {
                Some("light") => Some(Self::Light),
                Some("dark") => Some(Self::Dark),
                Some("minimal") => Some(Self::Minimal),
                _ => Some(Self::Dark)
            }
        } else {
            None
        }
    }

    /// Get a potential `ColorTheme` variant from a [Config].
    ///
    /// If the Yaml contains a [String](Yaml::String) value pointed to by "color" ->
    /// "theme" and it is one of "light", "dark" or "minimal", this returns its corresponding variant
    /// in a [Some]. Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(yaml) = &config.yaml {
            match &yaml["color"]["theme"] {
                Yaml::BadValue => None,
                Yaml::String(value) => Self::from_yaml_string(&value, &config),
                _ => {
                    config.print_wrong_type_warning("color->theme", "string");
                    None
                }
            }
        } else {
            None
        }
    }
}

/// The default value for `ColorOption` is [ColorOption::Auto].
impl Default for ColorTheme {
    fn default() -> Self {
        Self::Dark
    }
}

#[cfg(test)]
mod test_color_option {
    use super::ColorOption;

    use crate::app;
    use crate::config_file::Config;
    use crate::flags::Configurable;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_from_arg_matches_none() {
        let argv = vec!["lsd"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(None, ColorOption::from_arg_matches(&matches));
    }

    #[test]
    fn test_from_arg_matches_always() {
        let argv = vec!["lsd", "--color", "always"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Always),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_autp() {
        let argv = vec!["lsd", "--color", "auto"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Auto),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_never() {
        let argv = vec!["lsd", "--color", "never"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Never),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_arg_matches_classic_mode() {
        let argv = vec!["lsd", "--color", "always", "--classic"];
        let matches = app::build().get_matches_from_safe(argv).unwrap();
        assert_eq!(
            Some(ColorOption::Never),
            ColorOption::from_arg_matches(&matches)
        );
    }

    #[test]
    fn test_from_config_none() {
        assert_eq!(None, ColorOption::from_config(&Config::with_none()));
    }

    #[test]
    fn test_from_config_empty() {
        let yaml_string = "---";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(None, ColorOption::from_config(&Config::with_yaml(yaml)));
    }

    #[test]
    fn test_from_config_always() {
        let yaml_string = "color:\n  when: always";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(ColorOption::Always),
            ColorOption::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_auto() {
        let yaml_string = "color:\n  when: auto";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(ColorOption::Auto),
            ColorOption::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_never() {
        let yaml_string = "color:\n  when: never";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(ColorOption::Never),
            ColorOption::from_config(&Config::with_yaml(yaml))
        );
    }

    #[test]
    fn test_from_config_classic_mode() {
        let yaml_string = "classic: true\ncolor:\n  when: always";
        let yaml = YamlLoader::load_from_str(yaml_string).unwrap()[0].clone();
        assert_eq!(
            Some(ColorOption::Never),
            ColorOption::from_config(&Config::with_yaml(yaml))
        );
    }
}
