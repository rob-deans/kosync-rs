# Koreader Sync Server

 [![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

Koreader devices can register their devices to the sync server and sync the progress cross device.

Original: <https://github.com/koreader/koreader-sync-server>

## Setup

```bash
docker run -d -p 3000:3000 -v kosync-data:/app/data ghcri.io/rob-deans/kosync-rs:latest
```

API server is up and running at `localhost:3000` with the database persisted.

## Connecting

1. Open a document in KOReader on a device you want to sync
2. Settings > Progress Sync > Custom sync server
3. Enter URL
4. Enter username and password and select "Register" to create an account
5. Test with "Push progress from this device now"
6. Enable automatic progress
7. On another device, login using the credentials
8. Test with "Pull progress from other devices now"

---

Made for fun without AI
