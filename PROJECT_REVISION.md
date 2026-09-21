# Project Revision I — Ample

This branch is the ample side of the coordinated Project Revision I.

## Role in the revision

`ample` owns abstractions and representation machinery. It must not encode
userspace-specific memory policy.

## Representation laws

- `Bytes<Origin, Destination>` describes representation.
- Representation size and Rust in-memory size are distinct concepts.
- Serialization and deserialization are operations over representation.
- Allocation must not infer object layout from representation size.
- `Origin` and `Destination` remain meaningful dimensions of representation;
  current diagonal uses do not justify collapsing them.
- Macro-generated representation remains the source of truth for generated
  constants and representation behavior.

## Naming laws

- Every identifier must state what it means.
- Established Rust, systems, ABI, and standards terminology takes precedence
  over project-local vocabulary.
- Acronyms and abbreviations are aliases unless an external normative
  identifier is reproduced verbatim.
- Internal underscore-composed module names indicate hidden hierarchy.
- Concrete nouns precede qualifications.
- A misleading identifier is a semantic defect.

## Sprint I.2 — Representation Semantics

Completed:

- `BYTES_SIZE` became `REPRESENTATION_SIZE`;
- representation is the canonical term; wire and serialized representation are
  not separate ontological categories;
- `BYTES_ALIGN` was removed from `Bytes` because it only propagated Rust
  memory alignment and had no representation behavior;
- userspace allocation no longer derives object storage from representation
  extent;
- the integrated hub gate passed after the representation change.

## Sprint I.3 — Allocation Semantics

In progress:

- `Allocating` is the canonical unsafe allocation capability;
- `core::alloc::Layout` is the canonical description of allocation layout;
- the old `Allocatable` / `AllocatableResult` abstraction was removed;
- dormant allocation experiments and orphaned modules were deleted;
- linked containers were decoupled from `Bytes`, `Origin`, and `Destination`;
- allocation strategy belongs to the allocator, not to the represented value.

No compatibility promise is made for legacy names during this revision.
