use crate::check_some;
use crate::config::Config;
use crate::workspace::Workspace;

pub fn query<F>(f: F) -> Option<Vec<Workspace>>
where
    F: FnMut(&&Workspace) -> bool,
{
    let workspaces = super::workspace::get_workspaces();

    let workspaces: Vec<Workspace> = workspaces.iter().filter(f).map(|ws| ws.clone()).collect();

    if workspaces.len() > 0 {
        return Some(workspaces);
    }

    None
}

pub fn query_first<F>(f: F) -> Option<Workspace>
where
    F: FnMut(&&Workspace) -> bool,
{
    match query(f) {
        Some(workspaces) => Some(workspaces[0].clone()),
        None => None,
    }
}

pub fn query_focused() -> Workspace {
    check_some!(query_first(|ws| ws.focused), "Found no focused workspace")
}

pub fn query_index(main: &u32, sub: &u32) -> Option<Workspace> {
    query_first(|ws| &ws.main_index == main && &ws.sub_index == sub)
}

pub fn query_main(main: &u32) -> Option<Workspace> {
    query_first(|ws| &ws.main_index == main)
}

pub fn query_all_by_main(main: &u32) -> Option<Vec<Workspace>> {
    query(|ws| &ws.main_index == main)
}

pub fn query_swap(config: &Config) -> Option<Workspace> {
    query_first(|ws| &ws.prefix == &config.default_swap_prefix)
}

pub fn query_all_swaps(config: &Config) -> Option<Vec<Workspace>> {
    query(|ws| &ws.prefix == &config.default_swap_prefix)
}
