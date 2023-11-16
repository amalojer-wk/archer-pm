use std::fmt::Display;

use archer_package_manager::packages::processing::PackageObject;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, PartialEq, Parser)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum Command {
    #[command(name = "man")]
    Manager,
    #[command(name = "mod", about = "Modify an existing package")]
    Modifier {
        #[command(subcommand)]
        operation: ModiferOperation,
    },
    #[cfg(feature = "with-info")]
    #[command(name = "info", about = "Report Information about a package's contents")]
    Info {
        #[command(subcommand)]
        operation: InformationOperation,
        #[arg(
            conflicts_with = "path",
            short,
            help = "The name of the package as stored in the management DB",
            required_unless_present = "path"
        )]
        name: Option<String>,
        #[arg(
            conflicts_with = "name",
            short,
            help = "The path to the archer zip file",
            required_unless_present = "name"
        )]
        path: Option<String>,
    },
}

#[derive(ValueEnum, PartialEq, Debug, Hash, Clone, Copy)]
pub enum ExportDataFormat {
    Readable,
    #[cfg(feature = "json_exporter")]
    JSON,
    #[cfg(feature = "xml_exporter")]
    XML,
    #[cfg(feature = "toml_exporter")]
    TOML,
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum ModiferOperation {
    #[command(
        short_flag = 'r',
        long_flag = "rm-chk",
        about = "Remove the checksum from an existing package"
    )]
    RemoveChecksum {
        #[arg(
            conflicts_with = "path",
            short,
            help = "The name of the package as stored in the management DB",
            required_unless_present = "path"
        )]
        name: Option<String>,
        #[arg(
            conflicts_with = "name",
            short,
            help = "The path to the archer zip file",
            required_unless_present = "name"
        )]
        path: Option<String>,
        #[arg(short, help = "The path to the output zip file")]
        output_path: Option<String>,
        #[arg(short, long, help = "Show verbose output")]
        verbose: bool,
    },
    #[command(
        short_flag = 'a',
        long_flag = "add-chk",
        about = "Generate and add the checksum to an existing package"
    )]
    AddChecksum {
        #[arg(
            conflicts_with = "path",
            short,
            help = "The name of the package as stored in the management DB"
        )]
        name: Option<String>,
        #[arg(short, help = "Remove the checksum if present in the zip file")]
        remove_checksum: bool,
        #[arg(
            short,
            help = "The path to the archer zip file",
            required_unless_present = "name",
            conflicts_with = "name"
        )]
        path: Option<String>,
        #[arg(short, help = "The path to the output zip file")]
        output_path: Option<String>,
    },
    #[command(
        short_flag = 'm',
        long_flag = "mk-pkg",
        about = "Create a new package from a directory"
    )]
    MakePackage {
        #[arg(
            short = 'a',
            long = "add",
            help = "Add the package to the database",
            requires = "name"
        )]
        add_to_db: bool,
        input_directory: String,
        #[arg(
            long = "name",
            help = "Specify the name of the package if adding to the db"
        )]
        name: Option<String>,
        #[arg(long = "version", help = "Specify the version of the package")]
        version: Option<String>,
        #[arg(short, long, help = "Don't strip the base path from the zip file")]
        full_paths: bool,
        #[arg(
            name = "output_path",
            short = 'o',
            long = "output",
            help = "Specify the output path for the package"
        )]
        output_path: Option<String>,
        #[arg(short, long, help = "Show verbose output")]
        verbose: bool,
    },
    #[clap(short_flag = 'b', about = "Bulk update a object in a package")]
    BulkUpdate {
        #[arg(value_name = "OBJECT")]
        object: PackageObject,
        #[arg(
            help = "The keys to update, specify multiple using commas. (e.g. Description, Name)"
        )]
        key: String,
        #[arg(
            help = "The values to update the keys to, 1 value must be provided for each key. Some formatting arguments are supported, use '*' to insert the original field value or '{key_name}' to take the value of another key."
        )]
        value: String,
        #[arg(short = 'v', long = "verbose", help = "Display verbose output")]
        verbose: bool,
        #[arg(
            short = 'd',
            long = "dry",
            help = "Dry run, print changes without performing them"
        )]
        dry_run: bool,
        #[arg(
            short = 'f',
            long = "filter",
            help = "Specify a filter to filter the objects. A filter should use a '=' or '*' to either match exactly or check if the field contains. For example, '{Name}*Controls', would select any objects with a name field that contains the exact string 'Controls'."
        )]
        filter: Option<String>,
    },
}

#[cfg(feature = "with-info")]
#[derive(Debug, PartialEq, Subcommand)]
pub enum InformationOperation {
    #[command(
        short_flag = 'o',
        name = "overview",
        about = "Print information about a package"
    )]
    Overview {
        #[arg(short, help = "Lists package contents in increased detail")]
        detailed: bool,
    },
    #[command(
        short_flag = 'a',
        name = "apps",
        about = "Print information about the applications in a package"
    )]
    Applications {
        #[arg(short = 'l', long = "list-apps", help = "Lists all applications")]
        list_applications: bool,
        #[arg(
            short = 'a',
            long = "list-aw",
            help = "Lists all applications with advanced workflow"
        )]
        list_aw: bool,
    },
    #[command(
        short_flag = 'd',
        name = "datafeeds",
        about = "Print information about the datafeeds in a packages"
    )]
    Datafeeds {
        #[arg(short = 'a', help = "Lists all datafeeds")]
        list_all: bool,
        #[arg(short = 'd', help = "Lists all datafeeds with details")]
        list_detailed: bool,
        #[arg(short, help = "Specify the data format", default_value_t = Default::default())]
        format: ExportDataFormat,
    },
    Notifications,
    Solutions,
    Dashboards,
    Workspaces,
    DataDrivenEvents,
    Levels,
}

impl Default for ExportDataFormat {
    fn default() -> Self {
        return Self::Readable;
    }
}

impl Display for ExportDataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                ExportDataFormat::Readable => "Readable",
                #[cfg(feature = "json_exporter")]
                ExportDataFormat::JSON => "JSON",
                #[cfg(feature = "xml_exporter")]
                ExportDataFormat::XML => "XML",
                #[cfg(feature = "toml_exporter")]
                ExportDataFormat::TOML => "TOML",
            }
        );
    }
}
