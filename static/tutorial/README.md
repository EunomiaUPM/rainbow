# Eunomia DS Negotiation Agent Tutorial

This folder contains Python notebooks demonstrating how to interact with the Eunomia DS- Negotiation Agent and Transfer Agent.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed and running.
- [Python 3.8+](https://www.python.org/downloads/) installed.
- [JupyterLab](https://jupyter.org/install) or VS Code with Jupyter extension.

## Deployment

To start the system with all necessary components (Mock Provider, Consumer, Keycloak, Vault, etc.), use the provided auto-onboarding script.

**From the project root directory:**

```bash
# For MacOS/Linux
./scripts/bash/auto-onboarding.sh

# For Windows (PowerShell)
./scripts/powershell/auto-onboarding.ps1
```

This script will:
1. Start the Docker containers using `docker-compose.dev.yaml`.
2. Wait for services to be ready.
3. Onboard participants and generate necessary secrets.

**Note:** Ensure you have `jq` and `curl` installed if running the bash script.

## Running the Notebooks

1. Install the required Python dependencies:

```bash
pip install -r requirements.txt
```

2. Start JupyterLab:

```bash
jupyter lab
```

3. Open the notebooks in this folder (e.g., `01_negotiation.ipynb`) and follow the instructions.

## Notebooks

### 1. Setup & Configuration
- `setup_auth.ipynb`: Configures base URLs, links wallets, retrieves participant DIDs, requests credentials from Authority, and exchanges mutual peer access tokens.
- `setup_catalog_policies.ipynb`: Populates Provider DCAT catalog, creates datasets, distributions (`PULL` via `http+asd` and `PUSH` via `http+pushing`), ODRL policies, and registers `NO_AUTH` connector instances.
- `setup_catalog_policies_oauth.ipynb`: Populates catalog with secret provisioning in Keystore (`/ecostars/api_key`) and registers connector instances using `BEARER_TOKEN` authentication.

### 2. Contract Negotiation (Standalone & Callable)
- `contract_negotiation.ipynb`: Full canonical DSP 2025-1 Contract Negotiation sequence (Request -> Offer -> Counter-Request -> Acceptance -> Agreement -> Verification -> Finalization). Callable via `%run ./contract_negotiation.ipynb`.
- `bff_contract_negotiation.ipynb`: Automated BFF contract negotiation flows (both Consumer request flow and Provider offer flow). Callable via `%run ./bff_contract_negotiation.ipynb`.

### 3. Transfer Process Execution (Eunomia RPC)
- `pull_flow.ipynb`: PULL data transfer flow over Eunomia RPC (`setup-request`, `setup-start`, `setup-suspension`, restart, role/shape failure tests, and `setup-termination`). Automatically invokes `contract_negotiation.ipynb` with `setup_catalog_policies_oauth.ipynb`.
- `push_flow.ipynb`: PUSH data transfer flow over Eunomia RPC (`setup-request` with sink `dataAddress`, `setup-start`, `setup-suspension`, restart, role test, and `setup-completion`). Automatically invokes `contract_negotiation.ipynb` with `setup_catalog_policies.ipynb`.
- `bff_push_flow.ipynb`: PUSH data transfer flow driven by automated BFF contract negotiation. Automatically invokes `bff_contract_negotiation.ipynb` with `setup_catalog_policies.ipynb`.

