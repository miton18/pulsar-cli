extern crate structopt;

mod fn_cluster;
mod fn_tenant;
mod format;

use structopt::clap::Shell;
use structopt::StructOpt;
use std::error::{ Error };
use format::Format;
use pulsar_admin::models::ConnectionSettings;

#[derive(StructOpt, Debug, Clone)]
enum ClusterCommands {
    #[structopt(name = "list", alias = "ls")]
    List {},

    #[structopt(name = "show", alias = "describe")]
    Show {
        #[structopt(name = "CLUSTER_NAME")]
        name: String,
    },

    #[structopt(name = "create", alias = "add")]
    Create {}
}

#[derive(StructOpt, Debug, Clone)]
enum TenantCommands {
    #[structopt(name = "list", alias = "ls")]
    List {},

    #[structopt(name = "show", alias = "describe")]
    Show {
        #[structopt(name = "CLUSTER_NAME")]
        name: Option<String>,
    },

    #[structopt(name = "create", alias = "add")]
    Create {
        #[structopt(long = "name")]
        name: Option<String>,

        #[structopt(long = "admin-role")]
        admin_roles: Vec<String>,

        #[structopt(long = "allowed-cluster")]
        allowed_clusters: Vec<String>,
    },

    #[structopt(name = "update", alias = "set")]
    Update {}
}

#[derive(StructOpt, Debug, Clone)]
enum Commands {
    #[structopt(name = "cluster")]
    Cluster {
        #[structopt(subcommand)]
        cluster_cmd: ClusterCommands
    },

    #[structopt(name = "tenant")]
    Tenant {
        #[structopt(subcommand)]
        tenant_cmd: TenantCommands
    },

    #[structopt(name="completion")]
    Completion {}
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "Pulsar CLI", about = "Pulsar command line interface")]
struct Cmd {
    #[structopt(short = "h", long = "host", default_value = "http://localhost:8080")]
    host: String,

    #[structopt(long = "format", default_value = "text")]
    format: String,

    #[structopt(short = "f", long = "force")]
    force: bool,

    #[structopt(subcommand)]
    cmd: Commands
}

fn main() -> Result<(), Box<Error>> {
    let opt = Cmd::from_args();
    let format = Format::from_string(opt.format);

    let conn_settings = ConnectionSettings{
        host: opt.host,
    };

    match opt.cmd {
        Commands::Cluster{cluster_cmd} => {
            match cluster_cmd {
                ClusterCommands::List{} => {
                    fn_cluster::cluster_list(conn_settings, format)
                }
                ClusterCommands::Show{name} => {
                    fn_cluster::cluster_show(conn_settings, format, name)
                }
                ClusterCommands::Create{} => {
                    fn_cluster::cluster_create(conn_settings, format)
                }
            }
        },
        Commands::Tenant{tenant_cmd} => {
            match tenant_cmd {
                TenantCommands::List{} => {
                    fn_tenant::tenant_list(conn_settings, format)
                },
                TenantCommands::Show{name} => {
                    fn_tenant::tenant_show(conn_settings, format, name)
                },
                TenantCommands::Create{name, admin_roles, allowed_clusters} => {
                    fn_tenant::tenant_create(conn_settings, format, name, admin_roles, allowed_clusters, opt.force)
                }
                TenantCommands::Update{} => {
                    Ok(())
                }
            }
        },
        Commands::Completion{} => {
            Cmd::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");
            println!("Completion available under target directory");
            Ok(())
        }
    }
}
