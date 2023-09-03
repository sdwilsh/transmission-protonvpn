use anyhow::{anyhow, bail, Result};
use natpmp::*;
use reqwest::blocking::Client;
use reqwest::Url;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let gateway = std::env::var("NATPMP_GATEWAY_IP").unwrap_or("10.2.0.1".to_owned());
    let mut n =
        Natpmp::new_with((&gateway).parse().unwrap()).expect("Parsing gateway address failed!");
    let mut client = Client::default();

    let _ = query_gateway(&mut n).expect("Quering Public IP failed!");

    let mut mr = query_available_port(&mut n).expect("Quering a Port Mapping failed!");
    update_qbittorrent(&mut client, mr.public_port()).expect("Failed to update QBittorrent.");

    loop {
        thread::sleep(mr.lifetime().clone() / 2); //Renew at half lifetime
        let mr_ = query_port(&mut n, mr.private_port(), mr.public_port(), true)
            .or(query_available_port(&mut n))
            .expect("Every renewal method failed!");
        if mr.public_port() != mr_.public_port() {
            println!("Port has changed, setting incoming port on QBittorrent...");
            update_qbittorrent(&mut client, mr.public_port())
                .expect("Failed to update QBittorrent.");
        }
        mr = mr_;
    }

    Ok(())
}

fn update_qbittorrent(client: &mut Client, port: u16) -> Result<()> {
    client
        .post(Url::parse("http://127.0.0.1:8080/api/v2/app/setPreferences").unwrap())
        .form(&[("json", &format!(r#"{{"listen_port":{}}}"#, port))])
        .send()?
        .error_for_status()?;
    Ok(())
}

fn query_gateway(n: &mut Natpmp) -> Result<GatewayResponse> {
    let mut timeout = 250;
    while timeout <= 64000 {
        n.send_public_address_request()
            .map_err(|err| anyhow!("Fail with {:?}", err))?;
        println!(
            "Public address request sent! (will timeout in {}ms)",
            timeout
        );
        thread::sleep(Duration::from_millis(timeout));
        match n.read_response_or_retry() {
            Err(e) => match e {
                Error::NATPMP_TRYAGAIN => println!("Try again later"),
                _ => return Err(anyhow!("Try again: {:?}", e)),
            },
            Ok(Response::Gateway(gr)) => {
                println!(
                    "Got response: IP: {}, Epoch: {}",
                    gr.public_address(),
                    gr.epoch()
                );
                return Ok(gr);
            }
            _ => {
                bail!("Expecting a gateway response");
            }
        };
        timeout *= 2;
    }
    bail!("Quering gateway failed!");
}

fn query_available_port(n: &mut Natpmp) -> Result<MappingResponse> {
    return query_port(n, 0, 0, false);
}

fn query_port(
    n: &mut Natpmp,
    internal: u16,
    external: u16,
    check: bool,
) -> Result<MappingResponse> {
    let mut timeout = 250;
    while timeout <= 64000 {
        let _ = n.send_port_mapping_request(Protocol::TCP, 0, 0, 360)
            .map_err(|err| anyhow!("Fail with {:?}", err));
        println!("Port mapping request sent! (will timeout in {}ms)", timeout);

        // sleep for a while
        thread::sleep(Duration::from_millis(1000));
        match n.read_response_or_retry() {
            Err(e) => match e {
                Error::NATPMP_TRYAGAIN => println!("Try again later"),
                _ => return Err(anyhow!("Try again later: {:?}", e)),
            },
            Ok(Response::TCP(tr)) => {
                println!(
                    "Got response: Internal: {}, External: {}, Lifetime: {}s",
                    tr.private_port(),
                    tr.public_port(),
                    tr.lifetime().as_secs()
                );
                if (!check)
                    || (tr.private_port() == internal
                        && tr.public_port() == external
                        && tr.lifetime().as_secs() > 0)
                {
                    return Ok(tr);
                } else {
                    println!("Retring, port is not the one wanted!");
                }
            }
            _ => {
                bail!("Expecting a tcp response");
            }
        };
        timeout *= 2;
    }
    bail!("Mapping failed!");
}
