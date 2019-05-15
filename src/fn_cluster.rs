extern crate pulsar_admin;
extern crate colored;
extern crate dialoguer;

use std::error::{ Error };
use std::io;
use colored::*;
use pulsar_admin::cluster;
use pulsar_admin::models::Cluster;
use dialoguer::{Input, Confirmation};
use crate::format::*;
use pulsar_admin::models::ConnectionSettings;

pub fn cluster_list(settings: ConnectionSettings, format: Format) -> Result<(), Box<Error>> {
  let cls = cluster::list(settings.host)?;
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

pub fn cluster_show(settings: ConnectionSettings, format: Format, name: String) -> Result<(), Box<Error>> {
  let cls: Cluster = cluster::get(settings.host, &name)?;
  println!("Cluster: {}", cls.name.blue() );
  println!("Service URL: {}", cls.service_url.to_owned().green());
  println!("Broker service URL: {}", cls.broker_service_url.clone().green());

  if let Some(broker_service_url_tls) = cls.broker_service_url_tls {
      println!("Broker service URL (TLS): {}", broker_service_url_tls.clone().green())
  }
  if let Some(service_url_tls) = cls.service_url_tls {
      println!("Service URL (TLS): {}", service_url_tls.clone().green())
  }
  if let Some(peer_cluster_names) = cls.peer_cluster_names {
      println!("Peer cluster names: {}", peer_cluster_names.join(",").green())
  }

  Ok(())
}

pub fn cluster_create(settings: ConnectionSettings, format: Format) -> Result<(), Box<Error>> {
  let name = Input::<String>::new()
    .with_prompt("New cluster name?")
    .interact()?;

  let url = Input::<String>::new()
    .with_prompt("URL?")
    .interact()?;

  let broker_url = Input::<String>::new()
    .with_prompt("Broker URL?")
    .interact()?;

  println!("New Cluster: name='{}' url='{}' broker_url='{}'", name, url, broker_url);

  if !Confirmation::new().with_text("Are you OK?").interact()? {
    let cls = cluster::create(settings.host, name, url, broker_url)?;


    Ok(())
  } else {
    Err(Box::new(io::Error::new(io::ErrorKind::Other, "user abort action")))
  }
}