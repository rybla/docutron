---
trigger: always_on
---

Make good use of the Rust's logging feature, via the `log` module.
- Use `log::trace!(...)` at beginning of each function, and to indicate when important steps happen or import results are computed. 
- Use `log::warn!(...)` in code paths where an attempted computation had a problem, but the current thread should continue nevertheless.
