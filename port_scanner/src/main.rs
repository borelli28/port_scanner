use dioxus::prelude::*;
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::time::Duration;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        PortScanner {}
    }
}

#[component]
fn PortScanner() -> Element {
    let mut ip = use_signal(|| String::from("127.0.0.1"));
    let mut port_range = use_signal(|| String::from("1-100"));
    let mut result = use_signal(|| (String::new(), String::new(), String::new()));

    let scan = move |_| {
        let ip_addr: IpAddr = ip.read().parse().unwrap_or(IpAddr::V4("127.0.0.1".parse().unwrap()));
        let ports = parse_port_range(&port_range.read()).unwrap_or(vec![80]);
        let scan_result = scanner(ip_addr, &ports);
        result.set(scan_result);
    };

    rsx! {
        div { class: "container",
            div { class: "input-container",
                input { 
                    value: "{ip}", 
                    oninput: move |evt| ip.set(evt.value().clone()) 
                }
                input { 
                    value: "{port_range}", 
                    oninput: move |evt| port_range.set(evt.value().clone()) 
                }
                button { onclick: scan, "Scan Ports" }
            }
            div { class: "results-container",
                span { class: "open", "Open: " }
                p { class: "ports-text", "{result.read().0}" }
                span { class: "closed", "Closed: " }
                p { class: "ports-text", "{result.read().1}" }
                span { class: "filtered", "Filtered: " }
                p { class: "ports-text", "{result.read().2}" }
            }
        }
    }
}

fn parse_port_range(range: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    if range.contains('-') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 { return Err("Invalid range".into()); }
        let start: u16 = parts[0].parse()?;
        let end: u16 = parts[1].parse()?;
        Ok((start..=end).collect())
    } else {
        let port: u16 = range.parse()?;
        Ok(vec![port])
    }
}

fn scanner(ip: IpAddr, ports: &[u16]) -> (String, String, String) {
    let mut open = Vec::new();
    let mut closed = Vec::new();
    let mut filtered = Vec::new();

    for &port in ports {
        let socket = SocketAddr::new(ip, port);
        match TcpStream::connect_timeout(&socket, Duration::from_secs(3)) {
            Ok(_) => open.push(port),
            Err(err) => {
                if err.kind() == std::io::ErrorKind::ConnectionRefused {
                    closed.push(port);
                } else {
                    filtered.push(port);
                }
            }
        }
    }

    (
        open.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "),
        closed.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "),
        filtered.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
    )
}
