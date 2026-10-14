# Scripts

Helper scripts, grouped by purpose. Every script resolves its paths from its own location, so it
can be launched from any directory. Most tasks of the root `Taskfile.yml` call them (`task --list`).

| Folder | Scripts | What they do |
|---|---|---|
| `run/` | `run-mini.ps1` | Brings up the mini stacks (published images) and waits until each agent answers. `-Target provider\|consumer\|all`, `-Down`. |
| | `run-provider-dev`, `run-consumer-dev` (`.ps1`, `.sh`) | Dev mode: infrastructure in Docker (`deployment/dev`), the agent from source with `cargo watch`. |
| | `compile.ps1` | Builds the admin console and copies it into the BFF. |
| `onboarding/` | `auto-onboarding.ps1`, `auto-onboarding-dev` (`.ps1`, `.sh`), `auto-onboarding-mini.sh`, `auto-onboarding-prod.ps1` | Full onboarding against an authority on `:1500`: credentials for the consumer and the provider, then the consumer with the provider. One variant per environment (URLs). |
| | `gaia.ps1` | Gaia-X flow against the local authority. |
| `data/` | `populate_all.sh`, `populate_mock_*.sh` | Seed mock catalogs, contracts, transfers and mates into a running agent. |
| | `populate_peer_catalogs.sh` | Seeds peer catalogs into the agent's Redis cache. |
| `tck/` | `run-tck.sh`, `populate_tck.sh` | Prepare the agents and run the Eclipse DSP TCK. |
| `tools/` | `add_license.sh`, `build-admin-gui.sh`, `build-openapi.py` | License headers, admin console build, OpenAPI artefacts from the per-agent specs. |
| `lib/` | `auth.sh` | Shared by the bash scripts: the user token for the API calls. |
