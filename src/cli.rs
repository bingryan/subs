use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
	App::new("subs")
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.setting(AppSettings::ColoredHelp)
		.arg(
			Arg::with_name("config")
				.short("c")
				.long("config")
				.takes_value(true)
				.help("Path to a config file other than config.toml in the root of user"),
		)
		.subcommands(vec![
			SubCommand::with_name("init").about("Init subs command line config"),
			SubCommand::with_name("new")
				.about("Create a new pallet")
				.args(&[Arg::with_name("dir")
					.short("d")
					.long("dir")
					.default_value("pallet")
					.required(true)
					.help("Create a pallet in this directory")]),
		])
}
