use std::path;

use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::history::DirectoryHistory;
use crate::util::load_child::LoadChild;

pub fn new_directory(context: &mut JoshutoContext, p: &path::Path) -> JoshutoResult<()> {
    std::fs::create_dir_all(p)?;
    let options = context.config_ref().sort_option.clone();
    let curr_path = context.tab_context_ref().curr_tab_ref().pwd().to_path_buf();
    for tab in context.tab_context_mut().iter_mut() {
        tab.history_mut().reload(&curr_path, &options)?;
    }
    LoadChild::load_child(context)?;
    Ok(())
}
