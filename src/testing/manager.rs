pub struct TestManager {
    common: HashMap<String, Box<dyn Fn() -> Result<bool>>>,
    modules: HashMap<String, Box<dyn Fn() -> Result<HashMap<String, bool>>>>,
}

impl TestManager {
    pub fn new() -> Self {
        let mut common = HashMap::new();
        common.insert("rng".into(), Box::new(test_rng));
        common.insert("constant_time".into(), Box::new(test_constant_time));
        Self { common, modules: HashMap::new() }
    }

    pub fn run_common(&self) -> Result<HashMap<String, bool>> {
        self.common.iter()
            .map(|(k, v)| v().map(|res| (k.clone(), res)))
            .collect()
    }
} 