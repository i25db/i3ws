use crate::workspace::Workspace;

pub struct Query<'a> {
    pub prefix: Option<&'a str>,
    pub main_index: Option<&'a str>,
    pub sub_index: Option<&'a str>,
    pub suffix: Option<&'a str>,
    pub name: Option<&'a str>,
    pub focused: Option<bool>,
}

impl<'a> Default for Query<'a> {
    fn default() -> Self {
        Self {
            prefix: None,
            main_index: None,
            sub_index: None,
            suffix: None,
            name: None,
            focused: None,
        }
    }
}

pub fn query(query: &Query) -> Option<Vec<Workspace>> {
    let workspaces = super::workspace::get_workspaces();

    let workspaces: Vec<Workspace> = workspaces
        .iter()
        .filter(|ws| {
            (query.prefix.is_none() || query.prefix.unwrap() == ws.prefix)
                && (query.suffix.is_none() || query.suffix.unwrap() == ws.suffix)
                && (query.main_index.is_none() || query.main_index.unwrap() == ws.main_index)
                && (query.sub_index.is_none() || query.sub_index.unwrap() == ws.sub_index)
                && (query.name.is_none() || query.name.unwrap() == ws.get_name())
                && (query.focused.is_none() || query.focused.unwrap() == ws.focused)
        })
        .map(|ws| ws.clone())
        .collect();

    if workspaces.len() > 0 {
        return Some(workspaces);
    }

    None
}

pub fn query_first(query: &Query) -> Option<Workspace> {
    match self::query(query) {
        Some(workspaces) => Some(workspaces[0].clone()),
        None => None,
    }
}
