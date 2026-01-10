# ADR-3-3: ODRL Policy Templates & Instantiation Engine

**Date:** January 08, 2026  
**Status:** In Progress  
**Context:** Policy Management / User Experience

## Context and Problem Statement

Creating valid ODRL policies manually is an error-prone process due to the complexity and verbosity of the ODRL JSON-LD specification. Users often struggle with syntax and logic errors. To improve usability and compliance, we required a system to define "Policy Templates"—pre-validated structures that can be loaded at boot time and instantiated by users simply by providing a map of parameter values.

## Decision

We have decided to implement a **Policy Template System** centered around a rich data model. A `PolicyTemplate` structure will include metadata (title, author, version), the raw ODRL content containing placeholders (e.g., `$date_time`), and strict definitions for those parameters, including data types (String, Integer, Date) and validation restrictions (Regex, Min/Max values).

**Template Data Model:**

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")] 
pub struct PolicyTemplate {
    pub id: String,     
    pub version: String, 
    pub content: serde_json::Value, // Raw ODRL with $placeholders
    pub parameters: HashMap<String, ParameterDefinition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterDefinition {
    pub data_type: ParameterDataType, // String, Integer, Date, Selection...
    pub restrictions: ValidationRestrictions, // Regex, Range, AllowedValues...
    pub ui: UiHints, // Labels, Descriptions, Placeholders for UI generation
}
```

The core of this system is the Instantiation Engine. This service exposes an endpoint that accepts a Template ID and a map of parameters. It performs rigorous semantic validation, ensuring provided values match the defined data types and restrictions. Upon validation, it substitutes the placeholders in the ODRL content to generate a valid OdrlOffer.

## Traceability and Persistence

To ensure auditability, we have modified the database schema (SeaORM) to link every generated policy back to its source template. This allows us to track which template version was used to generate any given contract offer.

```rust
// SeaORM Migration Snippet
Table::create()
    .table(CatalogODRLOffers::Table)
    .col(ColumnDef::new(CatalogODRLOffers::Id).string().primary_key())
    // Tracking columns
    .col(ColumnDef::new(CatalogODRLOffers::SourceTemplateId).string().null())
    .col(ColumnDef::new(CatalogODRLOffers::SourceTemplateVersion).string().null())
    .col(ColumnDef::new(CatalogODRLOffers::InstantiationParameters).json_binary().null())
    // Foreign Key to ensure integrity
    .foreign_key(
        ForeignKey::create()
            .from(CatalogODRLOffers::Table, (CatalogODRLOffers::SourceTemplateId, CatalogODRLOffers::SourceTemplateVersion))
            .to(Alias::new("policy_templates"), (Alias::new("id"), Alias::new("version")))
            .on_delete(ForeignKeyAction::SetNull)
    )
    .to_owned()
```

Additionally, the Bootloader (defined in ADR-004) has been updated to scan a configured directory for template files upon startup, automatically upserting them into the database. This ensures that the available templates always match the deployment configuration.