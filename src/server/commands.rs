use std::fmt::{self, write, Display};

pub(crate) enum Commands<'a> {
    Mkdir {
        path: &'a str,
        parent: bool,
    },
    Rm {
        path: &'a str,
        recursive: bool,
        directory: bool,
    },
    Rmdir {
        path: &'a str,
        parent: bool,
    },
    Ls {
        all: bool,
        almost_all: bool,
        list: bool,
        reverse: bool,
        recursive: bool,
    },
    Cp {
        source: &'a str,
        destination: &'a str,
        recursive: bool,
        symlink: bool,
    },
    Mv {
        source: &'a str,
        destination: &'a str,
    },
    Touch {
        file: &'a str,
    },
    Pwd,
    Pushd,
    Popd,
    UnknowCommand,
}

impl<'a> Commands<'a> {
    fn as_string(&self) -> String {
        match self {
            Commands::Mkdir { .. } => "mkdir".to_string(),
            Commands::Rm { .. } => "rm".to_string(),
            Commands::Rmdir { .. } => "rmdir".to_string(),
            Commands::Ls { .. } => "ls".to_string(),
            Commands::Cp { .. } => "cp".to_string(),
            Commands::Mv { .. } => "mv".to_string(),
            Commands::Touch { .. } => "touch".to_string(),
            Commands::Pwd => "pwd".to_string(),
            Commands::Pushd => "pushd".to_string(),
            Commands::Popd => "popd".to_string(),
            Commands::UnknowCommand => "unknown".to_string(),
        }
    }
    pub(crate) fn new(command: &'a str) -> Commands<'a> {
        let mut split = command.split(' ');

        match split.next() {
            Some("mkdir") => Commands::generate_mkdir(split),
            Some("rm") => Commands::generate_rm(split),
            Some("rmdir") => Commands::generate_rmdir(split),
            Some("ls") => Commands::generate_ls(split),
            Some("cp") => Commands::generate_cp(split),
            Some("mv") => Commands::generate_mv(split),
            Some("touch") => Commands::generate_touch(split),
            Some("pwd") => Commands ::Pwd,
            Some("pushd") => Commands::Pushd,
            Some("popd") => Commands::Popd,
            Some(_) => Commands::UnknowCommand,
            None => Commands::UnknowCommand,
        }
    }

    fn generate_cp(mut split: std::str::Split<char>) -> Commands {
        //let recursive= split.filter(|&word| word == "--recursive");
        let words : Vec<&str> = split.collect();
        let source_and_des = words.iter().filter(|&&word| !word.starts_with('-'));
        let double_dash_options =words.iter().filter(|&&word| word.starts_with("--"));
        let single_dash_options=words.iter().filter(|&&word| word.starts_with('-') && word.starts_with("--"));
        Commands::Cp {
            source: "val",
            destination: "val",
            recursive: true,
            symlink: true,
        }
    }
    fn generate_rm(mut split: std::str::Split<char>) -> Commands {
        Commands::Cp {
            source: "val",
            destination: "val",
            recursive: true,
            symlink: true,
        }
    }
    fn generate_rmdir(mut split: std::str::Split<char>) -> Commands {
        Commands::Cp {
            source: "val",
            destination: "val",
            recursive: true,
            symlink: true,
        }
    }
    fn generate_ls(mut split: std::str::Split<char>) -> Commands {
        Commands::Cp {
            source: "val",
            destination: "val",
            recursive: true,
            symlink: true,
        }
    }
    fn generate_touch(mut split: std::str::Split<char>) -> Commands {
        Commands::Cp {
            source: "val",
            destination: "val",
            recursive: true,
            symlink: true,
        }
    }
    fn generate_mv(mut split: std::str::Split<char>) -> Commands {
        Commands::Cp {
            source: "val",
            destination: "val",
            recursive: true,
            symlink: true,
        }
    }
    fn generate_mkdir(mut split: std::str::Split<char>) -> Commands {
        Commands::Cp {
            source: "val",
            destination: "val",
            recursive: true,
            symlink: true,
        }
    }

    fn extract_single_dash_option() -> bool{
        false
    }
}
