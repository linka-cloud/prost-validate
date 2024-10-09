//! `prost-validate-build` contains [`Builder`] to configure [`prost_build::Config`]
//! to derive [`prost_validate::Validator`] for all messages in protocol buffers.
//!
//! The simplest way to generate protocol buffer API:
//!
//! ```no_run
//! // build.rs
//! use prost_validate_build::Builder;
//!
//! Builder::new()
//!     .compile_protos(&["path/to/protobuf.proto"], &["path/to/include"])
//!     .expect("Failed to compile protos");
//! ```
mod rules;

use crate::rules::IntoFieldAttribute;
use prost_reflect::{DescriptorPool, OneofDescriptor};
use prost_validate_types::{FieldRulesExt, MessageRulesExt, OneofRulesExt};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{env, fs, io};

/// Configuration builder for prost-validate code generation.
///
/// ```no_run
/// # use prost_validate_build::Builder;
/// Builder::new()
///     .compile_protos(&["path/to/protobuf.proto"], &["path/to/include"])
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Builder {
    file_descriptor_set_path: PathBuf,
}

impl Default for Builder {
    fn default() -> Self {
        let file_descriptor_set_path = env::var_os("OUT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("file_descriptor_set.bin");

        Self {
            file_descriptor_set_path,
        }
    }
}

impl Builder {
    /// Create a new builder with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the path where the encoded file descriptor set is created.
    /// By default, it is created at `$OUT_DIR/file_descriptor_set.bin`.
    ///
    /// This overrides the path specified by
    /// [`prost_build::Config::file_descriptor_set_path`].
    pub fn file_descriptor_set_path<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.file_descriptor_set_path = path.into();
        self
    }

    /// Configure `config` to derive [`prost_validate::Validator`] for all messages included in `protos`.
    /// This method does not generate prost-validate compatible code,
    /// but `config` may be used later to compile protocol buffers independently of [`Builder`].
    /// `protos` and `includes` should be the same when [`prost_build::Config::compile_protos`] is called on `config`.
    ///
    /// ```ignore
    /// let mut config = Config::new();
    ///
    /// // Customize config here
    ///
    /// Builder::new()
    ///     .configure(&mut config, &["path/to/protobuf.proto"], &["path/to/include"])
    ///     .expect("Failed to configure for reflection");
    ///
    /// // Custom compilation process with `config`
    /// config.compile_protos(&["path/to/protobuf.proto"], &["path/to/includes"])
    ///     .expect("Failed to compile protocol buffers");
    /// ```
    pub fn configure(
        &mut self,
        config: &mut prost_build::Config,
        protos: &[impl AsRef<Path>],
        includes: &[impl AsRef<Path>],
    ) -> io::Result<()> {
        config
            .file_descriptor_set_path(&self.file_descriptor_set_path)
            .compile_protos(protos, includes)?;

        let buf = fs::read(&self.file_descriptor_set_path)?;
        let descriptor = DescriptorPool::decode(buf.as_ref()).expect("Invalid file descriptor");

        for message in descriptor.all_messages() {
            let full_name = message.full_name();
            config.type_attribute(full_name, "#[derive(::prost_validate::Validator)]");
            if message.validation_ignored() || message.validation_disabled() {
                continue;
            }
            let mut oneofs: HashMap<String, Rc<OneofDescriptor>> = HashMap::new();
            for field in message.fields() {
                config.field_attribute(
                    field.full_name(),
                    format!("#[validate(name = \"{}\")]", field.full_name()),
                );
                let field_rules = match field.validation_rules().unwrap() {
                    Some(r) => r,
                    None => continue,
                };
                if oneofs.contains_key(field.full_name()) {
                    continue;
                }
                if let Some(ref desc) = field.containing_oneof() {
                    config.field_attribute(
                        desc.full_name(),
                        format!("#[validate(name = \"{}\")]", desc.full_name()),
                    );
                    let desc = Rc::new(desc.clone());
                    config
                        .type_attribute(desc.full_name(), "#[derive(::prost_validate::Validator)]");
                    if desc.required() {
                        config.field_attribute(desc.full_name(), "#[validate(required)]");
                    }
                    for field in desc.fields() {
                        let field = field.clone();
                        config.field_attribute(
                            format!("{}.{}", desc.full_name(), field.name()),
                            format!("#[validate(name = \"{}\")]", field.full_name()),
                        );
                        oneofs.insert(field.full_name().to_string(), desc.clone());
                        let field_rules = match field.validation_rules().unwrap() {
                            Some(r) => r,
                            None => continue,
                        };
                        let field_attribute = field_rules.into_field_attribute();
                        if let Some(attribute) = field_attribute {
                            // this is not very protobuf typical, but it is the way it is implemented in prost-build
                            config.field_attribute(
                                format!("{}.{}", desc.full_name(), field.name()),
                                format!("#[validate({})]", attribute),
                            );
                        }
                    }
                    continue;
                }
                let field_attribute = field_rules.into_field_attribute();
                if let Some(attribute) = field_attribute {
                    config
                        .field_attribute(field.full_name(), format!("#[validate({})]", attribute));
                }
            }
        }

        Ok(())
    }

    /// Compile protocol buffers into Rust with given [`prost_build::Config`].
    pub fn compile_protos_with_config(
        &mut self,
        mut config: prost_build::Config,
        protos: &[impl AsRef<Path>],
        includes: &[impl AsRef<Path>],
    ) -> io::Result<()> {
        self.configure(&mut config, protos, includes)?;

        config.skip_protoc_run().compile_protos(protos, includes)
    }

    /// Compile protocol buffers into Rust.
    pub fn compile_protos(
        &mut self,
        protos: &[impl AsRef<Path>],
        includes: &[impl AsRef<Path>],
    ) -> io::Result<()> {
        self.compile_protos_with_config(prost_build::Config::new(), protos, includes)
    }
}
