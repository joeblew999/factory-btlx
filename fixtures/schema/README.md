# BTLx schemas — provenance

The BTLx XML Schema Definitions, downloaded verbatim from the standard's home,
[design2machine](https://www.design2machine.com/btlx/schema.html). BTLx is the open
timber-fabrication interchange format, maintained by SEMA + CadWork.

| File | Source | Notes |
|------|--------|-------|
| `BTLx_2_3_1.xsd` | <https://www.design2machine.com/btlx/BTLx_2_3_1.xsd> | latest schema; what our serialiser targets |
| `BTLx_2_0_0.xsd` | <https://www.design2machine.com/btlx/BTLx_2_0_0.xsd> | the version real files in the wild actually use |
| `BTLx_2_3_1.offline.xsd` | **generated** from `BTLx_2_3_1.xsd` | the canonical XSD `<xs:include>`s an external X3D schema (fetched over the network), which blocks offline `xmllint`. We never emit `<Shape>` — the only X3D user — so this copy has the include, the `Shape` element and the now-unused `ShapeType` stripped, and validates offline. |

## Regenerate

```sh
mise run schema
```

Downloads the canonical XSDs from design2machine and rebuilds the offline copy. Pure
nushell (cross-platform, no `curl`/`sed`) — see `[tasks.schema]` in
[`../../mise.toml`](../../mise.toml).
