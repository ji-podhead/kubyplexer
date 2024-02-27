use notify::{Watcher, RecommendedWatcher, RecursiveMode};
use std::sync::mpsc::channel;
use futures::{StreamExt, TryStreamExt};
use kube::{Client, api::{Api, ResourceExt, ListParams, PostParams,WatchEvent}};
use k8s_openapi::api::core::v1::{Namespace,Pod};
use serde_json::{Result as SerdeResult, Value}; // use a different name for serde_json::Result
use notify::recommended_watcher;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use serde_json::json;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
    //let temp_file_path = "./namespaces.txt";
    //let mut file = File::create(temp_file_path);
    //let client = Client::try_default().await;


fn print_file_contents(path: &str) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
fn get_pod_proto() -> serde_json::Value {
    json!({
        "name": [],
        "uid": [],
    })
}
fn get_service_proto() -> serde_json::Value {
    json!({
        "name": [],
        "port": [],
    })
}
fn get_namespace_proto() -> serde_json::Value {
    json!({
        "pods": [],
        "services": [],
        "cronjobs":[]
    })
}
struct SharedState {
    counter: u32,
}

// Eine Funktion, die den Zustand liest
async fn read_state(state: Arc<RwLock<SharedState>>) {
    let state = state.read().unwrap(); // Lesen des Zustands
    println!("Counter: {}", state.counter);
}

// Eine Funktion, die den Zustand  ändert
async fn modify_state(state: Arc<RwLock<SharedState>>) {
    let mut state = state.write().unwrap(); // Schreiben des Zustands
    state.counter +=  1;
    println!("Counter incremented to: {}", state.counter);
}

// A function to get the namespaces from the Kubernetes API
async fn get_namespaces() -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;

    // Read namespaces into the typed interface from k8s-openapi
    let namespaces: Api<k8s_openapi::api::core::v1::Namespace> = Api::all(client);
    let mut result = Vec::new();
    for ns in namespaces.list(&ListParams::default()).await? {
        // Get the name of each namespace
        result.push(ns.name());
    }
    Ok(result)
}

fn jsoncreate() -> std::io::Result<()> {
    let temp_file_path = "./namespaces.txt";
    let file = File::create(temp_file_path)?; // Use the ? operator to handle the Result
    let data = json!({
        "allNamespaces":[],
        "namespaces":{}
    });
    let mut writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &data)?; // Use the ? operator to handle the Result
    Ok(())
}


use kube::{Client, api::{Api, ListParams}, api::Resource, api::watch::{WatchEvent, WatchStream}};
use std::error::Error;

async fn create_watcher<T: Resource + Clone + 'static>(client: Client, resource_type: &str) -> Result<WatchStream<T>, Box<dyn Error>> {
    let api: Api<T> = Api::all(client);
    let lp = ListParams::default();
    let stream_result = api.watch(&lp, "0").await;
    match stream_result {
        Ok(stream) => Ok(stream.boxed()),
        Err(e) => Err(kube_to_common_error(e)),
    }
}

fn kube_to_common_error(e: kube::Error) -> Box<dyn Error> {
    // Implementieren Sie hier die Konvertierung Ihrer kube::Error in Ihren allgemeinen Error-Typ
    Box::new(e)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_result = Client::try_default().await;
    let client = match client_result {
        Ok(client) => client,
        Err(e) => return Err(kube_to_common_error(e)),
    };

    let pod_stream = create_watcher::<kube::api::Pod>(client.clone(), "pods").await?;
    let deployment_stream = create_watcher::<kube::api::apps::Deployment>(client.clone(), "deployments").await?;
    let service_stream = create_watcher::<kube::api::Service>(client.clone(), "services").await?;

    // Verwenden Sie die Streams wie zuvor
    // ...

    Ok(())
}
async fn kube_properties() -> Result<(), CommonError> {
    let mut namespaces_vec = Vec::new();
    let temp_file_path = "./namespaces.txt";
    let mut file = File::create(temp_file_path);
    let client = Client::try_default().await;
    let client_result = Client::try_default().await;
    let client = match client_result {
        Ok(client) => client,
        Err(e) => return Err(kube_to_common_error(e)),
    };
    let namespaces: Api<Namespace> = Api::all(client);
    let lp = ListParams::default();

    let stream_result = namespaces.watch(&lp, "0").await;
    let mut stream = match stream_result {
        Ok(stream) => stream.boxed(),
        Err(e) => return Err(kube_to_common_error(e)),
    };
while let Ok(Some(event)) = stream.try_next().await {
    match event {
        WatchEvent::Added(ns) => { namespaces_vec.push(ns.name()); },
        WatchEvent::Modified(ns) => println!("Namespace modified: {}", ns.name()),
        WatchEvent::Deleted(ns) => println!("Namespace deleted: {}", ns.name()),
        _ => {}
    }
}