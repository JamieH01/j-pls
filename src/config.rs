use std::{path::{PathBuf, Path}, fs::File, io::Read};

use global_static::Global;

use crate::parse::clear_between;

pub static CONFIG: Global<Config> = Global::new(Config::new);

pub struct Config {
    pub xdg: xdg::BaseDirectories,
    pub look: PathBuf,
    pub global: PathBuf,
    pub show_cmd: bool, 
}

macro_rules! find_settings {
    (($str:tt, $var:ident): $($look_for:tt => $bl:block)*) => {
        $(
            if let Some(($look_for, $var)) = $str.split_once(':') $bl 
        )*
    };
}

impl Config {
    pub fn new() -> Config {
        let xdg = xdg::BaseDirectories::with_prefix("pls").unwrap();

        let mut settings = String::new();
        if let Ok(mut file) = File::open(xdg.get_config_file("config.pls")) {
            let _ = file.read_to_string(&mut settings);
        }

        settings = clear_between(settings, '#', '\n');

        let mut look = "rules.pls".into(); //default
        let mut global = "global.pls".into(); //default
        let mut show_cmd = false;

        for line in settings.lines() {
            find_settings!((line, back): 
                "look" => { look = back.trim().into() }
                "global" => { global = back.trim().into() }
                "show_cmd"  => { if back.trim() == "true" { show_cmd = true } }
            );
        }
         

        Config { xdg, look, global, show_cmd }
    }

    pub fn look(&self) -> &Path {
        &self.look
    }
    pub fn global(&self) -> PathBuf {
        self.xdg.get_config_file(&self.global)
    }
}
