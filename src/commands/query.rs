use crate::workspace::Workspace;

pub fn query<F>(f: F) -> Option<Vec<Workspace>>
where
    F: Fn(&&Workspace) -> bool,
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
    F: Fn(&&Workspace) -> bool,
{
    match query(f) {
        Some(workspaces) => Some(workspaces[0].clone()),
        None => None,
    }
}
