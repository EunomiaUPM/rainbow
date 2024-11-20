# Rainbow 🌈🌈<br>Dataspace Protocol Implementation

![Rainbow front](./docs/static/img/rainbow.png)

## What is Rainbow

Rainbow or also known as Dataspace Rainbow is an implementation
of [Dataspace Protocol 2024-1](https://docs.internationaldataspaces.org/ids-knowledgebase/dataspace-protocol) promoted
by IDSA (International Data Spaces Association).

This implementation has been made by the GING (Next Generation Internet Group) research group. GING is part of the DIT (
Department of Telematics Engineering) of the Universidad Politécnica de Madrid.

### What are dataspaces?

Dataspaces are services that allow the sharing of data, or the subscription to data services between entities in an
interoperable way and with a decentralized identity. Data spaces need different building blocks for their development,
ranging from self-sovereign identity systems, through transfer negotiation protocols, contracts, catalogs, through
policy enforcement systems. All this in order to generate the digital trust and security necessary for data sharing and
to generate value and a real data economy.

For more information, we recommend reading
the [Technical Convergence of Dataspaces](https://data-spaces-business-alliance.eu/wp-content/uploads/dlm_uploads/Data-Spaces-Business-Alliance-Technical-Convergence-V2.pdf).

The Dataspace protocol is an initiative to specify the negotiation of transfer, catalog and contracts between consumers
and data providers in a decentralized ecosystem in an interoperable manner.

For more information we recommend reading
the [Dataspace Protocol abstract](https://docs.internationaldataspaces.org/ids-knowledgebase/dataspace-protocol).

Rainbow intends to cover the implementation of that specification in its entirety, with some modifications and
enhancements, as well as an in-house implementation of the transfer data plane system for high efficiency scenarios such
as big data or machine learning.

### **Feature highlights**

- Written in Rust from scratch. Asynchronously based on Tokio runtime.
- HTTP APIs with Axum, SeaORM, Postgres. Future integration with gRPC.
- OpenApi integration with Utoipa-axum.
- Serde-based serialization, elegant error handling with thiserror and anyhow.
- Really low memory footprint and blazingly fast.

### Organization in crates

- Rainbow-catalog: Catalog system compatible with DCAT3, and based on the Catalog Protocol of the Dataspace Protocol.
- Rainbow-transfer: Complete implementation of the transfer system and control plane of the Transfer Process Protocol of
  the Dataspace Protocol. Specification and implementation of the data plane.
- Rainbow-data-plane: (Work in progress). Own implementation for big-data and multi-implementation environments based on
  massive transfer tools. Future integration with Databricks' Deltasharing protocol, and with Apache Arrow's Flight.
- Rainbow-contracts: (Work in progress). Implementation of the Contract Protocol of the Dataspace Protocol.
- Rainbow-common: lib crate for common functionality in the project.
- Rainbow-core: Binary to run the whole project.
- Deployment: Material needed to deploy Docker containers, and (Work in progress) recipes for Kubernetes deployment.

## Getting started

Coming soon…