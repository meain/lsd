//! This module defines the [IgnoreVCS] flag. To set it up from [ArgMatches], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;

use crate::config_file::Config;

use clap::ArgMatches;

/// The flag showing whether to follow symbolic links.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct IgnoreVCS(pub bool);

impl Configurable<Self> for IgnoreVCS {
    /// Get a potential `NoSymlink` value from [ArgMatches].
    ///
    /// If the "ignore-vcs" argument is passed, this returns a `IgnoreVCS` with value `true` in a
    /// [Some]. Otherwise this returns [None].
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        if matches.is_present("ignore-vcs") {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Get a potential `IgnoreVCS` value from a [Config].
    ///
    /// If the `Config::ignore-vcs` has value,
    /// this returns it as the value of the `IgnoreVCS`, in a [Some].
    /// Otherwise this returns [None].
    fn from_config(config: &Config) -> Option<Self> {
        if let Some(no_link) = config.ignore_vcs {
            Some(Self(no_link))
        } else {
            None
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::NoSymlink;

//     use crate::app;
//     use crate::config_file::Config;
//     use crate::flags::Configurable;

//     #[test]
//     fn test_from_arg_matches_none() {
//         let argv = vec!["lsd"];
//         let matches = app::build().get_matches_from_safe(argv).unwrap();
//         assert_eq!(None, NoSymlink::from_arg_matches(&matches));
//     }

//     #[test]
//     fn test_from_arg_matches_true() {
//         let argv = vec!["lsd", "--no-symlink"];
//         let matches = app::build().get_matches_from_safe(argv).unwrap();
//         assert_eq!(Some(NoSymlink(true)), NoSymlink::from_arg_matches(&matches));
//     }

//     #[test]
//     fn test_from_config_none() {
//         assert_eq!(None, NoSymlink::from_config(&Config::with_none()));
//     }

//     #[test]
//     fn test_from_config_true() {
//         let mut c = Config::with_none();
//         c.no_symlink = Some(true);
//         assert_eq!(Some(NoSymlink(true)), NoSymlink::from_config(&c));
//     }

//     #[test]
//     fn test_from_config_false() {
//         let mut c = Config::with_none();
//         c.no_symlink = Some(false);
//         assert_eq!(Some(NoSymlink(false)), NoSymlink::from_config(&c));
//     }
// }
