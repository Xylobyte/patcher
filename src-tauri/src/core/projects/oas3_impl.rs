use oas3::spec::{Info, Server};
use oas3::OpenApiV3Spec;
use serde_json::Value;
use std::collections::BTreeMap;
use time::OffsetDateTime;

pub trait OpenApiV3SpecExt {
    fn new(p_version: String, title: String, version: String, server: String) -> Self;
}

impl OpenApiV3SpecExt for OpenApiV3Spec {
    fn new(p_version: String, title: String, version: String, server: String) -> Self {
        let mut info_ext = BTreeMap::new();
        let now = OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap();
        info_ext.insert(
            "last-modified".to_string(),
            Value::String(now.to_string()),
        );

        let mut oas_ext = BTreeMap::new();
        oas_ext.insert("patcher".to_string(), Value::Bool(true));
        oas_ext.insert("patcher-version".to_string(), Value::String(p_version));

        OpenApiV3Spec {
            openapi: "3.1.0".to_string(),
            info: Info {
                title,
                summary: None,
                description: None,
                terms_of_service: None,
                version,
                contact: None,
                license: None,
                extensions: info_ext,
            },
            servers: vec![Server {
                url: server,
                description: None,
                variables: Default::default(),
            }],
            paths: None,
            components: None,
            tags: Default::default(),
            webhooks: Default::default(),
            external_docs: None,
            extensions: oas_ext,
        }
    }
}
