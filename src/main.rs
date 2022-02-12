use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Config {
    inner: Arc<Mutex<ConfigInner>>,
}

#[derive(Clone)]
struct ConfigFile {
    name: String,
    age: i32,
}

struct ConfigInner {
    cf: ConfigFile,
}

impl Config {
    fn new(name: &str, age: i32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ConfigInner {
                cf: ConfigFile {
                    name: String::from(name), age
                }
            }))
        }
    }

    fn get_cf(&self) -> ConfigFile {
        let lock = self.inner.lock().unwrap();
        lock.cf.clone()
    }

    pub fn with_cf<F, T>(&self, func: F) -> T
    where
       F: FnOnce(&mut ConfigFile) -> T
    {
        let mut lock = self.inner.lock().unwrap();
        func(&mut lock.cf)
    }
}

fn main() {
    let cfg = Config::new("Theo", 7);

    println!("name = {}", cfg.get_cf().name);
    println!("age = {}", cfg.get_cf().age);

    cfg.with_cf( | cf | {
                cf.age += 2;
            });

    println!("name = {}", cfg.get_cf().name);
    println!("age = {}", cfg.get_cf().age);

    let name = cfg.with_cf( |cf| { cf.name.to_owned() });
    let age = cfg.with_cf( |cf| { cf.age });

    println!("name = {}", name);
    println!("age = {}", age);
}
