# Test assets

Tiny fixtures used by the integration tests in
`thorvg/tests/integration.rs`.

## License

The files in this directory (`logo.png`, `logo.svg`, `red_1s.mp4`) are released
into the **public domain** under
[CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/).

They are hand-authored test fixtures and are **not** affiliated with
the official ThorVG brand assets (which live in the proprietary
[`thorvg.site`](https://github.com/thorvg/thorvg.site) repository and
are not redistributable).

## Contents

| File       | Size   | Format     | Purpose                                                    |
|------------|--------|------------|------------------------------------------------------------|
| `logo.png` | 32×32  | PNG (RGBA) | Returned from a Rust asset resolver to verify the success branch of the FFI trampoline. |
| `logo.svg` | 32×32  | SVG        | SVG counterpart for completeness; not currently exercised in tests but kept alongside the PNG. |
| `red_1s.mp4` | 16×16 | MP4 (H.264) | 1 s, 10 fps solid red clip for the `Video` tests (macOS / iOS with the `video` feature). Generated with `ffmpeg -f lavfi -i color=c=red:s=16x16:r=10:d=1 -c:v libx264 -pix_fmt yuv420p -profile:v baseline -movflags +faststart -map_metadata -1 -fflags +bitexact red_1s.mp4`. |
