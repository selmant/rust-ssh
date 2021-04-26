use crate::commands::Commands::{
    self, Cd, Cp, Ls, Mkdir, Mv, Popd, Pushd, Pwd, Rm, Rmdir, Touch, UnknowCommand,
};
use crate::commands::IO_COMMAND_ARRAY;
use std::{fs::{self, DirEntry}, io::Error, path::{Path, PathBuf}};
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
            Cp { .. } => self.cp(command),
            Mv { .. } => self.mv(command),
            Touch { .. } => self.touch(command),
            Pwd => Ok(self.pwd(command)),
            Pushd { .. } => Ok(self.pushd(command)),
            Popd => self.popd(command),
            UnknowCommand { .. } => self.unkown_command(command),
        }
    }

    pub(crate) fn get_wd(&self) -> &PathBuf {
        &self.working_directory
    }

    fn cp(&self, command: Commands) -> std::io::Result<Option<String>> {
        if let Cp {
            source,
            destination,
            recursive: _,
            symlink,
        } = command
        {
            let source_path = self.working_directory.join(source);
            let destination_path = self.working_directory.join(destination);
            if symlink {
                std::os::unix::fs::symlink(source_path, destination_path)?;
            } else {
                fs::copy(source_path, destination_path)?;
            }
        }
        Ok(None)
    }
    fn mv(&self, command: Commands) -> std::io::Result<Option<String>> {
        if let Mv {
            source,
            destination,
        } = command
        {
            let source_path = self.working_directory.join(source);
            let destination_path = self.working_directory.join(destination);
            match source_path.metadata().unwrap().permissions().readonly() {
                true => fs::remove_file(source_path)?,
                false => {
                    fs::copy(&source_path, destination_path)?;
                    fs::remove_file(source_path)?;
                }
            }
        }
        Ok(None)
    }
    fn touch(&self, command: Commands) -> std::io::Result<Option<String>> {
        if let Touch { file } = command {
            let path = self.working_directory.join(PathBuf::from(file));
            if path.is_file() {
                fs::File::open(path)?;
            } else {
                fs::File::create(path)?;
            }
        }
        Ok(None)
    }
    fn unkown_command(&self, command: Commands) -> std::io::Result<Option<String>> {
        let mut error_message = None;
        if let UnknowCommand { command } = command {
            if IO_COMMAND_ARRAY.to_vec().iter().any(|x| x.0 == command) {
                error_message = Some("Missing operands.")
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            error_message.unwrap_or("Command not found."),
        ))
    }
    fn rmdir(&self, command: Commands) -> std::io::Result<Option<String>> {
        if let Rmdir { path, parent } = command {
            let path = self.working_directory.join(path);
            match parent {
                false => fs::remove_dir(path)?,
                true => fs::remove_dir_all(path)?,
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
            let rm_path = self.working_directory.join(path);
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
            let path = self.working_directory.join(path);
            match parent {
                true => fs::create_dir_all(path)?,
                false => fs::create_dir(path)?,
            };
        }
        Ok(None)
    }

    fn pushd(&mut self, command: Commands) -> Option<String> {
        if let Pushd { path } = command {
            self.directory_stack.push(path.to_string());
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
                let last = self.directory_stack.pop();
                self.working_directory = PathBuf::from(last.clone().unwrap());
                return Ok(last);
            }
        }
        Ok(None)
    }
    fn cd(&mut self, command: Commands) -> std::io::Result<Option<String>> {
        if let Cd { path } = command {
            let mut new_dir = self.working_directory.clone();
            new_dir.push(path);
            new_dir.read_dir()?;
            self.working_directory = new_dir.canonicalize()?;

            return Ok(self.pwd(Pwd));
        }
        Ok(None)
    }
    fn ls(&self, command: Commands) -> std::io::Result<Option<String>> {
        if let Ls {
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
                almost_all,
            )?;
            println!("{:#?}", folder_vec);

            let display_entry = |entry: DirEntry| match list {
                false => format!("{} ", entry.file_name().to_str().unwrap()),
                true => {
                    let metadata = entry.metadata().unwrap();
                    let datetime: chrono::DateTime<chrono::offset::Local> =
                        metadata.created().unwrap().into();
                    format!(
                        "\n{:>8} {:20} {}",
                        metadata.len(),
                        datetime.format("%d/%m/%Y %T"),
                        entry.file_name().to_str().unwrap()
                    )
                }
            };

            let mut output = String::new();
            let entry_count: usize = folder_vec.iter().map(|inner| inner.len()).sum();
            output.reserve(if list {
                entry_count * 45
            } else {
                entry_count * 10
            });

            for mut dir_vec in folder_vec {
                if reverse {
                    dir_vec.reverse();
                }
                for entry in dir_vec {
                    output.push_str(display_entry(entry).as_str());
                }
                output.push_str("\n\n");
            }
            return Ok(Some(output));
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
        almost_all: bool,
    ) -> Result<String, std::io::Error> {
        let mut dir_vec = Vec::new();
        let hidden_filter = |wrapped_entry: &Result<DirEntry, Error>| {
            if !almost_all {
                if let Ok(entry) = wrapped_entry {
                    return !entry.file_name().to_str().unwrap().starts_with('.');
                }
            }
            false
        };
        for entry in path.read_dir()?.filter(hidden_filter) {
            let entry_unwrapped = entry.unwrap();
            if recursive && entry_unwrapped.metadata().as_ref().unwrap().is_dir() {
                Self::perform_ls(
                    folder_vec,
                    entry_unwrapped.path().as_path(),
                    recursive,
                    almost_all,
                )?;
            }
            dir_vec.push(entry_unwrapped);
        }
        folder_vec.push(dir_vec);
        Ok("".to_string())
    }
}
