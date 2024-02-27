#![feature(try_trait_v2)]
use notify::{Watcher, RecommendedWatcher, RecursiveMode};
use std::{clone, sync::mpsc::channel};
use futures::{StreamExt, TryStreamExt};
use kube::{{Client}, api::{Api, ResourceExt, ListParams, PostParams,WatchEvent}};
use kube::core::{Resource, CustomResourceExt};
use k8s_openapi::Metadata;
use k8s_openapi::api::{apps::v1::ReplicaSet, core::v1::{Namespace,Pod,Service}};
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use serde_json::{Result as SerdeResult, Value}; // use a different name for serde_json::Result
use notify::recommended_watcher;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::default::Default;
use serde_json::json;
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use core::fmt::Debug;
use serde::de::DeserializeOwned;
use std::ops::Try;
use std::fmt;
use tokio::sync::broadcast::error::SendError;
use std::marker::Send;
use tokio::sync::Mutex;
use futures::Stream; // Stellen Sie sicher, dass Sie den Stream-Trait importieren
use lazy_static::lazy_static;

lazy_static! {
    static ref RESOURCE_NAMES: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}
async fn add_resource_name(name: String, resource_names: &Arc<Mutex<Vec<String>>>) -> Result<(), Box<dyn Error>> {
    let mut names = resource_names.lock().await;
    names.push(name);
    Ok(())
}
slint::include_modules!();
#[tokio::main]
async fn ui_task() -> Result<(), Box<dyn Error + Send>> {
    let ui = AppWindow::new().map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
   
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() +   1);
        }
    });
    ui.run().map_err(|e| Box::new(e) as Box<dyn Error + Send>)
}

async fn create_watcher<T>( resource_type: &str) -> Result<(), Box<dyn Error + Send>>
where
    T: kube::Resource + Clone + DeserializeOwned + Debug + k8s_openapi::Metadata + 'static,
    <T as kube::Resource>::DynamicType: Default, // Fügen Sie diese Zeile hinzu
{    println!("starting {} stream", resource_type);
    let client: Client = Client::try_default().await.map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?;
    //let client = client.clone(); // Klonen Sie den Client innerhalb der Methode
    let api: Api<T> = Api::all(client);
    let lp = ListParams::default();
    let stream_result = api.watch(&lp, "0").await;
    let mut stream = stream_result.map_err(|e| Box::new(e) as Box<dyn Error + Send>)?.boxed();
    while let Ok(Some(event)) = stream.try_next().await {
        match event {
            WatchEvent::Added(ns) =>{
                let resource_name = format!("{} {} added! -n {}", resource_type, ns.name(), ns.namespace().unwrap_or_default());
                add_resource_name(resource_name, &RESOURCE_NAMES).unwrap();
                println!(" {} {} added! -n {}",resource_type, ns.name(),ns.namespace().unwrap_or_default())},
                
            WatchEvent::Modified(ns) => println!(" {} {} modified! -n {}",resource_type, ns.name(),ns.namespace().unwrap_or_default()),
            WatchEvent::Deleted(ns) => println!(" {} {} deleted! -n {}",resource_type, ns.name(),ns.namespace().unwrap_or_default()),
            _ => {}
        }
    }
    Ok(())
}

#[tokio::main]
async fn kube_properties() -> Result<(), Box<dyn Error + Send>> {
    let client = Client::try_default().await.map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?;

    let pod_watcher_handle = tokio::spawn(async move {create_watcher::<k8s_openapi::api::core::v1::Pod>( "pod").await});
    let service_watcher_handle = tokio::spawn(async move {create_watcher::<k8s_openapi::api::core::v1::Service>( "service").await});
    let deployment_watcher_handle = tokio::spawn(async move {create_watcher::<k8s_openapi::api::apps::v1::Deployment>( "deployment").await});
    let namespace_watcher_handle: tokio::task::JoinHandle<Result<(), Box<dyn Error + Send>>> = tokio::spawn(async move {create_watcher::<k8s_openapi::api::core::v1::Namespace>( "namespace").await});
    let replicaSet_watcher_handle: tokio::task::JoinHandle<Result<(), Box<dyn Error + Send>>> = tokio::spawn(async move {create_watcher::<k8s_openapi::api::core::v1::ReplicationController>( "replicationController").await});

    let handles = vec![pod_watcher_handle, service_watcher_handle,deployment_watcher_handle,namespace_watcher_handle,replicaSet_watcher_handle];
    for handle in handles {
        let result = handle.await.unwrap();
    }
  Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send>> {
    let (tx, _rx) = tokio::sync::broadcast::channel(100);
    // Clone the sender before moving it into the closures
    let tx_ui = tx.clone();
    let tx_kube = tx.clone();
    let ui_thread: thread::JoinHandle<Result<(), Box<dyn Error + Send>>> = thread::spawn(move || {
        ui_task().map_err(|e| {
            let error_message = e.to_string();
            tx_ui.send(error_message).unwrap(); // Use the cloned sender
            e // Return the original error
        })
    });

    let kube_thread: thread::JoinHandle<Result<(), Box<dyn Error + Send>>> = thread::spawn(move || {
        kube_properties().map_err(|e| {
            let error_message = e.to_string();
            tx_kube.send(error_message).unwrap(); // Use the cloned sender
            e // Return the original error
        })
    });
    kube_thread.join().unwrap();
    ui_thread.join().unwrap()
    let (tx, rx) = watch::channel(Vec::new());
    let watchable_resource_names = Arc::new(Mutex::new(rx));
    
}
