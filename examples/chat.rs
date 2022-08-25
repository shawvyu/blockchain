use anyhow::{Ok, Result};
use futures::StreamExt;
use libp2p::{
    core::transport::upgrade,
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsEvent},
    noise,
    swarm::{NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent},
    tcp::{GenTcpConfig, TokioTcpTransport},
    yamux, Multiaddr, NetworkBehaviour, PeerId, Transport,
};
use tokio::io::{self, AsyncBufReadExt};

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct MyBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
}

impl MyBehaviour {
    async fn new(id: PeerId) -> Result<Self> {
        Ok(Self {
            floodsub: Floodsub::new(id),
            mdns: Mdns::new(Default::default()).await?,
        })
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for MyBehaviour {
    fn inject_event(&mut self, message: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = message {
            println!(
                "收到消息：'{:?}' 来自 {:?}",
                String::from_utf8_lossy(&message.data),
                message.source
            );
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                    println!("在网络中加入节点:{peer}");
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                        println!("从网络中删除节点:{peer}");
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let id_keys = identity::Keypair::generate_ed25519();

    let peer_id = PeerId::from(id_keys.public());
    println!("节点ID:{peer_id}");

    let noise_keys = noise::Keypair::<noise::X25519Spec>::new().into_authentic(&id_keys)?;

    let transport = TokioTcpTransport::new(GenTcpConfig::default().nodelay(true))
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    let floodsub_topic = floodsub::Topic::new("chat");

    let mut swarm = {
        let mut behaviour = MyBehaviour::new(peer_id).await?;

        behaviour.floodsub.subscribe(floodsub_topic.clone());

        SwarmBuilder::new(transport, behaviour, peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build()
    };

    if let Some(to_dial) = std::env::args().nth(1) {
        let addr: Multiaddr = to_dial.parse()?;
        swarm.dial(addr)?;
        println!("链接远程节点：{to_dial}");
    }

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;

    loop {
        tokio::select! {
            line = stdin.next_line()=>{
                let line=line?.expect("stdin closed");
                swarm.behaviour_mut().floodsub.publish(floodsub_topic.clone(), line.as_bytes());
            }
            event=swarm.select_next_some()=>{
                if let SwarmEvent::NewListenAddr{
                    address,..
                }=event {
                    println!("本地监听地址：{address}");
                }
            }
        }
    }
}
