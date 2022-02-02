use std::io::{self, Stdout};
use std::path::{self, PathBuf};

use exitfailure::ExitFailure;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::file_ops::{self, DirectoryItem};

pub struct App<'a> {
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    pub selection_index: Option<usize>,
    pub current_directory: path::PathBuf,
    pub directory_contents: Vec<DirectoryItem>,
    pub search_buffer: Vec<char>,
    pub error: Option<String>,
    pub window_height: u16,

    max_file_selection: usize,
}

impl<'a> App<'a> {
    pub fn new(
        terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
        music_database: &str,
    ) -> Result<App<'a>, ExitFailure> {
        let window_height = terminal.size().unwrap().height - 5;
        let current_directory = path::PathBuf::from(music_database);

        let mut app = App {
            terminal,
            selection_index: None,
            current_directory,
            directory_contents: Vec::new(),
            search_buffer: Vec::new(),
            error: None,
            window_height,
            max_file_selection: 0,
        };

        app.populate_files()?;

        Ok(app)
    }

    pub fn update_window_height(&mut self) {
        //borders window height add up to 2
        self.window_height = self.terminal.size().unwrap().height - 5;
    }

    pub fn populate_files(&mut self) -> Result<(), io::Error> {
        let mut dir_items = file_ops::get_files_for_current_directory(self)?;
        // Sort: folder > file
        dir_items.sort_by(|a, b| b.cmp(a));

        self.directory_contents = dir_items;
        self.max_file_selection = self.directory_contents.len();

        if self.max_file_selection == 0 {
            self.selection_index = None;
        } else {
            self.selection_index = Some(0);
        }

        Ok(())
    }

    pub fn get_search_string(&mut self) -> String {
        let mut search_string = String::new();
        for c in &self.search_buffer {
            search_string.push(*c);
        }

        search_string
    }

    pub fn move_select_top(&mut self) {
        if let Some(_) = self.selection_index {
            self.selection_index = Some(0);
        }
    }

    pub fn move_select_bottom(&mut self) {
        if let Some(_) = self.selection_index {
            self.selection_index = Some(self.max_file_selection - 1);
        }
    }

    pub fn move_select_up(&mut self, step: i32) {
        if let Some(selection_index) = self.selection_index {
            let diff = selection_index as i32 - step;
            if diff > 0 {
                self.selection_index = Some(diff as usize);
            } else {
                self.selection_index = Some(0);
            }
        }
    }

    pub fn move_select_down(&mut self, step: usize) {
        if let Some(selection_index) = self.selection_index {
            if selection_index + step < self.max_file_selection {
                self.selection_index = Some(selection_index + step);
            } else {
                self.selection_index = Some(self.max_file_selection - 1);
            }
        }
    }

    pub fn next_page(&mut self) {
        if let Some(selection_index) = self.selection_index {
            let height = self.window_height;
            // show items length
            let display_pages =
                ((self.directory_contents.len() as f32) / (height as f32)).ceil() as usize;

            if display_pages > 1 {
                let mut page = 0;
                for i in 0..display_pages {
                    if selection_index < (i + 1) * height as usize {
                        page = i + 1;
                        break;
                    }
                }

                if page < display_pages {
                    self.selection_index = Some(page * height as usize);
                }
            }
        }
    }

    pub fn previous_page(&mut self) {
        if let Some(selection_index) = self.selection_index {
            let height = self.window_height;
            // show items length
            let display_pages =
                ((self.directory_contents.len() as f32) / (height as f32)).ceil() as usize;

            if display_pages > 1 {
                let mut page = 0;
                for i in 0..display_pages {
                    if selection_index < (i + 1) * height as usize {
                        page = i + 1;
                        break;
                    }
                }

                if page > 1 {
                    self.selection_index = Some(selection_index - height as usize);
                }
            }
        }
    }

    pub fn open_folder(&mut self) {
        if let Some(selection_index) = self.selection_index {
            match &self.directory_contents[selection_index] {
                DirectoryItem::File(_) => {}
                DirectoryItem::Directory(path) => {
                    let previous_dir = self.current_directory.clone();
                    self.current_directory.push(path);
                    if let Err(err) = self.populate_files() {
                        self.current_directory = previous_dir;
                        self.error = Some(err.to_string());
                    } else {
                        if self.max_file_selection == 0 {
                            self.selection_index = None;
                        } else {
                            self.selection_index = Some(0);
                        }
                    };
                }
            };
        }
    }

    pub fn back_previous_folder(&mut self, music_database: &str) {
        let music_database = PathBuf::from(music_database);
        let current_directory = self.current_directory.clone();

        if !current_directory.eq(&music_database) {
            let current_dir_str = current_directory.display().to_string();
            let current_dir_split: Vec<&str> = current_dir_str.split_inclusive("\\").collect();

            if current_dir_split.len() > 1 {
                let mut previous = String::new();
                for i in 0..current_dir_split.len() - 1 {
                    previous = previous + current_dir_split[i];
                }
                self.current_directory = PathBuf::from(previous);

                if let Err(err) = self.populate_files() {
                    self.current_directory = current_directory;
                    self.error = Some(err.to_string());
                } else {
                    if self.max_file_selection == 0 {
                        self.selection_index = None;
                    } else {
                        self.selection_index = Some(0);
                    }
                };
            }
        }
    }
}
