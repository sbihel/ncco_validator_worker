# 👷🦀🕸️  WASM NCCO Validator

[Cloudflare worker](https://workers.cloudflare.com/) to validate
[NCCOs](https://developer.nexmo.com/voice/voice-api/ncco-reference).

## 📠 Usage

```bash
$ curl -X POST \
    -H "Content-Type: application/json" \
    --data '[ { "action": "notify", "payload": { "foo": "bar" }, "eventUrl": [ "https://example.com/webhooks/event" ], "eventMethod": "POST" } ]' \
    -i https://ncco_validator.sbihel.workers.dev/
HTTP/2 200
```

---

Created with https://github.com/cloudflare/rustwasm-worker-template
