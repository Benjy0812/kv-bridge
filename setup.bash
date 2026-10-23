#!/usr/bin/env bash
set -euo pipefail

# This script is AI generated
ENV_FILE=".env"
OLD_TTL="20"
NEW_TTL="2h"
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

# Phase 1: OLD tokens
echo
echo "Creating OLD tokens"
make_token cloud   "$CLOUD_CONTAINER"   OLD_CLOUD_VAULT_TOKEN   "$OLD_TTL"
make_token antenna "$ANTENNA_CONTAINER" OLD_ANTENNA_VAULT_TOKEN "$OLD_TTL"

echo
echo "Waiting 30sec.."
echo
sleep 30

# Phase 2: NEW tokens
echo "Creating NEW tokens"
make_token cloud   "$CLOUD_CONTAINER"   NEW_CLOUD_VAULT_TOKEN   "$NEW_TTL"
make_token antenna "$ANTENNA_CONTAINER" NEW_ANTENNA_VAULT_TOKEN "$NEW_TTL"

echo
echo "Done"