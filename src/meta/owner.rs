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

#[cfg(windows)]
impl Owner {
    pub fn new(user: String, group: String) -> Self {
        Self {
            user: Some(user),
            group: Some(group),
        }
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
        let user = if let Some(u) = &self.user { u } else { "-".to_string() };
        colors.colorize(user.clone(), &Elem::User)
    }

    pub fn render_group(&self, colors: &Colors) -> ColoredString {
        let group = if let Some(u) = &self.group { u } else { "-".to_string() };
        colors.colorize(group.clone(), &Elem::Group)
    }
}
