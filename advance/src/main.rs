mod concurrency;
mod functional;
mod intotype;
mod lifetime;
mod point;

fn main() {
    concurrency::atomics::hello();
}
