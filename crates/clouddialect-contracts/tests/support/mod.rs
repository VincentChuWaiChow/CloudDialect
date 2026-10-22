//! In-memory pinned-engine controls, not a production catalog or isolation proof.

use std::{collections::BTreeMap, error::Error, io};

use boon::{CompileError, Compiler, SchemaIndex, Schemas, UrlLoader};
use serde_json::Value;

struct MemoryLoader {
    resources: BTreeMap<String, Vec<u8>>,
}

impl UrlLoader for MemoryLoader {
    fn load(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        let bytes = self.resources.get(url).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("test resources deny URI {url}"),
            )
        })?;
        Ok(serde_json::from_slice(bytes)?)
    }
}

pub struct SchemaHarness {
    schemas: Schemas,
    root: SchemaIndex,
}

impl SchemaHarness {
    pub fn new(
        resources: BTreeMap<String, Vec<u8>>,
        root: &str,
        assert_formats: bool,
    ) -> Result<Self, Box<CompileError>> {
        let mut compiler = Compiler::new();
        compiler.use_loader(Box::new(MemoryLoader { resources }));
        if assert_formats {
            compiler.enable_format_assertions();
        }
        let mut schemas = Schemas::new();
        let root = compiler.compile(root, &mut schemas)?;
        Ok(Self { schemas, root })
    }

    /// False means a schema mismatch; Err means the instance bytes are not JSON.
    pub fn validate_json(&self, bytes: &[u8]) -> Result<bool, serde_json::Error> {
        let instance = serde_json::from_slice(bytes)?;
        Ok(self.schemas.validate(&instance, self.root).is_ok())
    }
}
