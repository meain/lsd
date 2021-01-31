use crate::color::{ColoredString, Colors, Elem};
#[cfg(unix)]
use std::fs::Metadata;

#[cfg(unix)]
#[derive(Clone, Debug)]
pub struct Owner {
    user: String,
    group: String,
}

#[cfg(windows)]
#[derive(Clone, Debug)]
pub struct Owner {
    user: Option<String>,
    group: Option<String>,
}

impl Owner {
    #[cfg_attr(unix, allow(dead_code))]
    pub fn new(user: String, group: String) -> Self {
        Self { user, group }
    }
}

#[cfg(unix)]
impl<'a> From<&'a Metadata> for Owner {
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;
        use users::{get_group_by_gid, get_user_by_uid};

        let user = match get_user_by_uid(meta.uid()) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => meta.uid().to_string(),
        };

        let group = match get_group_by_gid(meta.gid()) {
            Some(res) => res.name().to_string_lossy().to_string(),
            None => meta.gid().to_string(),
        };

        Self { user, group }
    }
}

#[cfg(unix)]
impl Owner {
    pub fn render_user(&self, colors: &Colors) -> ColoredString {
        colors.colorize(self.user.clone(), &Elem::User)
    }

    pub fn render_group(&self, colors: &Colors) -> ColoredString {
        colors.colorize(self.group.clone(), &Elem::Group)
    }
}

#[cfg(windows)]
impl Owner {
    pub fn render_user(&self, colors: &Colors) -> ColoredString {
        let user = if let Some(u) = self.user { u } else { "-" };
        colors.colorize(u.clone(), &Elem::User)
    }

    pub fn render_group(&self, colors: &Colors) -> ColoredString {
        let group = if let Some(u) = self.group { u } else { "-" };
        colors.colorize(g.clone(), &Elem::Group)
    }
}
