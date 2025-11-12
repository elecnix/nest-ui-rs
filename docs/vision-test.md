# UI Screenshot Validation Flow

This document describes how to capture a simulator frame and validate it with OpenAI's vision API.

## Prerequisites
- `OPENAI_API_KEY` environment variable set to a valid key.
- `curl` and `base64` installed (available on Ubuntu by default).

## 1. Capture the UI Screenshot

From the project root (`nest-ui-rs/`):

```bash
./scripts/capture_ui.sh artifacts/ui-snapshot.png
```

This script builds the simulator in release mode and writes the current frame to `artifacts/ui-snapshot.png`.

## 2. Inspect Locally (Optional)

```bash
xdg-open artifacts/ui-snapshot.png
```

## 3. Send to OpenAI Vision API

The API accepts either a file upload as base64 data or a publicly available URL. The snippet below inlines the PNG as base64.

```bash
IMAGE64=$(base64 -w0 artifacts/ui-snapshot.png)

curl https://api.openai.com/v1/responses \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d "{
    \"model\": \"gpt-4o-mini\",
    \"input\": [{
      \"role\": \"user\",
      \"content\": [
        {\"type\": \"input_text\", \"text\": \"Describe this Nest Thermostat UI screenshot. Highlight any mismatches with the original Nest interface.\"},
        {\"type\": \"input_image\", \"image_base64\": \"$IMAGE64\"}
      ]
    }]
  }"
```

The response will be a JSON payload describing what the model sees. Store the JSON alongside the PNG for future regressions (e.g., `artifacts/ui-snapshot.json`).

## 4. Automating in CI (Optional)

Wrap the capture and API call in a shell script or Makefile target. Failing the pipeline when the vision API reports mismatches keeps the UI aligned with the Nest look and feel.
