#!/bin/bash
# serve_web.sh — serves the web/ build at http://localhost:8000

cd "$(dirname "$0")/web" || exit 1
echo "Serving at http://localhost:8000"
python3 -m http.server 8000
