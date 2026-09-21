#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$ROOT/target/debug/crane"
DEMO="$(mktemp -d)"
trap 'rm -rf "$DEMO"' EXIT
cd "$DEMO"
git init -q
git config user.email crane@example.com
git config user.name "Crane Demo"
cat > GatewayService.java <<'JAVA'
class GatewayService {
    public void call() {
        System.out.println("payment");
    }
}
JAVA
git add GatewayService.java
git commit -qm "trusted baseline"
"$BIN" init
"$BIN" checkpoint --name baseline
"$BIN" protect --function GatewayService.call --policy payment_gateway
"$BIN" check >/tmp/crane_pass.txt
grep -q "Crane check: PASS" /tmp/crane_pass.txt
python3 - <<'PY'
from pathlib import Path
p=Path("GatewayService.java")
p.write_text('''class GatewayService {\n    public void call() {\n        System.out.println("modified");\n    }\n}\n''')
PY
if "$BIN" check >/tmp/crane_fail.txt 2>&1; then
  echo "expected check to fail"
  exit 1
fi
grep -q "FAIL payment_gateway" /tmp/crane_fail.txt
grep -q "Crane check: FAIL" /tmp/crane_fail.txt
echo "Smoke test passed."
