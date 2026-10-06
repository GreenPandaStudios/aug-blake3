# August BLAKE3

Hash Bytes with the Rust blake3 crate and return its lowercase hexadecimal digest.

**Unreleased 0.2.0 candidate.** This source requires August 1.0.0; compiler qualification and publication are pending. With the published August 0.23.0 compiler, use package v0.1.5.

Supported artifacts target macOS 14+ ARM64 and GNU/Linux x86-64 or ARM64 with glibc 2.36+. Consumers need Node 24+ and August. The CLI obtains prebuilt libraries and the compiler pack; no separate native compiler is required.

## Use it after publication

```sh
aug init native-example
cd native-example
aug add https://github.com/GreenPandaStudios/aug-blake3#v0.2.0 --as blake3
```

Save this as main.aug:

```aug
import HashError and hash from blake3
try:
    Bytes input = "abc".bytes()
    print(value=hash(input))
catch HashError error:
    print(value=error.message)
```

Run aug run. Expected output:

```text
6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85
```

Commit aug.lock.json. A frozen offline run uses the locked source and verified cached artifacts. Deploy the executable with its neighboring lib and share directories. An unsupported target or missing artifact stops installation; source builds require an explicit maintainer action.

## Ownership and failures

The wrapper borrows input bytes only for the call and copies the digest into August text. HashError is checked. Rust panics are contained at the C ABI boundary. No native object or Rust allocation escapes into application code.

## Maintain the package

Start at src/export.aug and the adjacent compiled specifications. native.abi.json declares symbols, input bounds, ownership, release functions and checked failures. Update native declarations and their descriptor together. Run aug check, aug test and aug spec with the required compiler.

A native build uses Rust 1.98.1 and Clang, plus the Apple SDK on macOS or the pinned Debian 12 maintainer image on Linux; its exact requirements and upstream inputs are in aug-package.json and native/sources.lock.json. Run node native/build.mjs, review the candidate archive and manifest, and publish the exact measured bytes. The archive contains dependency, license and provenance records. Installation never runs the recipe.

The 0.2.0 candidate preserves the previous native and August bindings and reuses their checksum-pinned archives. The compatibility publisher checks those inputs against the original release. A binding or adapter change requires new native qualification. Repeat real LLVM operations, cleanup and clean-consumer tests for the new compiler before publishing its support claim.
