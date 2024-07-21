use clap::{Args, command, Parser, ValueEnum};


#[derive(clap::Parser, Debug)]
#[command(name = "epher", version, about, author, long_about = None)]
pub struct Argument {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, short='v',global = true)]
    verbose:bool,
    #[arg(long, global = true)]
    debug:bool
}

impl Argument {
    pub fn exec(&self){
        if self.debug {
            println!("{:?}", self);
        }
        match &self.command {
            Commands::Add { .. } => {}
            Commands::Install { .. } => {}
            Commands::Env { .. } => {}
            #[cfg(feature="utils")]
            Commands::Utils{command} => {
                crate::cmd::utils::utils_cmd(self,command)
            }
        }
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InstallMode {
    Ephemeral,
    Local,
    Permanent,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct InstallModeArg {
    //#[clap(short, long,value_enum)]
    #[arg(long, short, value_enum, default_value_t = InstallMode::Ephemeral)]
    save_mode: InstallMode,
    #[arg(long, short)]
    ephemeral: bool,
    #[arg(long, short)]
    local: bool,
    #[arg(long, short)]
    permanent: bool,
}

impl InstallModeArg {
    pub fn to_value(self) -> InstallMode {
        if self.ephemeral {
            return InstallMode::Ephemeral;
        } else if self.local {
            return InstallMode::Local;
        } else if self.permanent {
            return InstallMode::Permanent;
        }
        return self.save_mode;
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    
    #[command(alias = "a", about = "install epher packages")]
    Add {
        #[command(flatten)]
        save_mode: InstallModeArg,
        /// Add & Install
        #[arg(short, long)]
        install: bool,
    },
    #[command(alias = "i", about = "install epher packages")]
    Install {
        /// A name of package to install
        #[arg()]
        package_name: Option<String>,
    },
    Env {},
    #[cfg(feature="utils")]
    #[command()]
    Utils{
        #[command(subcommand)]
        command: crate::cmd::utils::UtilsCommand,
    },
}




pub fn parse_args() -> Argument {
    return Argument::parse();
}