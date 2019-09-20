use clap::{App, AppSettings, Arg, SubCommand};
use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Outer<'a, 'b> {
    inner: HeapApp<'a, 'b>,
}

struct HeapApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Default for HeapApp<'a, 'b> {
    fn default() -> Self {
        let mut app = App::new("admin1_reports")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190521")
            .about("Fetches reports for the administrators of G Suite customers about the usage, collaboration, security, and risk for their users.")
            .after_help("All documentation details can be found at <TODO figure out URL>")
            .arg(Arg::with_name("scope")
                .long("scope")
                .help("Specify the authentication method should be executed in. Each scope requires the user to grant this application permission to use it. If unset, it defaults to the shortest scope url for a particular method.")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("folder")
                .long("config-dir")
                .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation." )
                .multiple(false)
                .takes_value(true))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Provide more output to aid with debugging")
                .multiple(false)
                .takes_value(false));
        let mut activities0 = SubCommand::with_name("activities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and watch");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves a list of activities for a specific customer and application.");
            activities0 = activities0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Push changes to activities");
            activities0 = activities0.subcommand(mcmd);
        }
        let mut channels0 = SubCommand::with_name("channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: stop");
        {
            let mcmd =
                SubCommand::with_name("stop").about("Stop watching resources through this channel");
            channels0 = channels0.subcommand(mcmd);
        }
        let mut customer_usage_reports0 = SubCommand::with_name("customer_usage_reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a report which is a collection of properties / statistics for a specific customer.");
            customer_usage_reports0 = customer_usage_reports0.subcommand(mcmd);
        }
        let mut entity_usage_reports0 = SubCommand::with_name("entity_usage_reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a report which is a collection of properties / statistics for a set of objects.");
            entity_usage_reports0 = entity_usage_reports0.subcommand(mcmd);
        }
        let mut user_usage_report0 = SubCommand::with_name("user_usage_report")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a report which is a collection of properties / statistics for a set of users.");
            user_usage_report0 = user_usage_report0.subcommand(mcmd);
        }
        app = app.subcommand(user_usage_report0);
        app = app.subcommand(entity_usage_reports0);
        app = app.subcommand(customer_usage_reports0);
        app = app.subcommand(channels0);
        app = app.subcommand(activities0);

        Self { app }
    }
}
use google_admin1_reports as api;

fn main() {
    // TODO: set homedir afterwards, once the address is unmovable, or use Pin for the very first time
    // to allow a self-referential structure :D!
    let _home_dir = dirs::config_dir()
        .expect("configuration directory can be obtained")
        .join("google-service-cli");
    let outer = Outer::default_boxed();
    let app = outer.inner.app;
    let _matches = app.get_matches();
}
