#!/usr/bin/env bash
set -euo pipefail

SCREENSHOT=${1:-artifacts/ui-snapshot.png}
ROOT_DIR=$(cd -- "$(dirname "$0")/.." && pwd)
cd "$ROOT_DIR"

if [[ ! -f "$SCREENSHOT" ]]; then
    echo "[validate] Screenshot not found: $SCREENSHOT"
    echo "[validate] Run ./scripts/capture_ui.sh first"
    exit 1
fi

if [[ -z "${OPENAI_API_KEY:-}" ]]; then
    echo "[validate] ERROR: OPENAI_API_KEY environment variable not set"
    exit 1
fi

echo "[validate] Encoding screenshot..."
IMAGE_BASE64=$(base64 -w0 "$SCREENSHOT")

echo "[validate] Calling OpenAI Vision API..."
RESPONSE=$(curl -s https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d "{
    \"model\": \"gpt-4o\",
    \"messages\": [{
      \"role\": \"user\",
      \"content\": [
        {
          \"type\": \"text\",
          \"text\": \"This is a screenshot of a Nest Learning Thermostat UI clone built in Rust. Please analyze this interface and compare it to the original Nest Thermostat design. Describe what you see and highlight any visual differences, missing elements, or areas that need improvement to match the authentic Nest look and feel. Be specific about colors, layout, typography, and UI elements.\"
        },
        {
          \"type\": \"image_url\",
          \"image_url\": {
            \"url\": \"data:image/png;base64,$IMAGE_BASE64\"
          }
        }
      ]
    }],
    \"max_tokens\": 1000
  }")

echo "[validate] Response received"
echo ""

# Extract and display the content
echo "$RESPONSE" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    if 'error' in data:
        print('ERROR:', data['error'].get('message', data['error']))
        sys.exit(1)
    content = data['choices'][0]['message']['content']
    print('=== OpenAI Vision Feedback ===')
    print(content)
    print()
except Exception as e:
    print('Failed to parse response:', e)
    print('Raw response:', data if 'data' in locals() else 'N/A')
    sys.exit(1)
"

# Save response to file
RESPONSE_FILE="${SCREENSHOT%.png}.json"
echo "$RESPONSE" > "$RESPONSE_FILE"
echo "[validate] Full response saved to $RESPONSE_FILE"
