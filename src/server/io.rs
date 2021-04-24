use crate::commands::Commands;
use std::{
    fs::DirEntry,
    io::Error,
    path::{Path, PathBuf},
};

pub(crate) struct IOOperationHandler {
    working_directory: PathBuf,
    directory_stack: Vec<PathBuf>,
}

impl IOOperationHandler {
    pub(crate) fn new(working_directory: PathBuf) -> IOOperationHandler {
        let directory_stack: Vec<PathBuf> = Vec::new();
        IOOperationHandler {
            working_directory,
            directory_stack,
        }
    }
    pub(crate) fn perform_operation(
        &mut self,
        command: Commands,
    ) -> Result<Option<String>, std::io::Error> {
        match command {
            Commands::Cd { .. } => self.cd(command),
            Commands::Ls { .. } => self.ls(command),
            Commands::Mkdir { .. } => Ok(None),
            Commands::Rm { .. } => Ok(None),
            Commands::Rmdir { .. } => Ok(None),
            Commands::Cp { .. } => Ok(None),
            Commands::Mv { .. } => Ok(None),
            Commands::Touch { .. } => Ok(None),
            Commands::Pwd => Ok(self.pwd(command)),
            Commands::Pushd => Ok(None),
            Commands::Popd => Ok(None),
            Commands::UnknowCommand => Ok(None),
        }
    }

    fn pushd(&mut self, command: Commands) -> Option<String> {
        None
    
    }
    fn cd(&mut self, command: Commands) -> Result<Option<String>, std::io::Error> {
        if let Commands::Cd { path } = command {
            self.working_directory.push(path);
            self.working_directory.read_dir()?;

            self.working_directory = self.working_directory.canonicalize()?;

            return Ok(self.pwd(Commands::Pwd));
        }
        Ok(None)
    }
    fn ls(&self, command: Commands) -> Result<Option<String>, std::io::Error> {
        if let Commands::Ls {
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
        if let Commands::Pwd = command {
            return Some(self.working_directory.to_string_lossy().to_string())
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
