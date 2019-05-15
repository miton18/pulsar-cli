extern crate pulsar_admin;
extern crate colored;
extern crate dialoguer;

use std::error::{ Error };
use colored::*;
use std::io;
use pulsar_admin::tenant;
use pulsar_admin::models::Tenant;
use dialoguer::{Input, Confirmation, Select};
use crate::format::*;
use pulsar_admin::models::ConnectionSettings;

pub fn tenant_list(settings: ConnectionSettings, format: Format) -> Result<(), Box<Error>> {
  let cls = tenant::list(settings.host)?;
  let count = cls.len();

  match format {
    Format::Text => {
      let count_str = format!("{}", count).yellow();
      println!("{} {}", "Clusters: ", count_str);
      for cl in cls {
        println!("{}", cl.blue());
      }
    },
    Format::JSON => {
      serde_json::to_writer(std::io::stdout(), &cls)?;
    }
  }
  Ok(())
}

pub fn tenant_show(settings: ConnectionSettings, format: Format, name: Option<String>) -> Result<(), Box<Error>> {
  let mut final_name: String;
  match name {
    Some{n} => {
      final_name = n;
    },
    None => {
      let cls = tenant::list(settings.host)?;
      final_name = Select::new().with_prompt("Select a tenant to show").items(&cls).interact()?;
    }
  }


  let tenant: Tenant = tenant::get(settings.host, &final_name)?;

  match format {
    Format::Text => {
      println!("Tenant: {}", tenant.name.blue() );
      println!("Admin roles: {}", tenant.admin_roles.join(", ").green());
      println!("Allowed clusters: {}", tenant.allowed_clusters.join(", ").green());

    },
    Format::JSON => {
      serde_json::to_writer_pretty(std::io::stdout(), &tenant)?;
    }
  }
  Ok(())
}

pub fn tenant_create(
  settings: ConnectionSettings,
  format: Format,
  name: Option<String>,
  admin_roles: Vec<String>,
  allowed_clusters: Vec<String>,
  force: bool
) -> Result<(), Box<Error>> {

  let final_name: String;
  if let Some(name) = name {
    final_name = name;
  } else {
    final_name = Input::<String>::new()
    .with_prompt("New tenant name?")
    .interact()?;
  }

  let mut final_admin_roles = admin_roles.clone();
  if final_admin_roles.len() == 0 {
    let input = Input::<String>::new().with_prompt("Admin roles (coma separated)?").interact()?;
    let splits = input.split(",");
    final_admin_roles = splits.map(|split| {
      split.trim().to_string()
    }).collect();
  }

let mut final_allowed_clusters = allowed_clusters.clone();
  if allowed_clusters.len() == 0 {
    let input = Input::<String>::new().with_prompt("Allowed clusters (coma separated)?").interact()?;
    let splits = input.split(",");
    final_allowed_clusters = splits.map(|split| {
      split.trim().to_string()
    }).collect();
  }

  println!("New Cluster: name='{}' roles='{}' clusters='{}'", final_name, final_admin_roles.join("|"), final_allowed_clusters.join("|"));

  if !force {
    let confirm = Confirmation::new().with_text("Is it OK?").default(false).interact()?;
    if !confirm {
      return Err(Box::new(io::Error::new(io::ErrorKind::Other, "user abort action")));
    }
  }

  let cls = tenant::create(settings.host, Tenant{
      name: final_name,
      admin_roles: final_admin_roles,
      allowed_clusters: final_allowed_clusters
    })?;

    println!("Tenant '{}' successfully created !", cls.name.blue());

    return Ok(());
}