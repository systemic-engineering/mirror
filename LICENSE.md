# License

This project uses a **layered license model**:

- **The compiler, the Prism algebra, the boot grammars (`boot/`, `boot/std/`), `@spectral/portal`, `@spectral/mosaic`, the `@spectral/db/{mnesia,sql/postgres,sql/lite}` adapter contracts, and all open substrate** are licensed under **Apache License 2.0**. See [`license/APACHE2.md`](./license/APACHE2.md).

- **The curated corpus, the `@spectral/garden` reviewed packages, deployed instances of the `@spectral/db` engine, and any Covered System built on the `@mirror` substrate that exits the open Apache-2.0 surface into operational deployment** is licensed under the **systemic.engineering License (SEL)**. See [`license/SEL.md`](./license/SEL.md).

- **The `@spectral/db` graph engine** itself is closed-source (binary-only).

- **`@spectral/garden` package contents** are licensed per-package by their curators (Apache-2.0, SEL, commercial, or mixed); the substrate verifies signatures regardless of license.

See [`README.md`](./README.md) for the framing and the `e^(n+1) < e^n` proof.

The glass is Apache-2.0. The wine governs itself per the curator's choice. The `au + io` boundary is where the SEL petri-net enforcement attaches; see `license/SEL.md` Part II and `tasks/103` for the operational substrate.
