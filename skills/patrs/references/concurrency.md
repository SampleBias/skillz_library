# Concurrency

## Decisions

- Use async when concurrent asynchronous I/O and the project's runtime make it useful; do not introduce a runtime for synchronous work without a reason. Avoid blocking an executor worker: use the runtime's blocking facility or dedicated threads for blocking calls and CPU-heavy work, with appropriate limits.
- Prefer single ownership and message passing when they clarify responsibility; channels also introduce buffering, backpressure, shutdown, and error-handling decisions. Use locks or atomics when shared state is simpler and their invariants are clear.
- Keep lock scope short; avoid holding a synchronous lock across `.await`. Document lock ordering for multiple locks, and define cancellation and shutdown behavior where interruption can leave partial work. An async mutex across `.await` can be legitimate, but examine contention and deadlock risks.
- Select bounded versus unbounded queues deliberately, handle closed channels, and decide how work is drained or cancelled. Test relevant races and failure paths, without assuming tests prove all interleavings safe.

## Example

```rust
// Good: release the lock before slow work.
let snapshot = {
    let state = shared.lock().map_err(|_| Error::Poisoned)?;
    state.snapshot()
};
process(snapshot).await?;

// Poor: holds a synchronous lock while asynchronous work is pending.
let state = shared.lock().map_err(|_| Error::Poisoned)?;
process(state.snapshot()).await?;
```

`shared`, `Error`, and `process` are illustrative. Copying a snapshot has a cost and must preserve the desired consistency semantics. Common generated-code antipatterns include unbounded task spawning, unbounded queues without backpressure, and assuming dropping a future rolls back external effects.
