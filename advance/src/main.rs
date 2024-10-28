mod asyncs;
mod circle;
mod coco;
mod concurrency;
mod errors;
mod functional;
mod globalvar;
mod intotype;
mod lifetime;
mod macros;
mod point;
mod tokios;
mod trust;

#[tokio::main]
async fn main() {
    tokios::futures::hello().await;
}
