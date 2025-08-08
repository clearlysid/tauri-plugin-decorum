# tauri-plugin-decorum

An opinionated Tauri (v2) plugin for custom window decorations that retains native features like Windows Snap Layout while providing better UI integration with transparency and overlay controls, plus properly inset macOS traffic lights.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Required System Dependencies
Install these system dependencies before building (Linux only):
```bash
sudo apt-get update
sudo apt-get install -y webkit2gtk-4.1 libgtk-3-dev libglib2.0-dev libpango1.0-dev libatk1.0-dev libgdk-pixbuf-2.0-dev libappindicator3-dev librsvg2-dev
```

### Bootstrap and Build Process
1. Install dependencies and build the plugin:
   ```bash
   cargo check  # Quick validation - takes ~1 minute. NEVER CANCEL.
   cargo build  # Full build - takes ~3-5 minutes. NEVER CANCEL. Set timeout to 10+ minutes.
   ```

2. Build the TypeScript/JavaScript API:
   ```bash
   yarn install  # Takes ~5-35 seconds
   yarn build    # Takes ~1-2 seconds. Builds to dist-js/
   ```

### Test and Lint Commands
- Run tests: `cargo test` -- takes ~1-2 minutes. NEVER CANCEL. Set timeout to 5+ minutes.
- Format code: `cargo fmt --all` -- takes <1 second
- Check formatting: `cargo fmt --all -- --check` -- takes <1 second 
- Lint code: `cargo clippy --all-targets --all-features -- -D warnings` -- takes ~2-3 minutes. NEVER CANCEL. Set timeout to 10+ minutes.

**CRITICAL NOTE**: The codebase currently has clippy warnings that will cause CI to fail. Always run `cargo clippy` and fix warnings before committing.

### Example Application Testing
The repository includes a fully functional example Tauri app in `examples/tauri-app/`:

1. Navigate to example app:
   ```bash
   cd examples/tauri-app/
   yarn install  # Takes ~35 seconds
   ```

2. Run in development mode:
   ```bash
   yarn tauri dev  # Takes 3-5 minutes to compile. NEVER CANCEL. Set timeout to 15+ minutes.
   ```

3. Build production version:
   ```bash
   yarn tauri build  # Takes ~4-5 minutes. NEVER CANCEL. Set timeout to 15+ minutes.
   # Note: AppImage bundling may fail due to certificate issues in CI environments
   ```

## Validation Requirements

### Manual Testing Scenarios
After making changes to the plugin, ALWAYS test the complete functionality:

1. **Build and run the example app**: Use `yarn tauri dev` in `examples/tauri-app/` 
2. **Test window controls**: Verify minimize, maximize, and close buttons work
3. **Test platform-specific features**:
   - **macOS**: Traffic light positioning and transparency
   - **Windows**: Custom titlebar and snap layouts
   - **Linux**: Custom decorations work properly

### CI Requirements
Always run these commands before committing to ensure CI passes:
```bash
cargo fmt --all                                    # Format code
cargo clippy --all-targets --all-features -- -D warnings  # Lint and fix ALL warnings
cargo test                                         # Run tests
```

**WARNING**: CI will fail if there are any clippy warnings or formatting issues.

## Build Time Expectations

| Command | Expected Time | Timeout Setting |
|---------|---------------|-----------------|
| `cargo check` | ~1 minute | 5+ minutes |
| `cargo build` | ~3-5 minutes | 10+ minutes |
| `cargo test` | ~1-2 minutes | 5+ minutes |
| `cargo clippy` | ~2-3 minutes | 10+ minutes |
| `yarn build` | ~1-2 seconds | 1 minute |
| `yarn tauri dev` | ~3-5 minutes | 15+ minutes |
| `yarn tauri build` | ~4-5 minutes | 15+ minutes |

**NEVER CANCEL** any build or test commands. Builds can take several minutes, especially on first run.

## Repository Structure

### Key Source Files
- `src/lib.rs` - Main plugin implementation and WebviewWindow extensions
- `src/commands.rs` - Tauri command handlers
- `src/traffic.rs` - macOS traffic light positioning logic
- `src/dconf.rs` - Linux desktop configuration utilities
- `src/js/` - JavaScript files injected into webviews for window controls
- `guest-js/index.ts` - TypeScript API for the npm package

### Build Configuration
- `Cargo.toml` - Rust dependencies and plugin metadata
- `package.json` - npm package configuration for guest API
- `rollup.config.js` - Builds TypeScript to JavaScript for npm distribution
- `build.rs` - Tauri build script for embedding JavaScript

### Example Application
- `examples/tauri-app/` - Complete example showing plugin usage
- Uses React + Vite frontend with Tauri backend
- Linked to main plugin via `"tauri-plugin-decorum-api": "link:../../"`

## Platform-Specific Behavior

### macOS
- Provides traffic light positioning with `set_traffic_lights_inset(x, y)`
- Supports window transparency with `make_transparent()`
- Handles window level changes with `set_window_level(level)`

### Windows  
- Creates custom titlebar overlay with `create_overlay_titlebar()`
- Preserves native snap layouts and window management
- Injects custom window control buttons

### Linux
- Uses system theme icons via linicon for window controls
- Configures desktop environment settings through dconf
- Provides fallback window control implementation

## Common Development Tasks

### Adding New Plugin Commands
1. Add command handler in `src/commands.rs`
2. Register command in `src/lib.rs` plugin builder
3. Export command in `guest-js/index.ts` for JavaScript API
4. Update permissions in `permissions/` if needed
5. Test with example app

### Updating JavaScript API
1. Modify `guest-js/index.ts`
2. Run `yarn build` to compile to `dist-js/`
3. Test with example app that links to local package

### Testing Changes
1. Always test with the example app: `cd examples/tauri-app && yarn tauri dev`
2. Run on target platforms when possible
3. Verify window control functionality manually
4. Run all CI commands locally before committing

## Common Issues

### Build Failures
- **Linux**: Missing webkit2gtk dependencies - install system packages listed above
- **Network timeouts**: Normal during first build - wait for completion, don't cancel
- **Clippy warnings**: Must be fixed - CI treats warnings as errors

### Development Environment
- Requires Rust 1.70+ and Node.js for full development
- Example app requires Tauri prerequisites: https://beta.tauri.app/start/prerequisites/
- Different platforms may need platform-specific build tools

## Package Information

This repository builds two artifacts:
1. **Rust crate**: `tauri-plugin-decorum` for Tauri app integration  
2. **npm package**: `tauri-plugin-decorum-api` for JavaScript/TypeScript APIs

The plugin is in maintenance mode - no breaking API changes planned, only bugfixes and improvements.