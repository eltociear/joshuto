extern crate ncurses;

use std;
use std::fmt;

use joshuto;
use joshuto::command;
use joshuto::preview;
use joshuto::ui;

#[derive(Clone, Debug)]
pub struct ParentDirectory;

impl ParentDirectory {
    pub fn new() -> Self { ParentDirectory }
    pub fn command() -> &'static str { "parent_directory" }

    pub fn parent_directory(context: &mut joshuto::JoshutoContext)
    {
        if context.tabs[context.tab_index].curr_path.pop() == false {
            return;
        }

        match std::env::set_current_dir(&context.tabs[context.tab_index].curr_path) {
            Ok(_) => {
                {
                    let curr_tab = &mut context.tabs[context.tab_index];

                    let curr_list = curr_tab.curr_list.take();
                    curr_tab.history.put_back(curr_list);
                    let parent_list = curr_tab.parent_list.take();
                    curr_tab.curr_list = parent_list;

                    match curr_tab.curr_path.parent() {
                        Some(parent) => {
                            curr_tab.parent_list = match curr_tab.history.pop_or_create(&parent, &context.config_t.sort_type) {
                                Ok(s) => Some(s),
                                Err(e) => {
                                    ui::wprint_err(&context.views.left_win, e.to_string().as_str());
                                    None
                                },
                            };
                        },
                        None => {
                            ncurses::werase(context.views.left_win.win);
                            ncurses::wnoutrefresh(context.views.left_win.win);
                        },
                    }
                    curr_tab.refresh(&context.views, &context.theme_t, &context.config_t,
                        &context.username, &context.hostname);
                }
                preview::preview_file(context);
            },
            Err(e) => {
                ui::wprint_err(&context.views.bot_win, e.to_string().as_str());
            },
        };
        ncurses::doupdate();
    }
}

impl command::JoshutoCommand for ParentDirectory {}

impl std::fmt::Display for ParentDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl command::Runnable for ParentDirectory {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        Self::parent_directory(context);
    }
}
