extern crate pulsar_admin;
extern crate colored;
extern crate dialoguer;

use std::io;
use colored::*;
use std::error::{ Error };
use dialoguer::{ Input, Confirmation, Select };
use pulsar_admin::models::{ ConnectionSettings, Tenant };
use pulsar_admin::tenant;
use crate::format::*;

fn send_error(err: &str) -> Box<Error> {
  Box::new(io::Error::new(io::ErrorKind::Other, err))
}

/// List all tenants of a Pulsar instance
pub fn tenant_list(settings: ConnectionSettings, format: Format) -> Result<(), Box<Error>> {
  let cls = tenant::list(settings.host)?;
  let count = cls.len();

  match format {
    Format::Text => {
      let count_str = format!("{}", count).yellow();
      println!("Clusters: {}", count_str.green());
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

/// Show informations for a given tenant
/// If no tanant name is pecified, a choice between existing ones will be prompt
pub fn tenant_show(settings: ConnectionSettings, format: Format, name: Option<String>) -> Result<(), Box<Error>> {
  let mut final_name: String;
  match name {
    Some(n) => {
      final_name = n;
    },
    None => {
      let tnt = tenant::list(settings.host.clone())?;
      let i = Select::new()
        .with_prompt("Select a tenant to show")
        .items(&tnt)
        .interact()?;

      if let Some(tn) = tnt.get(i) {
        final_name = tn.clone();
      } else {
        return Err(send_error("ununderstandable choice"));
      }
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

/// Create a new tenant
/// For the name / admin roles / allowed cluster, if the field is empty it will be prompt
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

  let final_admin_roles = if !admin_roles.is_empty() {
    admin_roles.clone()
  } else {
    let input = Input::<String>::new().with_prompt("Admin roles (coma separated)?").interact()?;
    let splits = input.split(',');
    splits.map(|split| {
      split.trim().to_string()
    }).collect()
  };

  let final_allowed_clusters = if !allowed_clusters.is_empty() {
    allowed_clusters.clone()
  } else {
    let input = Input::<String>::new().with_prompt("Allowed clusters (coma separated)?").interact()?;
    let splits = input.split(',');
    splits.map(|split| {
      split.trim().to_string()
    }).collect()
  };

  println!("New Cluster: name='{}' roles='{}' clusters='{}'", final_name, final_admin_roles.join("|"), final_allowed_clusters.join("|"));

  if !force {
    let confirm = Confirmation::new().with_text("Is it OK?").default(false).interact()?;
    if !confirm {
      return Err(send_error("user abort action"));
    }
  }

  let tn = tenant::create(settings.host, Tenant{
      name: final_name,
      admin_roles: final_admin_roles,
      allowed_clusters: final_allowed_clusters
    })?;

  match format {
    Format::Text => {
      println!("Tenant '{}' successfully created !", tn.name.blue());
    },
    Format::JSON => {
      serde_json::to_writer_pretty(std::io::stdout(), &tn)?;
    }
  }

  Ok(())
}

pub fn tenant_update(
  settings: ConnectionSettings,
  format: Format,
  name: String,
  admin_roles: Vec<String>,
  allowed_clusters: Vec<String>
) -> Result<(), Box<Error>> {
  if name.is_empty() {
    return Err(send_error("Tenant name mustn't be empty to delete it"))
  }

  let tn = tenant::update(settings.host, Tenant{
    name, admin_roles, allowed_clusters
  })?;

  match format {
    Format::Text => {
      println!("Tenant '{}' updated !", tn.name.green());
    },
    Format::JSON => {
      serde_json::to_writer(std::io::stdout(), &tn)?;
    }
  }

  Ok(())
}

pub fn tenant_delete(settings: ConnectionSettings, format: Format, tenant_name: String) -> Result<(), Box<Error>> {
  if tenant_name.is_empty() {
    return Err(send_error("Tenant name mustn't be empty to delete it"))
  }

  tenant::delete(settings.host, tenant_name.as_str())?;

  match format {
    Format::Text => {
      println!("Tenant '{}' deleted !", tenant_name.green());
    },
    Format::JSON => {
      serde_json::to_writer(std::io::stdout(), &tenant_name)?;
    }
  }

  Ok(())
}