# Changelog

All notable changes to the `thorvg-sys` crate are documented here. The
format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Versioning.** This crate uses its **own** SemVer, decoupled from the
vendored ThorVG C++ release. The bundled ThorVG version is recorded as
SemVer **build metadata** — e.g. `0.1.0+thorvg-1.0.5` is crate `0.1.0`
bundling ThorVG `1.0.5`. Because the crate is `0.x`, a **minor** bump is
breaking; the safe [`thorvg`](../thorvg/CHANGELOG.md) crate's dependency
moves in lockstep.

## [0.4.0+thorvg-1.1.2] - 2026-10-13

Bumps the vendored engine to **ThorVG 1.1.2** (through 1.1.1) and
regenerates the FFI. A **minor** (breaking) bump: `Tvg_Result` gains a
variant, which breaks exhaustive matches on it. Every other binding
change is an addition.

### Changed

- **Bundled ThorVG 1.1.0 → 1.1.2.** The vendored submodule tracks the
  rebased `bare-metal/v1.1.2` patch branch; the series applied unchanged
  and `THORVG_VERSION_STRING` is updated to match.
- **`Tvg_Result::TVG_RESULT_SYSTEM_ERROR`** (new, experimental upstream)
  — reported when an underlying system operation fails. Loaders also now
  return more specific results from `tvg_picture_load*` instead of a
  generic failure (thorvg/thorvg@58940d8d).

### Added

- **`video` feature** (off by default) — compiles ThorVG's experimental
  media loader with its AVFoundation backend
  (`src/loaders/media/apple/tvgAvfMediaLoader.mm`) and links the
  `AVFoundation`, `CoreVideo`, `CoreMedia`, `Foundation` and
  `CoreFoundation` frameworks. ThorVG ships a native backend only for
  Apple platforms, so the feature applies on macOS / iOS and is a no-op
  elsewhere. It also enables video playback inside Lottie image assets.
- **`tvg_video_*`** — video playback API (`new` / `del` /
  `get_picture`, `play` / `pause` / `stop` / `seek`, loop, volume, mute,
  time and duration). Always bound; `tvg_video_new` returns null when no
  media backend is compiled in.
- **`tvg_paint_set_data`** / **`tvg_paint_get_data`** — an opaque
  user-data slot on every paint.
- **`tvg_lottie_animation_set_volume`** /
  **`tvg_lottie_animation_get_volume`** — master volume for a Lottie
  animation's media assets.
- **`tvg_lottie_animation_expressions_supported`** — whether the build
  evaluates Lottie expressions.

### Fixed

Inherited from the engine bump:

- Security hardening: a heap overflow in the JPEG loader, malformed
  Lottie asset handling, an out-of-bounds read on malformed SVG tags, a
  use-after-free from dangling clip / mask parent pointers, and memory
  leaks in the PNG and font loaders.
- Lottie: matte sources hidden when referenced by `tp`, double-applied
  solid-layer opacity, playback clamping to the last frame, and audio
  offsets of trimmed layers.
- Trim paths losing `moveTo` on wrap-around, drop-shadow offset
  transforms, and anti-aliasing of 8-bit and repeat gradients.

### Packaging

- `*.mm` sources are now part of the package `include` list, so the
  AVFoundation backend ships in the published crate.

## [0.3.2+thorvg-1.1.0] - 2026-08-18

Packaging fix; same **ThorVG 1.1.0** engine and identical FFI as
`0.3.1`. A **patch** bump — no API, symbol, or behaviour change.

### Fixed

- **Bare-metal builds from crates.io failed: the published package
  omitted the vendored picolibc tree entirely.** The `include` allowlist
  in `Cargo.toml` listed the `thorvg/**` sources but no `picolibc/` or
  `picolibc-config/` entries, so `cargo package` shipped no picolibc.
  Host and system-lib builds were unaffected (picolibc is only compiled
  for `target_os = "none"`), which is why it went unnoticed until an
  embedded crate consumed the package from crates.io rather than via a
  path dependency. Added `picolibc/libc/**`, `picolibc/libm/**`
  (`libc/` headers reach into `libm/`), `picolibc/COPYING.picolibc`, and
  `picolibc-config/**` to the include list. Affected `0.3.1` and earlier
  bare-metal-capable releases.

### CI

- **Added a regression guard** (`bare-metal.yml`) that `cargo package`s
  the crate and builds the resulting tarball for
  `riscv32imac-unknown-none-elf` — exercising the package `include` list
  against a `target_os = "none"` target, which the existing
  workspace-tree build cannot do.

## [0.3.1+thorvg-1.1.0] - 2026-08-17

Bumps the vendored engine to **ThorVG 1.1.0** and regenerates the FFI. A
**patch** (non-breaking) bump: the regenerated bindings only add symbols
— nothing was removed and no signature or enum changed.

### Changed

- **Bundled ThorVG 1.0.7 → 1.1.0.** The vendored submodule tracks the
  rebased `bare-metal/v1.1.0` patch branch; `THORVG_VERSION_STRING` is
  updated to match.
- **The bare-metal patch series shrank from 5 files to 4.** Upstream 1.1
  replaced `tvgSwRenderer.cpp`'s file-scope `static mutex _rendererMtx`
  with a `StrictKey`, whose no-thread form is inert — so the downstream
  `_NullMutex` shim is no longer needed. `tvgLock.h` still drops
  `StrictKey`'s mutex member outright rather than relying on upstream's
  `__STDCPP_THREADS__` guard, so the bare-metal guarantee does not depend
  on how the cross toolchain defines that macro.

### Added

- **`tvg_paint_intersects_region`** — hit-test variant of
  `tvg_paint_intersects` taking a `visibleOnly` flag that excludes hidden
  paints from the test.
- **`tvg_lottie_animation_tween_go`** and
  **`tvg_lottie_animation_tween_to`** — progress- and target-based
  tweening entry points alongside the existing
  `tvg_lottie_animation_tween`.

### Fixed

Inherited from the engine bump, all on the CPU (software) path this
crate builds:

- Outdated gradient fill on transform-only updates.
- Hit-test miss on retained axis-aligned shapes.
- Lottie point-text vertical alignment.

### Note

`tvg_lottie_animation_get_marker` is marked deprecated upstream. bindgen
does not propagate the attribute, so the generated binding is unchanged
and no deprecation warning is emitted.

## [0.3.0+thorvg-1.0.7] - 2026-07-10

Bumps the vendored engine to **ThorVG 1.0.7** and regenerates the FFI. A
**minor** (breaking) bump: the regenerated bindings change a public
function signature and add an enum variant.

### Changed

- **Bundled ThorVG 1.0.6 → 1.0.7.** The vendored submodule tracks the
  rebased `bare-metal/v1.0.7` patch branch; `THORVG_VERSION_STRING` is
  updated to match.
- **`tvg_text_get_glyph_metrics` gained a fourth parameter** — a
  `const char** next` out-pointer that receives the position just past
  the processed UTF-8 character. Existing three-argument call sites no
  longer compile.
- **`Tvg_Colorspace` gained the `TVG_COLORSPACE_GRAYSCALE8` variant**
  (single 8-bit channel). As a `rustified_enum`, an added variant breaks
  exhaustive matches.

### Added

- **`Tvg_WgContext`** struct (WebGPU instance / adapter / device) and
  **`tvg_wgcanvas_set_target_with_context`** for setting a WgCanvas
  target from an explicit WebGPU context.

## [0.2.1+thorvg-1.0.6] - 2026-06-15

Build-system and portability fixes; still bundles **ThorVG 1.0.6**. No
API change — a patch bump.

### Fixed

- **System (non-vendored) build now finds ThorVG 1.x.** `link_system`
  probed the unversioned `thorvg` pkg-config module, but ThorVG 1.x
  installs `thorvg-1` (meson `filebase: 'thorvg-' + vmaj`). Probe
  `thorvg-1` first, falling back to `thorvg` for distros that ship the
  unversioned name.
- **MSVC vendored build.** Define `NOMINMAX` — the Windows SDK's
  `min`/`max` macros mangled `tvgMath.h`'s `Point min(...)` / `max(...)`
  declarations (C2059) — and pass `/EHsc` for the standard C++ exception
  model.
- **Static linkage on MSVC.** Define `TVG_STATIC`: the vendored library
  is linked as a static archive, but without it the C API expanded to
  `__declspec(dllimport)` and MSVC rejected the definitions (C2491).

## [0.2.0+thorvg-1.0.6] - 2026-06-15

Bundles **ThorVG 1.0.6**. The minor bump reflects a breaking change to
the generated FFI surface (a removed C API symbol).

### Changed

- **Bumped vendored ThorVG to 1.0.6.** The seven bare-metal patches that
  sat on `v1.0.5` were rebased unchanged onto `v1.0.6`.

### Added

- `tvg_lottie_animation_set_audio_resolver`, the `Tvg_Audio_Resolver`
  callback type, and the `Tvg_Audio_Info` struct — new in ThorVG 1.0.6
  for synchronizing external audio playback with the Lottie timeline.
- `TVG_ENGINE_OPTION_ALIASED` — re-introduced upstream (disables
  anti-aliased rendering); it had been dropped in 1.0.5.

### Removed

- `tvg_lottie_animation_assign` — removed upstream in ThorVG 1.0.6.
  **Breaking** for any code that called it through the raw bindings.

## [0.1.0+thorvg-1.0.5] - 2026-06-13

First release under the crate's own versioning. **Supersedes the yanked
`1.0.0` / `1.0.1` / `1.0.5`**, which mirrored the upstream ThorVG version
number 1:1 — a scheme abandoned because it left no room to publish
sys-crate-only changes (build system, bare-metal support) while upstream
stayed at 1.0.5. Bundles **ThorVG 1.0.5**.

### Changed

- **Versioning scheme** — the crate version is now independent of
  upstream; the bundled ThorVG release is carried as `+thorvg-X.Y.Z`
  build metadata. The previous upstream-mirroring `1.0.x` releases are
  yanked.

### Build system

- **Replaced the meson + ninja build with the `cc` crate** — ThorVG is
  compiled from source via Cargo's configured (cross-)compiler; no meson
  or ninja required.
- **Feature-gated loaders and capabilities** — `lottie`, `svg`, `png`,
  `fonts`, `expressions`, `threads`, `file-io` Cargo features select
  which ThorVG components compile (all enabled by default; embedded users
  disable defaults and pick what they need).
- bindgen now passes an explicit `--target=` to libclang on host builds.

### Bare-metal support (`target_os = "none"`)

- Toolchain-agnostic cross-compilation pipeline, split into bare-metal
  vs. SDK-runtime policy.
- **Vendors picolibc** (submodule pinned to 1.8.11) as the libc on
  bare-metal: compile-time `picolibc.h` configuration plus a compile-only
  validation phase in `build.rs`.
- Per-concern runtime stubs welded to picolibc declarations with weak
  linkage; bridges newlib's `__errno()` to picolibc's plain `errno`;
  stubs `_on_exit`.
- RISC-V canonical-multilib selection; `expressions` enabled on
  bare-metal ESP32-C6.

### Vendored ThorVG patches

- Local shims layered on ThorVG 1.0.5 to support bare-metal builds:
  a `bsearch` shim, a Lottie-loader shim extension, and runtime-stub
  welding across `tvgLock.h`, `tvgInitializer.cpp`, `tvgRender.cpp`,
  `tvgSwRenderer.cpp`, and `tvgSwMemPool.cpp`.

[0.2.0+thorvg-1.0.6]: https://github.com/goyox86/thorvg-rs/releases/tag/thorvg-sys-v0.2.0
[0.1.0+thorvg-1.0.5]: https://github.com/goyox86/thorvg-rs/releases/tag/thorvg-sys-v0.1.0
