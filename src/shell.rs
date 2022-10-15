mod alias_templates {
    use super::Shell;

    pub fn get_shell_template(shell: Shell) -> &'static str {
        match shell {
            Shell::Zsh => ZSH,
            Shell::Bash => BASH,
            Shell::Fish => FISH,
            Shell::Cmd => CMD,
            Shell::Powershell => POWERSHELL,
        }
    }

    const ZSH: &str = "SAVEPATH_ALIAS_PLACEHOLDER() {{
    SAP_CMD=$(
        savepathcmd $@
    ) && eval $SAP_CMD
}}";

    const BASH: &str = "function SAVEPATH_ALIAS_PLACEHOLDER () {
    SAP_CMD=$(
        savepathcmd $@
    ) && eval $SAP_CMD; 
}";

    const FISH: &str = "function SAVEPATH_ALIAS_PLACEHOLDER
    savepathcmd $argv | source
end\n";
// Need to use "sap -a -s fish | source" instead of eval $(sap -a -s fish) to setup

    const CMD: &str = "@echo off \n\
        savepathcmd !*& | source /C";

    const POWERSHELL: &str = "function SAVEPATH_ALIAS_PLACEHOLDER { \n\
        savepathcmd $args | source\n\
    }";
}

pub enum Shell {
    Zsh,
    Bash,
    Fish,
    Cmd,
    Powershell,
}

pub fn shell_from_str(s: &str) -> Option<Shell> {
    match &s.to_lowercase()[..] {
        "zsh" => Some(Shell::Zsh),
        "bash" => Some(Shell::Bash),
        "fish" => Some(Shell::Fish),
        // "cmd" => Some(Shell::Cmd), // TODO: Test cmd, powershell functions on windows
        // "powershell" => Some(Shell::Powershell),
        _ => None,
    }
}

pub fn print_alias(shell: Option<Shell>, alias_name: &str) {
    let shell: Shell = match shell {
        Some(shell) => shell,
        None => {
            eprintln!(
                "No valid shell specified, defaulting to Bash.\n 
            Please specify a supported shell with --shell [zsh|bash|fish]" // |cmd|powershell
            );
            // if cfg!(windows) {
            //     Shell::Powershell
            // // } else {
                Shell::Bash
            // }
        }
    };

    let script: String = alias_templates::get_shell_template(shell)
        .to_string()
        .replace("SAVEPATH_ALIAS_PLACEHOLDER", alias_name);
    println!("{}", script);
}
