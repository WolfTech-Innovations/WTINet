use async_trait::async_trait;
use egui::{CentralPanel, Context, Label, Window};
use eframe::App;
use futures::prelude::*;
use libp2p::{
    core::{identity, muxing::StreamMuxerBox, transport::Boxed, upgrade},
    gossipsub::{self, Gossipsub, GossipsubConfig, GossipsubEvent, MessageAuthenticity, Topic},
    identify::{Identify, IdentifyConfig, IdentifyEvent},
    kad::{record::store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent},
    mdns::{Mdns, MdnsEvent},
    noise,
    swarm::{SwarmBuilder, Swarm, SwarmEvent},
    tcp::TcpConfig,
    yamux::YamuxConfig,
    PeerId, Transport,
};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

#[derive(Debug)]
enum NetworkEvent {
    Gossipsub(GossipsubEvent),
    Kademlia(KademliaEvent),
    Identify(IdentifyEvent),
    Mdns(MdnsEvent),
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "NetworkEvent")]
struct WTINetBehaviour {
    gossipsub: Gossipsub,
    kademlia: Kademlia<MemoryStore>,
    identify: Identify,
    mdns: Mdns,
}

struct NetworkGateway {
    peer_id: PeerId,
    swarm: Swarm<WTINetBehaviour>,
    known_entries: Arc<RwLock<HashMap<String, String>>>,
    html_display: Arc<RwLock<String>>,
    status_display: Arc<RwLock<String>>,
}

impl NetworkGateway {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let identity_keys = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(identity_keys.public());
        let noise_keys = noise::NoiseKeypair::<noise::X25519Spec>::new()
            .into_authentic(&identity_keys)?;

        let transport = TcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(YamuxConfig::default())
            .boxed();

        let gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(identity_keys.clone()),
            GossipsubConfig::default(),
        )?;
        let kademlia = Kademlia::new(peer_id.clone(), MemoryStore::new(peer_id.clone()));
        let identify = Identify::new(IdentifyConfig::new("WTINet/1.0".to_string(), peer_id.clone()));
        let mdns = Mdns::new(Default::default()).await?;

        let behaviour = WTINetBehaviour {
            gossipsub,
            kademlia,
            identify,
            mdns,
        };

        let swarm = SwarmBuilder::new(transport, behaviour, peer_id.clone())
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        Ok(Self {
            peer_id,
            swarm,
            known_entries: Arc::new(RwLock::new(HashMap::new())),
            html_display: Arc::new(RwLock::new(String::new())),
            status_display: Arc::new(RwLock::new("Disconnected".to_string())),
        })
    }

    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let topic = Topic::new("wtinet-html");
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        loop {
            match self.swarm.next().await {
                Some(SwarmEvent::Behaviour(event)) => match event {
                    NetworkEvent::Gossipsub(GossipsubEvent::Message { message, .. }) => {
                        let html_msg = String::from_utf8_lossy(&message.data).to_string();
                        *self.html_display.write().unwrap() = html_msg;
                    }
                    NetworkEvent::Kademlia(KademliaEvent::RoutingUpdated { peer, .. }) => {
                        *self.status_display.write().unwrap() = "Connected".to_string();
                    }
                    NetworkEvent::Identify(IdentifyEvent::Received { peer_id, info }) => {
                        *self.status_display.write().unwrap() =
                            format!("Connected to peer: {:?}", peer_id);
                    }
                    NetworkEvent::Mdns(MdnsEvent::Discovered(peers)) => {
                        for (peer_id, _) in peers {
                            println!("mDNS discovered peer: {:?}", peer_id);
                        }
                    }
                    _ => {}
                },
                Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) => {
                    *self.status_display.write().unwrap() =
                        format!("Connection established with {:?}", peer_id);
                }
                Some(SwarmEvent::ConnectionClosed { peer_id, cause, .. }) => {
                    if let Some(err) = cause {
                        *self.status_display.write().unwrap() =
                            format!("Connection closed with {:?} due to {:?}", peer_id, err);
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Default)]
struct WTINetApp {
    html_display: Arc<RwLock<String>>,
    status_display: Arc<RwLock<String>>,
}

impl App for WTINetApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("WTINet - Your Peer Status:");
            ui.label(self.status_display.read().unwrap().as_str());
            ui.separator();
            ui.label("HTML Displayed over WTINet:");
            ui.label(self.html_display.read().unwrap().as_str());
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut gateway = NetworkGateway::new().await?;

    tokio::spawn(async move {
        if let Err(e) = gateway.run().await {
            eprintln!("WTINet error: {}", e);
        }
    });

    let app = WTINetApp {
        html_display: gateway.html_display.clone(),
        status_display: gateway.status_display.clone(),
    };
    let options = eframe::NativeOptions::default();
    eframe::run_native("WTINet GUI", options, Box::new(|_cc| Box::new(app)));
    Ok(())
}
