use libp2p::{
    core::identity,
    gossipsub::{GossipConfig, Gossipsub, Topic, GossipsubEvent},
    NetworkBehaviour,
    PeerId,
    Swarm,
    Transport,
    Multiaddr,
    tcp::TcpConfig,
    yamux::YamuxConfig,
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use tokio::task;
use warp::Filter;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WtidLmsEntry {
    name: String,
    address: String, // This is a local address within WTINet
    timestamp: u64,
    dlm: String, // Device Letter Map identifier
}

#[derive(Serialize, Deserialize, Debug)]
enum WtidLmsMessage {
    Register(WtidLmsEntry),
    Query(String),
    Response(WtidLmsEntry),
    Lookup(String),
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    gossipsub: Gossipsub,
    known_entries: HashMap<String, WtidLmsEntry>,
}

impl Behaviour {
    fn new() -> Self {
        let gossip_config = GossipConfig::default();
        let gossipsub = Gossipsub::new(gossip_config);
        Self {
            gossipsub,
            known_entries: HashMap::new(),
        }
    }
    
    fn handle_message(&mut self, message: WtidLmsMessage) {
        match message {
            WtidLmsMessage::Register(mut entry) => {
                // Assign a DLM if not already assigned
                if entry.dlm.is_empty() {
                    entry.dlm = generate_dlm(); // Generate a DLM
                }
                // Store the entry locally
                self.known_entries.insert(entry.name.clone(), entry.clone());
                println!("Registered entry: {:?}", entry);
            }
            WtidLmsMessage::Lookup(name) => {
                // Check if we have the requested entry by name
                if let Some(entry) = self.known_entries.get(&name) {
                    println!("Found entry for {}: {:?}", name, entry);
                } else {
                    println!("Entry for {} not found", name);
                }
            }
            WtidLmsMessage::Response(entry) => {
                // Store the received entry if it's new
                self.known_entries.insert(entry.name.clone(), entry);
                println!("Received entry: {:?}", entry);
            }
            _ => {}
        }
    }
}

// Function to generate a unique DLM
fn generate_dlm() -> String {
    Uuid::new_v4().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate an identity keypair for the peer
    let identity_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(identity_keys.public());
    println!("Peer ID: {:?}", peer_id);

    // Create a transport for the peer-to-peer network
    let transport = TcpConfig::new()
        .upgrade(YamuxConfig::default())
        .boxed();

    // Initialize the behaviour and swarm
    let behaviour = Behaviour::new();
    let mut swarm = Swarm::new(transport, behaviour, peer_id.clone());

    // Start listening on a random port (not accessible via the internet)
    let listening_address: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(listening_address)?;

    println!("Listening on {:?}", listening_address);

    // Subscribe to the gossip topic
    let topic = Topic::new("wtidlm");
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    // Run the WTINet subsystem in the background
    let swarm = Arc::new(Mutex::new(swarm));
    let swarm_clone = swarm.clone();
    task::spawn(async move {
        loop {
            // Poll the swarm for events
            let mut swarm = swarm.lock().unwrap();
            match swarm.select_next_some().await {
                // Handle incoming Gossipsub messages
                GossipsubEvent::Message(_, _, data) => {
                    if let Ok(message) = serde_json::from_slice::<WtidLmsMessage>(&data) {
                        swarm.behaviour_mut().handle_message(message);
                    } else {
                        println!("Failed to deserialize message: {:?}", data);
                    }
                },
                _ => {}
            }
        }
    });

    // Create a warp filter for HTML displaying
    let lookup = warp::path!("MLMLS" / String)
        .map(move |name| {
            let swarm = swarm_clone.lock().unwrap();
            if let Some(entry) = swarm.behaviour().known_entries.get(&name) {
                // Return an HTML page displaying the entry details
                let html = format!(
                    "<html>
                    <head><title>Entry Details</title></head>
                    <body>
                        <h1>Entry Details</h1>
                        <p><strong>Name:</strong> {}</p>
                        <p><strong>Address:</strong> {}</p>
                        <p><strong>Timestamp:</strong> {}</p>
                        <p><strong>DLM:</strong> {}</p>
                    </body>
                    </html>",
                    entry.name, entry.address, entry.timestamp, entry.dlm
                );
                warp::reply::html(html)
            } else {
                warp::reply::html("<h1>Entry not found</h1>".to_string())
            }
        });

    // Create a warp filter for HTTPOWTIN
    let httpowtin = warp::path!("HTTPOWTIN" / String)
        .map(move |name| {
            let swarm = swarm_clone.lock().unwrap();
            if let Some(entry) = swarm.behaviour().known_entries.get(&name) {
                // Return an HTML page displaying the entry details via HTTPOWTIN
                let html = format!(
                    "<html>
                    <head><title>HTTPOWTIN - Entry Details</title></head>
                    <body>
                        <h1>HTTPOWTIN - Entry Details</h1>
                        <p><strong>Name:</strong> {}</p>
                        <p><strong>Address:</strong> {}</p>
                        <p><strong>Timestamp:</strong> {}</p>
                        <p><strong>DLM:</strong> {}</p>
                    </body>
                    </html>",
                    entry.name, entry.address, entry.timestamp, entry.dlm
                );
                warp::reply::html(html)
            } else {
                warp::reply::html("<h1>HTTPOWTIN - Entry not found</h1>".to_string())
            }
        });

    // Start the HTTP server to handle both MLMLS and HTTPOWTIN requests
    warp::serve(lookup.or(httpowtin)).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
