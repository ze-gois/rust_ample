# Project Revision I — Ample

This branch is the ample side of the coordinated Project Revision I.

## Role in the revision

`ample` owns abstractions and representation machinery. It must not encode
userspace-specific memory policy.

The first focus is representation semantics.

## Representation laws

- `Bytes<Origin, Destination>` describes representation.
- Representation size and Rust in-memory size are distinct concepts.
- Serialization and deserialization are operations over representation.
- Representation alignment, if such a property is retained, must describe
  representation rather than Rust in-memory alignment.
- Allocation must not infer object layout from representation size.
- `Origin` and `Destination` remain meaningful dimensions of representation;
  current diagonal uses do not justify collapsing them.
- Macro-generated representation must remain the source of truth for generated
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

Initial findings:

- `REPRESENTATION_SIZE` is predominantly used as representation extent.
- `BYTES_ALIGN` frequently derives from Rust `align_of::<T>()`, mixing two
  different semantic domains.
- `userspace::memory::heap::Allocating` currently consumes `REPRESENTATION_SIZE` and
  `BYTES_ALIGN` as allocation layout; that coupling must be removed in Sprint
  I.3.
- The canonical term is representation. Terms such as wire or serialized
  representation are not separate ontological categories.
- The revision must decide whether `REPRESENTATION_SIZE` and `BYTES_ALIGN` should be
  renamed, split, or otherwise made semantically explicit before changing
  consumers.

No compatibility promise is made for legacy names during this revision.
