# ZCommerce - Actix Web REST API

### Documentation

### Dev

### Set SQLX Offline mode

SQLX_OFFLINE=true cargo sqlx prepare --workspace

### Run Docker image

Environment Variables

```Env
DATABASE_URL=""
SECRET=""
SERVICE_ADDR=""
SERVICE_PORT=""
SERVICE_WORKER=""

```

##### Defaut Env Defined in Docker Image

SERVICE_ADDR=""
SERVICE_PORT=""

### Toolkits

- Cargo make - task runner
- Docker     - containers
- sqlx-cli   - db migration
- shuttle-cli - cloud hosting
- bacon/cargo watch - recompile on code source change
- uv - python package manager (for basic scripting)
