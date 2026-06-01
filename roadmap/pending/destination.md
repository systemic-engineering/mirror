# Phase 7 — The Destination

Mirror compiles mirror. The Rust crate becomes a thin runtime substrate: syscalls, LAPACK eigenvalue computation, SHA hashing (only at `@fragmentation/git` adapter boundary; substrate uses `CoincidenceHash` natively), Metal/OpenCL GPU dispatch, the BEAM FFI for spectral-db distribution. Everything else is `.mirror` source. Fragmentation's Rust source is generated from `@fragmentation + @code/rust`. The Scheduler Tower regulates the runtime's temperature at the KMS-equilibrium point. The system can be deployed at spectral.engineer and serve real workloads.
