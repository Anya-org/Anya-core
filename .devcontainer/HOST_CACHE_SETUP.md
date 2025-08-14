# Host Cache Directory Setup (F: Drive)

These steps prepare the Windows host F: drive for build caches used by either:

- The base dynamic config (`devcontainer.json`) which auto-selects a cache location.
- The explicit F: bind config (`devcontainer.fdrive.json`) that forces use of host F: drive.

Required directories (created once):

```
F:\anya-rust-target
F:\anya-cargo-registry
F:\anya-cargo-git
F:\anya-sccache
```

## Quick Setup (PowerShell)

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
./.devcontainer/create-host-cache-dirs.ps1
```

## Quick Setup (CMD)

```cmd
cmd /c .devcontainer\create-host-cache-dirs.bat
```

## Rebuild Dev Container (Dynamic Base Config)

1. Open Command Palette: Ctrl+Shift+P
2. Select: Dev Containers: Rebuild and Reopen in Container
3. After rebuild, verify inside container:

```bash
ls -ld target
du -sh target 2>/dev/null || true
echo "SCCACHE DIR: $SCCACHE_DIR"
```

## Alternate F: Bind Config

To force host F: usage:

1. Ensure directories exist (above).
2. Command Palette: Dev Containers: Open Folder in Container...
3. Choose `.devcontainer/devcontainer.fdrive.json`.
4. Rebuild.

The dynamic script still runs but finds pre-bound paths.

## Maintenance

- Prune target occasionally: `cargo clean` (will re-populate)
- Prune cargo registry (rare): `cargo cache -a` (after `cargo install cargo-cache`)
- sccache stats: `sccache --show-stats`
- cache status script: `bash .devcontainer/cache-maintenance.sh`

### Dynamic Script Reference

`dynamic-cache-setup.sh` evaluates candidate paths (e.g. `/mnt/f`, `/workspace/F`) selecting the first with >= `DYNAMIC_CACHE_MIN_FREE_GB` (default 8). Override via env:

```
DYNAMIC_CACHE_MIN_FREE_GB=4
DYNAMIC_CACHE_FORCE_ROOT=/workspaces/Anya-core/.fast-cache
DYNAMIC_CACHE_VERBOSE=1
```

### Auto-Prune Environment Variables

The dynamic setup optionally auto-triggers pruning if space is low.

Variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `AUTO_PRUNE_ON_SETUP` | `1` | If `1`, dynamic-cache-setup invokes pruning when free space < `PRUNE_TRIGGER_GB`. Set to `0` to disable. |
| `PRUNE_TRIGGER_GB` | `6` | Free space (GB) threshold below which pruning is attempted. |
| `PRUNE_MIN_FREE_GB` | `6` | Minimum free GB target passed to prune script (overridable when running prune manually). |
| `PRUNE_TARGET_MAX_GB` | `15` | Target directory size threshold (GB) beyond which it is removed entirely during pruning. |

Manual prune example:

```bash
PRUNE_MIN_FREE_GB=10 PRUNE_TARGET_MAX_GB=20 bash .devcontainer/prune-caches.sh
```

Disable auto prune in a container rebuild:

```jsonc
"containerEnv": { "AUTO_PRUNE_ON_SETUP": "0" }
```

## WSL Alignment Notes

If using WSL2:

- Prefer cloning the repo into the Linux filesystem (`/home/<user>/Anya-core`) for faster IO.
- Only bind/consume Windows F: for persistence or space; the dynamic script auto-detects `/mnt/f`.
- Validate presence: `ls /mnt/f` (list should appear). If missing: `wsl --shutdown` then restart your terminal.
- Run helper: `bash .devcontainer/wsl-align.sh` for checks & tips.
- Adjust Windows `%UserProfile%\\.wslconfig` for resource tuning (see script output).

Performance:

- Keep `sccache` size modest given F: capacity.
- Use sparse protocol (already configured).
- Consider enabling `CARGO_NET_GIT_FETCH_WITH_CLI=true` for large git dependencies.

## Troubleshooting

| Symptom | Cause | Fix |
|---------|-------|-----|
| Container fails to start (bind mount error) | Directories missing on F: | Run creation script, rebuild |
| Builds fail with No space left on device | F: full | Delete old artifacts (`cargo clean`, prune registry, reduce sccache) |
| Performance slow | F: is slower HDD | Move only `target` back to volume or reduce caching scope |

## Adjusting Cache Size

Edit `SCCACHE_CACHE_SIZE` in `devcontainer.json` (currently 4G) if you need more/less space.

---
Document supports offloading build caches due to limited C: drive capacity.
