use std::env;
use std::process::exit;
use std::net::IpAddr;

use anyhow::{anyhow, Result};
use trust_dns_resolver::{
    AsyncResolver,
    config::*};


#[tokio::main]
async fn main() -> Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("\n\nUsage: dns-rs-example domain\n");
        exit(0);
    }
    let host = args.nth(1).unwrap();
    let ip: IpAddr = "180.76.76.76".parse()?;
    let address = lookup(Some(&[ip]), host).await?;
    println!("address: {}", address);
    Ok(())
}


async fn lookup(nameservers: Option<&[IpAddr]>, host: String) -> Result<IpAddr> {
    let resolver = match nameservers {
        Some(nameservers) => {
            let nameserver_group = NameServerConfigGroup::from_ips_clear(nameservers, 53);
            AsyncResolver::tokio(ResolverConfig::from_parts(None, vec![], nameserver_group), ResolverOpts::default()).await?
        },
        None => AsyncResolver::tokio_from_system_conf().await?
    };
    let response = resolver.lookup_ip(host).await?;
    match response.iter().next() {
        Some(address) => Ok(address),
        None => Err(anyhow!("not found lookup address"))
    }
}
