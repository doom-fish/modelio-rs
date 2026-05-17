#!/bin/bash
set -u
log_file="publish-modelio.log"
sleep_until_hms() {
  python3 - "$1" <<'PY'
from datetime import datetime, timedelta, timezone
import sys
retry_hms = sys.argv[1]
now = datetime.now(timezone.utc)
retry = datetime.strptime(now.strftime('%Y-%m-%d') + ' ' + retry_hms, '%Y-%m-%d %H:%M:%S').replace(tzinfo=timezone.utc)
if retry <= now:
    retry += timedelta(days=1)
delay = int((retry - now).total_seconds()) + 120
print(max(delay, 1200))
PY
}
first_retry_hms="${1:-}"
if [ -n "$first_retry_hms" ]; then
  delay=$(sleep_until_hms "$first_retry_hms")
  echo "initial-sleep ${delay}s $(date -u +"%Y-%m-%dT%H:%M:%SZ")" >> "$log_file"
  sleep "$delay"
fi
while true; do
  ts=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
  echo "attempt $ts" >> "$log_file"
  out=$(cargo publish 2>&1)
  status=$?
  printf '%s\n' "$out" >> "$log_file"
  if [ $status -eq 0 ]; then
    if ! git rev-parse -q --verify refs/tags/v0.1.0 >/dev/null; then
      git tag v0.1.0 >> "$log_file" 2>&1 || exit 1
    fi
    git push origin v0.1.0 >> "$log_file" 2>&1 || exit 1
    echo "success $(date -u +"%Y-%m-%dT%H:%M:%SZ")" >> "$log_file"
    exit 0
  fi
  retry_hms=$(printf '%s\n' "$out" | sed -n 's/.*Please try again after .* \([0-9][0-9]:[0-9][0-9]:[0-9][0-9]\) GMT.*/\1/p' | tail -n1)
  if [ -n "$retry_hms" ]; then
    delay=$(sleep_until_hms "$retry_hms")
  else
    delay=1800
  fi
  echo "sleeping ${delay}s $(date -u +"%Y-%m-%dT%H:%M:%SZ")" >> "$log_file"
  sleep "$delay"
done
