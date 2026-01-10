# ADR-03-1: Multi-Level Caching Strategy

**Date:** December 29, 2025  
**Status:** Accepted  
**Context:** Performance / Persistence

## Context and Problem Statement

The Dataspace Protocol is read-heavy by design. Operations such as deep "Get All" requests or recursive "Lookups" were identifying the direct SQL database access as a significant performance bottleneck. To achieve sub-millisecond latency for metadata retrieval, a caching layer was necessary. However, caching complex relational data introduces challenges regarding consistency and invalidation.

## Decision

We have decided to implement a transparent caching layer via a new `EntityCacheTrait`. This abstraction allows us to intercept service calls before they reach the SQL repository. We selected **Redis** as the backing store due to its support for complex data types. The strategy employs a hybrid approach:
1.  **Single Entities:** Stored as JSON strings mapped directly by their URN (Key-Value).
2.  **Collections:** Stored using Redis **Sorted Sets** (`ZSET`), where the score is the numeric timestamp (`dct_issued`). This allows for extremely efficient pagination without interacting with the database.

```rust
#[async_trait]
pub trait EntityCacheTrait<M>: Send + Sync 
where M: Serialize + DeserializeOwned + Clone + Send + Sync 
{
    // O(1) Access
    async fn get_single(&self, id: &Urn) -> anyhow::Result<Option<M>>;
    
    // Sorted Sets for Pagination (Score = Timestamp)
    async fn get_collection(&self, key: &str, offset: isize, limit: isize) -> anyhow::Result<Vec<M>>;
    
    // Batch Retrieval with Gap Detection
    async fn get_batch(&self, ids: &[Urn]) -> anyhow::Result<Vec<Option<M>>>;
}
```

## Resilience and Consistency

A critical component of this design is the Resilient Hydration algorithm. When fetching a collection, the system first retrieves IDs from the Sorted Set and then fetches the bodies via MGET. If any bodies return nil (meaning the payload was evicted but the index remains), the system identifies these "gaps," queries the SQL database only for the missing items, and repairs the cache in real-time before responding to the user.

To maintain consistency during write operations, we enforce Atomic Deletes using Redis Pipelines. When an entity is deleted, the pipeline simultaneously removes the single key, the entry in the global Sorted Set, and any references in relational Lookup sets. This prevents "dangling pointers" where an ID exists in a list but the data is gone, ensuring the integrity of the API responses.



