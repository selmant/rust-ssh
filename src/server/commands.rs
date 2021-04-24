#[derive(Debug, Clone)]
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
    Cd {
        path: &'a str,
    },
    Pwd,
    Pushd {
        path: &'a str,
    },
    Popd,
    UnknowCommand {
        command: &'a str,
    },
}

struct Opts<'a> {
    command: &'a str,
    single_dash_options: Vec<&'a str>,
    double_dash_options: Vec<&'a str>,
    non_options: Vec<&'a str>,
}

impl<'a> Opts<'a> {
    fn new(mut words: Vec<&'a str>) -> Opts<'a> {
        let command = words.remove(0);
        let non_options = words
            .iter()
            .filter(|&&word| !word.starts_with('-'))
            .cloned()
            .collect();
        let double_dash_options = words
            .iter()
            .filter(|&&word| word.starts_with("--"))
            .cloned()
            .collect();
        let single_dash_options = words
            .iter()
            .filter(|&&word| word.starts_with('-') && !word.starts_with("--"))
            .cloned()
            .collect();
        Opts {
            command,
            single_dash_options,
            double_dash_options,
            non_options,
        }
    }
    fn is_exist(&self, double_dash: Option<&str>, single_dash: Option<char>) -> bool {
        let mut exist = false;
        if let Some(double) = double_dash {
            exist = self
                .double_dash_options
                .iter()
                .any(|&word| format!("--{}", double).as_str() == word);
        }
        if let Some(single) = single_dash {
            exist = self
                .single_dash_options
                .iter()
                .any(|&word| word.contains(single));
        }
        exist
    }
    fn nth_non_option(&self, index: usize) -> &'a str {
        self.non_options[index]
    }
}

impl<'a> Commands<'a> {
    pub(crate) fn as_string(&self) -> String {
        use Commands::*;
        match self {
            Mkdir { .. } => "mkdir".to_string(),
            Rm { .. } => "rm".to_string(),
            Rmdir { .. } => "rmdir".to_string(),
            Ls { .. } => "ls".to_string(),
            Cp { .. } => "cp".to_string(),
            Mv { .. } => "mv".to_string(),
            Touch { .. } => "touch".to_string(),
            Cd { .. } => "cd".to_string(),
            Pwd => "pwd".to_string(),
            Pushd { .. } => "pushd".to_string(),
            Popd => "popd".to_string(),
            UnknowCommand { command } => command.to_string(),
        }
    }
    pub(crate) fn new(command: &'a str) -> Commands<'a> {
        let words = command.split(' ').collect();
        let opts = Opts::new(words);

        match opts.command {
            "mkdir" => Commands::generate_mkdir(opts),
            "rm" => Commands::generate_rm(opts),
            "rmdir" => Commands::generate_rmdir(opts),
            "ls" => Commands::generate_ls(opts),
            "cp" => Commands::generate_cp(opts),
            "mv" => Commands::generate_mv(opts),
            "touch" => Commands::generate_touch(opts),
            "cd" => Commands::generate_cd(opts),
            "pwd" => Commands::Pwd,
            "pushd" => Commands::generate_pushd(opts),
            "popd" => Commands::Popd,
            _ => Commands::generate_unknown(opts),
        }
    }
    fn generate_unknown(opts: Opts) -> Commands {
        Commands::UnknowCommand {
            command: opts.command,
        }
    }
    fn generate_cp(opts: Opts) -> Commands {
        //let recursive= split.filter(|&word| word == "--recursive");
        Commands::Cp {
            source: opts.nth_non_option(0),
            destination: opts.nth_non_option(1),
            recursive: opts.is_exist(Some("recursive"), Some('r')),
            symlink: opts.is_exist(Some("link"), Some('l')),
        }
    }
    fn generate_rm(opts: Opts) -> Commands {
        Commands::Rm {
            path: opts.nth_non_option(0),
            recursive: opts.is_exist(Some("recursive"), Some('r')),
            directory: opts.is_exist(Some("directory"), Some('d')),
        }
    }
    fn generate_rmdir(opts: Opts) -> Commands {
        Commands::Rmdir {
            path: opts.nth_non_option(0),
            parent: opts.is_exist(Some("parent"), Some('p')),
        }
    }
    fn generate_ls(opts: Opts) -> Commands {
        Commands::Ls {
            all: opts.is_exist(Some("all"), Some('a')),
            almost_all: opts.is_exist(Some("almost-all"), Some('A')),
            list: opts.is_exist(Some("list"), Some('l')),
            reverse: opts.is_exist(Some("reverse"), Some('r')),
            recursive: opts.is_exist(Some("recursive"), Some('R')),
        }
    }
    fn generate_touch(opts: Opts) -> Commands {
        Commands::Touch {
            file: opts.nth_non_option(0),
        }
    }
    fn generate_mv(opts: Opts) -> Commands {
        Commands::Mv {
            source: opts.nth_non_option(0),
            destination: opts.nth_non_option(1),
        }
    }
    fn generate_mkdir(opts: Opts) -> Commands {
        Commands::Mkdir {
            path: opts.nth_non_option(0),
            parent: opts.is_exist(Some("parent"), Some('p')),
        }
    }
    fn generate_cd(opts: Opts) -> Commands {
        Commands::Cd {
            path: opts.nth_non_option(0),
        }
    }
    fn generate_pushd(opts: Opts) -> Commands {
        Commands::Pushd {
            path: opts.nth_non_option(0),
        }
    }
}
