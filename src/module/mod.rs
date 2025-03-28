#![feature(edition2021)]
macro_rules! impl_installable_module {
    ($struct:ty, $install:expr, $test:expr) => {
        impl InstallableModule for $struct {
            fn install(&self, config: &BitcoinConfig) -> Result<()> {
                $install(self, config)
            }
            fn test(&self) -> Result<HashMap<String, bool>> {
                $test(self)
            }
            fn activate(&self) -> Result<()> { Ok(()) }
            fn deactivate(&self) -> Result<()> { Ok(()) }
        }
    };
}

impl_installable_module!(
    LightningModule,
    |module, config| { /* install logic */ },
    |module| { Ok(hashmap!{/* test results */}) }
); 