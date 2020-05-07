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
        let mut app = App::new("licensing1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20170213")
            .about("Views and manages licenses for your domain.")
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
        let mut license_assignments0 = SubCommand::with_name("license_assignments")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, insert, list_for_product, list_for_product_and_sku, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Revoke License.");
            license_assignments0 = license_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get license assignment of a particular product and sku for a user");
            license_assignments0 = license_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Assign License.");
            license_assignments0 = license_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_for_product")
                .about("List license assignments for given product of the customer.");
            license_assignments0 = license_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_for_product_and_sku")
                .about("List license assignments for given product and sku of the customer.");
            license_assignments0 = license_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Assign License. This method supports patch semantics.");
            license_assignments0 = license_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Assign License.");
            license_assignments0 = license_assignments0.subcommand(mcmd);
        }
        app = app.subcommand(license_assignments0);

        Self { app }
    }
}
use google_licensing1 as api;

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