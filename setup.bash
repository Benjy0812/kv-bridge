#!/usr/bin/env bash
set -euo pipefail

# This script is AI generated
ENV_FILE=".env"
NEW_TTL="2h"
OLD_CLOUD_TTL="10"      # only used for the 403 check near the end
OLD_ANTENNA_TTL="90"    # must outlive the ~47s of sleeps before the new secret is pushed
touch "$ENV_FILE"

# set_env KEY VALUE: update the key in .env, or append it if missing
set_env() {
  local key="$1" val="$2"
  if grep -q "^${key}=" "$ENV_FILE"; then
    sed -i.bak "s|^${key}=.*|${key}=${val}|" "$ENV_FILE" && rm -f "${ENV_FILE}.bak"
  else
    echo "${key}=${val}" >> "$ENV_FILE"
  fi
}

# write_policy NAME CONTAINER_ID
write_policy() {
  local name="$1" container="$2"
  docker exec -i "$container" vault policy write "${name}-access" - <<EOT
path "secret/*" {
    capabilities = ["create", "read", "update"]
}
EOT
}

# make_token NAME CONTAINER_ID ENV_KEY TTL
make_token() {
  local name="$1" container="$2" env_key="$3" ttl="$4"
  local token
  token=$(docker exec "$container" vault token create \
    -policy="${name}-access" -ttl="$ttl" -field=token)
  set_env "$env_key" "$token"
  echo "Updated ${env_key} in ${ENV_FILE} (ttl=${ttl})"
}

CLOUD_CONTAINER=Cloud
ANTENNA_CONTAINER=Antenna

clear
echo
echo "Starting setup"
echo

# Policies (once per vault)
write_policy cloud   "$CLOUD_CONTAINER"
write_policy antenna "$ANTENNA_CONTAINER"

# Phase 1: NEW tokens
echo
echo "Creating NEW tokens"
make_token cloud   "$CLOUD_CONTAINER"   NEW_CLOUD_VAULT_TOKEN   "$NEW_TTL"
make_token antenna "$ANTENNA_CONTAINER" NEW_ANTENNA_VAULT_TOKEN "$NEW_TTL"

# Phase 2: OLD tokens (short-lived, so run the demo right after)
echo
echo "Creating OLD tokens"
make_token cloud   "$CLOUD_CONTAINER"   OLD_CLOUD_VAULT_TOKEN   "$OLD_CLOUD_TTL"
make_token antenna "$ANTENNA_CONTAINER" OLD_ANTENNA_VAULT_TOKEN "$OLD_ANTENNA_TTL"

echo
echo "Done"
cargo run