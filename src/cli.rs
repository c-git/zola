use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("gutenberg")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Path to a config file other than config.toml")
        )
        .subcommands(vec![
            SubCommand::with_name("init")
                .about("Create a new Gutenberg project")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("Name of the project. Will create a new directory with that name in the current directory")
                ),
            SubCommand::with_name("build")
                .about("Builds the site")
                .args(&[
                    Arg::with_name("base_url")
                        .short("u")
                        .long("base-url")
                        .help("Force the base URL to be that value (default to the one in config.toml)"),
                ]),
            SubCommand::with_name("serve")
                .about("Serve the site. Rebuild and reload on change automatically")
                .args(&[
                    Arg::with_name("interface")
                        .short("i")
                        .long("interface")
                        .default_value("127.0.0.1")
                        .help("Interface to bind on"),
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .default_value("1111")
                        .help("Which port to use"),
                ]),
        ])
}