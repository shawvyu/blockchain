use std::str::from_utf8;

use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    kad::{store::MemoryStore, Kademlia, KademliaEvent, PeerRecord, QueryResult, Record, record::Key, Quorum},
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent},
    NetworkBehaviour, PeerId, identity,
};
use tokio::io::{self,AsyncBufReadExt};

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct MyBehaviour {
    kademlia: Kademlia<MemoryStore>,
    mdns: Mdns,
}

impl MyBehaviour {
    async fn new(peer_id: PeerId) -> Result<Self> {
        let store = MemoryStore::new(peer_id);
        let kademlia = Kademlia::new(peer_id, store);
        Ok(Self {
            kademlia,
            mdns: Mdns::new(Default::default()).await?,
        })
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                self.kademlia.add_address(&peer_id, multiaddr);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<KademliaEvent> for MyBehaviour {
    fn inject_event(&mut self, event: KademliaEvent) {
        if let KademliaEvent::OutboundQueryCompleted {  result,.. } = event {
            match result {
                QueryResult::GetProviders(Ok(ok)) => {
                    for peer in ok.providers {
                        println!("节点{:?}提供了key {:?}", peer, from_utf8(ok.key.as_ref()));
                    }
                }
                QueryResult::GetProviders(Err(err)) => {
                    eprintln!("Failed to get providers:{:?}", err);
                }
                QueryResult::GetRecord(Ok(ok)) => {
                    for PeerRecord {
                        record: Record { key, value, .. },
                        ..
                    } in ok.records
                    {
                        println!(
                            "获取存储记录 {:?} {:?}",
                            from_utf8(key.as_ref()).unwrap(),
                            from_utf8(&value.as_ref()).unwrap()
                        );
                    }
                }
                QueryResult::GetRecord(Err(err)) => {
                    eprintln!("Failed to get record:{:?}", err);
                }
                QueryResult::PutRecord(Ok(ok)) => {
                    println!("成功存储记录 {:?}", from_utf8(ok.key.as_ref()).unwrap());
                }
                QueryResult::PutRecord(Err(err)) => {
                    eprintln!("Failed to put record: {:?}", err);
                }
                QueryResult::StartProviding(Ok(ok)) => {
                    println!(
                        "成功存储记录提供者 {:?}",
                        from_utf8(ok.key.as_ref()).unwrap()
                    );
                }
                QueryResult::StartProviding(Err(err)) => {
                    eprintln!("Failed to put provider record: {:?}", err);
                }
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main()->Result<()> {
    let key_pair=identity::Keypair::generate_ed25519();

    let peer_id=PeerId::from(key_pair.public());
    println!("节点ID：{:?}",peer_id);

    let transport=libp2p::development_transport(key_pair).await?;

    let mut swarm={
        let behaviour=MyBehaviour::new(peer_id).await?;

        SwarmBuilder::new(transport, behaviour, peer_id)
            .executor(Box::new(|fut|{
                tokio::spawn(fut);
            }))
            .build()
    };

    let mut stdin=io::BufReader::new(io::stdin()).lines();
    
    swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;

    loop {
        tokio::select! {
            line = stdin.next_line()=>{
                let line = line?.expect("stdin close");
                handle_input_line(&mut swarm.behaviour_mut().kademlia, line);
            }
            event = swarm.select_next_some()=>{
                if let SwarmEvent::NewListenAddr{address,..}=event{
                    println!("本地监听地址:{address}");
                }
            }
        }
    }
}


fn handle_input_line(kademlia:&mut Kademlia<MemoryStore>,line:String){
    let mut args=line.split(" ");
    match args.next() {
        Some("GET")=>{
          let key={
            match args.next() {
                Some(key)=>Key::new(&key),
                None=>{
                    eprintln!("Expected key");
                    return ;
                }
            }
          };
          kademlia.get_record(key, Quorum::One);
        }
        Some("GET_PROVIDERS")=>{
            let key={
                match args.next() {
                    Some(key)=>Key::new(&key),
                    None=>{
                        eprintln!("Expected key providers");
                        return ;
                    }
                }
            };
            kademlia.get_providers(key);
        }
        Some("PUT")=>{
            let key={
                match args.next() {
                    Some(key)=>Key::new(&key),
                    None=>{
                        eprintln!("put Expected key");
                        return ;
                    }
                }
            };
            let value={
                match args.next() {
                    Some(value)=>value.as_bytes().to_vec(),
                    None=>{
                        eprintln!("put Expected value");
                        return ;
                    }
                }
            };
            let record=Record{
                key,
                value,
                publisher:None,
                expires:None,
            };
            kademlia.put_record(record, Quorum::One)
                .expect("Failed to store record locally.");

        }
        Some("PUT_PROVIDER")=>{
            let key={
                match args.next() {
                    Some(key)=>Key::new(&key),
                    None=>{
                        eprintln!("put_provider Expected key");
                        return ;
                    }
                }
            };
            kademlia.start_providing(key)
                .expect("Failed to start providing key");
        }
        _=>{
            eprintln!("expected GET,GET_PROVIDERS,PUT or PUT_PROVIDER");
        }
    }
}