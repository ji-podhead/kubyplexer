#![feature(try_trait_v2)]
use notify::{Watcher, RecommendedWatcher, RecursiveMode};
use std::sync::mpsc::channel;
use futures::{StreamExt, TryStreamExt};
use kube::{Client, api::{Api, ResourceExt, ListParams, PostParams,WatchEvent},api::watch::{WatchEvent, WatchStream}};
use k8s_openapi::api::core::v1::{Namespace,Pod,Service};
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::{Resource,Metadata};
use serde_json::{Result as SerdeResult, Value}; // use a different name for serde_json::Result
use notify::recommended_watcher;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::Read;
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
async fn create_watcher<kube: Resource + Clone + k8s_openapi::Metadata + DeserializeOwned + Debug + 'static>(client: Client, resource_type: &str) -> Result<WatchStream<T>, Box<dyn Error + Send>> {   
     let api: Api<T> = Api::all(client);
    let lp = ListParams::default();
    let stream_result = api.watch(&lp, "0").await;
    let mut stream = stream_result.map_err(|e| Box::new(e) as Box<dyn Error + Send>)?.boxed();
while let Ok(Some(event)) = stream.try_next().await {
    match event {
        WatchEvent::Added(ns) => println!("resource added: {}"),
        WatchEvent::Modified(ns) => println!("resource modified: {}"),
        WatchEvent::Deleted(ns) => println!("resource deleted: {}"),
        _ => {}
    }
}
}
#[tokio::main]
async fn kube_properties() -> Result<(), Box<dyn Error>> {
    let mut namespaces_vec = Vec::new();
    let client_result = Client::try_default().await.map_err(|e| e.into());
    let client = client_result?;
    let pod_stream: <Result<_, Box<dyn Error>> as Try>::Output = create_watcher::<k8s_openapi::api::core::v1::Pod>(client.clone(), "pods").await?;
    let service_stream: <Result<_, Box<dyn Error>> as Try>::Output = create_watcher::<k8s_openapi::api::core::v1::Pod>(client.clone(), "services").await?;
    let deployment_stream: <Result<_, Box<dyn Error>> as Try>::Output = create_watcher::<k8s_openapi::api::apps::v1::Deployment,>(client.clone(), "deployments").await?;
    let namespace_stream: <Result<_, Box<dyn Error>> as Try>::Output = create_watcher::<k8s_openapi::api::core::v1::Namespace>(client.clone(), "namespaces").await?;
    let replicaset_stream: <Result<_, Box<dyn Error>> as Try>::Output = create_watcher::<k8s_openapi::api::apps::v1::ReplicaSet>(client.clone(), "replicasets").await?;
    //let deployment_stream: <Result<_, Box<dyn Error>> as Try>::Output = create_watcher::<k8s_openapi::api::apps::v1::Deployment, ObjectMeta>(client.clone(), "deployments").await?;
Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send>> {
    let (tx, _rx) = tokio::sync::broadcast::channel(100);

    let ui_thread: thread::JoinHandle<Result<(), Box<dyn Error + Send>>> = thread::spawn(move || {
        ui_task().map_err(|e| {
            let error_message = e.to_string();
            tx.send(error_message).unwrap(); // Senden Sie die Fehlermeldung über den Sender
            e // Geben Sie den ursprünglichen Fehler zurück
        })
    });
    
    let kube_thread: thread::JoinHandle<Result<(), Box<dyn Error + Send>>> = thread::spawn(move || {
        kube_properties().map_err(|e| {
            let error_message = e.to_string();
            tx.send(error_message).unwrap(); // Senden Sie die Fehlermeldung über den Sender
            e // Geben Sie den ursprünglichen Fehler zurück
        })
    });
        kube_thread.join().unwrap();
    ui_thread.join().unwrap()
}