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
        let mut app = App::new("run1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190916")
            .about("Deploy and manage user provided container images that scale automatically based on HTTP traffic.")
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
        let mut namespaces0 = SubCommand::with_name("namespaces")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: authorizeddomains, autodomainmappings, configurations, domainmappings, revisions, routes and services");
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut authorizeddomains1 = SubCommand::with_name("authorizeddomains")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List authorized domains.");
            authorizeddomains1 = authorizeddomains1.subcommand(mcmd);
        }
        let mut autodomainmappings1 = SubCommand::with_name("autodomainmappings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and replace_auto_domain_mapping");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new auto domain mapping.");
            autodomainmappings1 = autodomainmappings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an auto domain mapping.");
            autodomainmappings1 = autodomainmappings1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get information about an auto domain mapping.");
            autodomainmappings1 = autodomainmappings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List auto domain mappings.");
            autodomainmappings1 = autodomainmappings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_auto_domain_mapping").about("Replace an auto domain mapping.\n\nOnly the spec and metadata labels and annotations are modifiable. After\nthe Update request, Cloud Run will work to make the \'status\'\nmatch the requested \'spec\'.\n\nMay provide metadata.resourceVersion to enforce update from last read for\noptimistic concurrency control.");
            autodomainmappings1 = autodomainmappings1.subcommand(mcmd);
        }
        let mut configurations1 = SubCommand::with_name("configurations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and replace_configuration");
        {
            let mcmd = SubCommand::with_name("create").about("Create a configuration.");
            configurations1 = configurations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("delete a configuration.\nThis will cause the configuration to delete all child revisions. Prior to\ncalling this, any route referencing the configuration (or revision\nfrom the configuration) must be deleted.");
            configurations1 = configurations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get information about a configuration.");
            configurations1 = configurations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List configurations.");
            configurations1 = configurations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_configuration").about("Replace a configuration.\n\nOnly the spec and metadata labels and annotations are modifiable. After\nthe Update request, Cloud Run will work to make the \'status\'\nmatch the requested \'spec\'.\n\nMay provide metadata.resourceVersion to enforce update from last read for\noptimistic concurrency control.");
            configurations1 = configurations1.subcommand(mcmd);
        }
        let mut domainmappings1 = SubCommand::with_name("domainmappings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and replace_domain_mapping");
        {
            let mcmd = SubCommand::with_name("create").about("Create a new domain mapping.");
            domainmappings1 = domainmappings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a domain mapping.");
            domainmappings1 = domainmappings1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get information about a domain mapping.");
            domainmappings1 = domainmappings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List domain mappings.");
            domainmappings1 = domainmappings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_domain_mapping").about("Replace a domain mapping.\n\nOnly the spec and metadata labels and annotations are modifiable. After\nthe Update request, Cloud Run will work to make the \'status\'\nmatch the requested \'spec\'.\n\nMay provide metadata.resourceVersion to enforce update from last read for\noptimistic concurrency control.");
            domainmappings1 = domainmappings1.subcommand(mcmd);
        }
        let mut revisions1 = SubCommand::with_name("revisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a revision.");
            revisions1 = revisions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get information about a revision.");
            revisions1 = revisions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List revisions.");
            revisions1 = revisions1.subcommand(mcmd);
        }
        let mut routes1 = SubCommand::with_name("routes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and replace_route");
        {
            let mcmd = SubCommand::with_name("create").about("Create a route.");
            routes1 = routes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete a route.\nThis will cause the route to stop serving traffic.");
            routes1 = routes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get information about a route.");
            routes1 = routes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List routes.");
            routes1 = routes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_route").about("Replace a route.\n\nOnly the spec and metadata labels and annotations are modifiable. After\nthe Update request, Cloud Run will work to make the \'status\'\nmatch the requested \'spec\'.\n\nMay provide metadata.resourceVersion to enforce update from last read for\noptimistic concurrency control.");
            routes1 = routes1.subcommand(mcmd);
        }
        let mut services1 = SubCommand::with_name("services")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and replace_service");
        {
            let mcmd = SubCommand::with_name("create").about("Create a service.");
            services1 = services1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a service.\nThis will cause the Service to stop serving traffic and will delete the\nchild entities like Routes, Configurations and Revisions.");
            services1 = services1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get information about a service.");
            services1 = services1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List services.");
            services1 = services1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_service").about("Replace a service.\n\nOnly the spec and metadata labels and annotations are modifiable. After\nthe Update request, Cloud Run will work to make the \'status\'\nmatch the requested \'spec\'.\n\nMay provide metadata.resourceVersion to enforce update from last read for\noptimistic concurrency control.");
            services1 = services1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut authorizeddomains2 = SubCommand::with_name("authorizeddomains")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List authorized domains.");
            authorizeddomains2 = authorizeddomains2.subcommand(mcmd);
        }
        let mut autodomainmappings2 = SubCommand::with_name("autodomainmappings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and replace_auto_domain_mapping");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new auto domain mapping.");
            autodomainmappings2 = autodomainmappings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an auto domain mapping.");
            autodomainmappings2 = autodomainmappings2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get information about an auto domain mapping.");
            autodomainmappings2 = autodomainmappings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List auto domain mappings.");
            autodomainmappings2 = autodomainmappings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_auto_domain_mapping").about("Replace an auto domain mapping.\n\nOnly the spec and metadata labels and annotations are modifiable. After\nthe Update request, Cloud Run will work to make the \'status\'\nmatch the requested \'spec\'.\n\nMay provide metadata.resourceVersion to enforce update from last read for\noptimistic concurrency control.");
            autodomainmappings2 = autodomainmappings2.subcommand(mcmd);
        }
        let mut configurations2 = SubCommand::with_name("configurations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and replace_configuration");
        {
            let mcmd = SubCommand::with_name("create").about("Create a configuration.");
            configurations2 = configurations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("delete a configuration.\nThis will cause the configuration to delete all child revisions. Prior to\ncalling this, any route referencing the configuration (or revision\nfrom the configuration) must be deleted.");
            configurations2 = configurations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get information about a configuration.");
            configurations2 = configurations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List configurations.");
            configurations2 = configurations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_configuration").about("Replace a configuration.\n\nOnly the spec and metadata labels and annotations are modifiable. After\nthe Update request, Cloud Run will work to make the \'status\'\nmatch the requested \'spec\'.\n\nMay provide metadata.resourceVersion to enforce update from last read for\noptimistic concurrency control.");
            configurations2 = configurations2.subcommand(mcmd);
        }
        let mut domainmappings2 = SubCommand::with_name("domainmappings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and replace_domain_mapping");
        {
            let mcmd = SubCommand::with_name("create").about("Create a new domain mapping.");
            domainmappings2 = domainmappings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a domain mapping.");
            domainmappings2 = domainmappings2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get information about a domain mapping.");
            domainmappings2 = domainmappings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List domain mappings.");
            domainmappings2 = domainmappings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_domain_mapping").about("Replace a domain mapping.\n\nOnly the spec and metadata labels and annotations are modifiable. After\nthe Update request, Cloud Run will work to make the \'status\'\nmatch the requested \'spec\'.\n\nMay provide metadata.resourceVersion to enforce update from last read for\noptimistic concurrency control.");
            domainmappings2 = domainmappings2.subcommand(mcmd);
        }
        let mut revisions2 = SubCommand::with_name("revisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a revision.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get information about a revision.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List revisions.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        let mut routes2 = SubCommand::with_name("routes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and replace_route");
        {
            let mcmd = SubCommand::with_name("create").about("Create a route.");
            routes2 = routes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete a route.\nThis will cause the route to stop serving traffic.");
            routes2 = routes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get information about a route.");
            routes2 = routes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List routes.");
            routes2 = routes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_route").about("Replace a route.\n\nOnly the spec and metadata labels and annotations are modifiable. After\nthe Update request, Cloud Run will work to make the \'status\'\nmatch the requested \'spec\'.\n\nMay provide metadata.resourceVersion to enforce update from last read for\noptimistic concurrency control.");
            routes2 = routes2.subcommand(mcmd);
        }
        let mut services2 = SubCommand::with_name("services")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, replace_service, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Create a service.");
            services2 = services2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a service.\nThis will cause the Service to stop serving traffic and will delete the\nchild entities like Routes, Configurations and Revisions.");
            services2 = services2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get information about a service.");
            services2 = services2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Get the IAM Access Control policy currently in effect for the given\nCloud Run service. This result does not include any inherited policies.");
            services2 = services2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List services.");
            services2 = services2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_service").about("Replace a service.\n\nOnly the spec and metadata labels and annotations are modifiable. After\nthe Update request, Cloud Run will work to make the \'status\'\nmatch the requested \'spec\'.\n\nMay provide metadata.resourceVersion to enforce update from last read for\noptimistic concurrency control.");
            services2 = services2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the IAM Access control policy for the specified Service. Overwrites\nany existing policy.");
            services2 = services2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified Project.\n\nThere are no permissions required for making this API call.");
            services2 = services2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(services2);
        locations1 = locations1.subcommand(routes2);
        locations1 = locations1.subcommand(revisions2);
        locations1 = locations1.subcommand(domainmappings2);
        locations1 = locations1.subcommand(configurations2);
        locations1 = locations1.subcommand(autodomainmappings2);
        locations1 = locations1.subcommand(authorizeddomains2);
        projects0 = projects0.subcommand(locations1);
        namespaces0 = namespaces0.subcommand(services1);
        namespaces0 = namespaces0.subcommand(routes1);
        namespaces0 = namespaces0.subcommand(revisions1);
        namespaces0 = namespaces0.subcommand(domainmappings1);
        namespaces0 = namespaces0.subcommand(configurations1);
        namespaces0 = namespaces0.subcommand(autodomainmappings1);
        namespaces0 = namespaces0.subcommand(authorizeddomains1);
        app = app.subcommand(projects0);
        app = app.subcommand(namespaces0);

        Self { app }
    }
}
use google_run1 as api;

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
