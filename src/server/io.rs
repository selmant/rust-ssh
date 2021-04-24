use crate::commands::Commands::{
    self, Cd, Cp, Ls, Mkdir, Mv, Popd, Pushd, Pwd, Rm, Rmdir, Touch, UnknowCommand,
};
use std::{
    fs::{self, remove_dir, remove_dir_all, DirEntry},
    io::Error,
    path::{Path, PathBuf},
};
#[allow(clippy::clippy::upper_case_acronyms)]
pub(crate) struct IOOperationHandler {
    working_directory: PathBuf,
    directory_stack: Vec<String>,
}

impl IOOperationHandler {
    pub(crate) fn new(working_directory: PathBuf) -> IOOperationHandler {
        let directory_stack: Vec<String> = Vec::new();
        IOOperationHandler {
            working_directory,
            directory_stack,
        }
    }
    pub(crate) fn perform_operation(
        &mut self,
        command: Commands,
    ) -> std::io::Result<Option<String>> {
        match command {
            Cd { .. } => self.cd(command),
            Ls { .. } => self.ls(command),
            Mkdir { .. } => self.mkdir(command),
            Rm { .. } => self.rm(command),
            Rmdir { .. } => self.rmdir(command),
            Cp { .. } => Ok(None),
            Mv { .. } => Ok(None),
            Touch { .. } => Ok(None),
            Pwd => Ok(self.pwd(command)),
            Pushd { .. } => Ok(self.pushd(command)),
            Popd => self.popd(command),
            UnknowCommand => Ok(None),
        }
    }
    fn rmdir(&self, command: Commands) -> std::io::Result<Option<String>> {
        if let Rmdir { path, parent } = command {
            let path = PathBuf::from(path);
            let mut remove_leaf_path = self.working_directory.clone();
            remove_leaf_path.push(path);
            match parent {
                false => remove_dir(remove_leaf_path)?,
                true => remove_dir_all(remove_leaf_path)?,
            }
        };
        Ok(None)
    }

    fn rm(&self, command: Commands) -> std::io::Result<Option<String>> {
        if let Rm {
            path,
            recursive,
            directory,
        } = command
        {
            let mut rm_path = self.working_directory.clone();
            rm_path.push(path);
            match directory {
                true => {
                    return self.rmdir(Commands::Rmdir {
                        path,
                        parent: recursive,
                    })
                }
                false => match recursive {
                    false => fs::remove_file(rm_path)?,
                    true => {
                        rm_path.read_dir()?;
                        Self::rm_all_dir(rm_path)?;
                    }
                },
            }
        }
        Ok(None)
    }
    fn rm_all_dir(path: PathBuf) -> std::io::Result<()> {
        for entry in path.read_dir()? {
            let entry_unwrapped = entry.unwrap();
            if entry_unwrapped.metadata().unwrap().is_dir() {
                Self::rm_all_dir(entry_unwrapped.path())?;
            } else {
                fs::remove_file(entry_unwrapped.path())?;
            }
        }
        fs::remove_dir(path)?;
        Ok(())
    }
    fn mkdir(&self, command: Commands) -> std::io::Result<Option<String>> {
        if let Mkdir { path, parent } = command {
            let mut dir_path = self.working_directory.clone();
            dir_path.push(path);
            match parent {
                true => fs::create_dir_all(dir_path)?,
                false => fs::create_dir(dir_path)?,
            };
        }
        Ok(None)
    }

    fn pushd(&mut self, command: Commands) -> Option<String> {
        if let Pushd { path } = command {
            self.directory_stack.push(path.to_string());

            return Some(crate::session::DEFAULT_PATH.to_string());
        }
        None
    }
    fn popd(&mut self, command: Commands) -> std::io::Result<Option<String>> {
        if let Popd = command {
            if self.directory_stack.is_empty() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Directory stack is empty.",
                ));
            } else {
                return Ok(self.directory_stack.pop());
            }
        }
        Ok(None)
    }
    fn cd(&mut self, command: Commands) -> std::io::Result<Option<String>> {
        if let Cd { path } = command {
            self.working_directory.push(path);
            self.working_directory.read_dir()?;

            self.working_directory = self.working_directory.canonicalize()?;

            return Ok(self.pwd(Pwd));
        }
        Ok(None)
    }
    fn ls(&self, command: Commands) -> std::io::Result<Option<String>> {
        if let Ls {
            all,
            almost_all,
            list,
            recursive,
            reverse,
        } = command
        {
            let mut folder_vec = Vec::new();
            Self::perform_ls(
                &mut folder_vec,
                self.working_directory.as_path(),
                recursive,
                all || almost_all,
            )?;
            println!("{:#?}", folder_vec)
        }

        Ok(None)
    }

    fn pwd(&self, command: Commands) -> Option<String> {
        if let Pwd = command {
            return Some(self.working_directory.to_string_lossy().to_string());
        }
        None
    }

    fn perform_ls(
        folder_vec: &mut Vec<Vec<DirEntry>>,
        path: &Path,
        recursive: bool,
        show_hidden: bool,
    ) -> Result<String, std::io::Error> {
        let mut dir_vec = Vec::new();
        let hidden_filter = |wrapped_entry: &Result<DirEntry, Error>| {
            if show_hidden {
                if let Ok(entry) = wrapped_entry {
                    return !entry.file_name().to_str().unwrap().starts_with('.');
                }
            }
            true
        };

        for entry in path.read_dir()?.filter(hidden_filter) {
            let entry_unwrapped = entry.unwrap();

            if recursive && entry_unwrapped.metadata().as_ref().unwrap().is_dir() {
                Self::perform_ls(
                    folder_vec,
                    entry_unwrapped.path().as_path(),
                    recursive,
                    show_hidden,
                )?;
            }
            dir_vec.push(entry_unwrapped);
        }
        folder_vec.push(dir_vec);
        Ok("".to_string())
    }
}
