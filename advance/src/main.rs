mod asyncs;
mod circle;
mod concurrency;
mod errors;
mod functional;
mod globalvar;
mod intotype;
mod lifetime;
mod macros;
mod point;
mod trust;

fn main() {
    asyncs::future::hello();
}
