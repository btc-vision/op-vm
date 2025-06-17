use std::collections::VecDeque;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

pub struct RuntimePool {
    runtimes: Mutex<VecDeque<Arc<Runtime>>>,
    idling: Mutex<Arc<AtomicUsize>>,
    expected_size: Mutex<Arc<AtomicUsize>>,
}

impl RuntimePool {
    pub fn new(size: usize) -> Self {
        let mut runtimes = VecDeque::with_capacity(size);
        for _ in 0..size {
            let runtime = Self::create_runtime();
            runtimes.push_back(runtime);
        }
        Self {
            runtimes: Mutex::new(runtimes),
            idling: Mutex::new(Arc::new(AtomicUsize::new(0))),
            expected_size: Mutex::new(Arc::new(AtomicUsize::new(size))),
        }
    }

    /*pub fn destroy(&self) {
        let mut runtimes = self.runtimes.lock().unwrap();
        for mut runtime in runtimes.iter() {
            runtime.shutdown_background();
        }
    }*/

    /*pub fn increase(&self, size: usize) {
        let l = self
            .expected_size
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock expected_size"))
            .unwrap();
        l.fetch_add(size, Relaxed);

        let mut runtimes = self.runtimes.lock().unwrap();
        for _ in 0..size {
            let runtime = Runtime::new().expect("Failed to create Tokio Runtime");
            runtimes.push_back(Arc::new(runtime));
        }
    }*/

    /*pub fn decrease(&self, size: usize) {
        let mut runtimes = self.runtimes.lock().unwrap();
        for _ in 0..size {
            runtimes.pop_back();
        }
    }

    pub fn resize(&self, size: usize) {
        let mut runtimes = self.runtimes.lock().unwrap();
        let current_size = runtimes.len();
        if size > current_size {
            for _ in 0..(size - current_size) {
                let runtime = Runtime::new().expect("Failed to create Tokio Runtime");
                runtimes.push_back(Arc::new(runtime));
            }
        } else if size < current_size {
            for _ in 0..(current_size - size) {
                runtimes.pop_back();
            }
        }
    }*/

    fn create_runtime() -> Arc<Runtime> {
        Arc::new(Runtime::new().expect("Failed to create Tokio Runtime"))
    }

    pub fn get_runtime(&self) -> Option<Arc<Runtime>> {
        let idling = self
            .idling
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock idling"))
            .unwrap();
        if idling.load(Relaxed) == 0 {
            return Some(Self::create_runtime());
        }

        idling.fetch_sub(1, Relaxed);

        let mut runtimes = self
            .runtimes
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock runtimes"))
            .unwrap();
        runtimes.pop_front()
    }

    pub fn return_runtime(&self, runtime: Arc<Runtime>) -> anyhow::Result<()> {
        let idling = self
            .idling
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock idling"))?;
        let expected_size = self
            .expected_size
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock expected_size"))?;

        if idling.load(Relaxed) > expected_size.load(Relaxed) {
            //println!("RuntimePool: Destroying runtime. idling: {}, expected_size: {}", idling.load(Relaxed), expected_size.load(Relaxed));

            return Ok(());
        }

        idling.fetch_add(1, Relaxed);

        //println!("RuntimePool: Returning runtime. idling: {}, expected_size: {}", idling.load(Relaxed), expected_size.load(Relaxed));

        let mut runtimes = self
            .runtimes
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock runtimes"))?;
        Ok(runtimes.push_back(runtime))
    }
}
