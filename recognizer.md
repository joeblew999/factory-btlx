# BTLx 2.3.1 — Recognizer Build Backlog

Classification of all 54 BTLx `ProcessingElements` operations by **how you recover them from geometry** in an OCCT + AAG recognition pipeline.

Three buckets:

- **B1 — Direct from a kernel primitive.** OCCT/AAG detects the shape; a semantic *rule* assigns the BTL label. Runs in the native recognizer service.
- **B2 — Bespoke timber recognizer.** Compound/oblique joints with no mechanical analog. You author these. This is the moat. Runs in the native recognizer service.
- **B3 — Not geometrically recoverable.** Authored/semantic intent (fastening paths, marks, zones, meta). No recognizer can recover these. Source = IFC/BIM property data or your authoring layer (edge/Rust world).

Counts: **B1 = 22, B2 = 19, B3 = 13.**

A core nuance for B1: several ops share ONE primitive (a prismatic cavity, or a planar clip) and differ only by a labeling rule. The detection is cheap; the disambiguation is the work.

---

## B1 — Direct from a kernel primitive (22)

### Planar-clip family → one primitive, label by rule
Primitive: a plane (or planar-bounded region) separating/removing material. OCCT: `BRepAdaptor_Surface` → `GeomAbs_Plane`.

| Op | Label rule (disambiguates the plane primitive) |
|----|------------------------------------------------|
| `SawCut` | Generic planar cut; default when no more specific rule fires. |
| `JackRafterCut` | Single plane at an end face (5/6), skew to the beam axis. |
| `LongitudinalCut` | Plane roughly parallel to the axis, running along the length (rip). |
| `RidgeValleyCut` | Angled plane at an end forming a ridge/valley bevel. |
| `DoubleCut` | Two adjacent end planes meeting at a line (point/vee at the end). |
| `TriangleCut` | Planar-bounded triangular removal. |

### Cavity family → one primitive, label by rule
Primitive: a prismatic (or thin) cavity. OCCT: closed void with planar walls; classify by through/blind, footprint, aspect, position.

| Op | Label rule |
|----|-----------|
| `Pocket` | Blind rectangular cavity on a face; general case. |
| `Slot` | Thin cavity (small `Thickness`), through or blind; slot proportions. |

*(`Lap`, `Mortise`, `House` share this primitive but are labeled by timber semantics → see B2 note.)*

### Single-primitive detectors

| Op | Primitive / OCCT cue |
|----|----------------------|
| `Drilling` | Cylindrical face → `GeomAbs_Cylinder`. Read axis, `Diameter`, `Depth`. |
| `Chamfer` | Small bevel face on an edge; chamfer-chain along adjacent edges. |
| `Sphere` | Spherical face → `GeomAbs_Sphere`. |
| `RoundArch` | Circular/arc profile bounding a removal. |
| `ProfileCambered` | Arc/cambered profile along the member. |
| `Planing` | Thin inward offset of a whole reference face (surface dressing). |
| `FreeSurface` | Non-analytic (NURBS/BSpline) face → `GeomAbs_BSplineSurface`. |

### Contour family → extract the path; tool type is policy, not geometry
Primitive: a bounded wire (polyline + arcs) on a face → OCCT `TopExp` wire extraction. You **cannot** tell saw vs mill from the finished shape — the tool is a manufacturing choice, so assign by policy/config.

| Op | Note |
|----|------|
| `FreeContour` | Generic contour removal following an extracted wire. |
| `SawContour` | Same geometry; tool = saw (policy). |
| `MillContour` | Same geometry; tool = mill (policy). |
| `ProfileFront` | Shaped contour at an end face; extract wire, classify light. |
| `ProfileHead` | Shaped contour at the head; extract wire. |

---

## B2 — Bespoke timber recognizer (19) — the moat

Each needs a custom recognizer on the AAG / OCCT primitives. Cue = the geometric signature to match. All are anchored to the 6-face reference frame; most are 2.5D relative to one face.

### Notch / seat joints

| Op | Recognition cue |
|----|-----------------|
| `BirdsMouth` | Re-entrant V-notch open to one longitudinal face: two interior planes (seat + heel) meeting at a line across the member width; does not sever the member. |
| `HipValleyRafterNotch` | Birdsmouth pattern **plus** a second (compound) bevel — non-zero inclination on both axes. |
| `StepJoint` | Oblique bearing seat cut into a longitudinal face at the rafter/tie angle; heel and/or toe step, load-bearing front/back notch. |
| `StepJointNotch` | The notch-only component of a step joint. |

### Cavity ops that are labeled by timber semantics (share the B1 cavity primitive)

| Op | Recognition cue |
|----|-----------------|
| `Lap` | Full-width reduction of the cross-section over a length (half-lap), typically at an end or a crossing. Prismatic cavity + full-width rule. |
| `Mortise` | Rectangular blind/through cavity proportioned to receive a tenon; pair with a `Tenon` on the mating member. |
| `House` | Full-width shallow trench (dado) across a longitudinal face to seat a crossing member. |
| `HouseMortise` | `House` trench combined with a mortise pocket inside it. |

### Protrusions (recognized via the *complement* — shoulders, not cavities)

| Op | Recognition cue |
|----|-----------------|
| `Tenon` | Residual reduced-section stub projecting to an end face, past a ring/row of planar shoulder faces. Detect the shoulder pattern; the stub is the leftover. |
| `DovetailTenon` | Tenon whose flank walls are non-orthogonal (`FlankAngle` ≠ 0) — trapezoidal in plan. |
| `JapaneseTenon` | Multi-step / hooked compound protrusion (≥2 stepped shoulders or interlock). |

### Trapezoidal (dovetail) cavities & joints

| Op | Recognition cue |
|----|-----------------|
| `DovetailMortise` | Cavity with undercut (angled, wider-at-base) opposing walls. |
| `Dovetail` | Trapezoidal lap/housing with angled flanks. |
| `TyroleanDovetail` | Regional dovetail variant; trapezoidal flanks with characteristic proportions. |
| `JapaneseMortise` | Multi-step compound cavity mating a Japanese tenon. |

### End-to-end lengthening joints

| Op | Recognition cue |
|----|-----------------|
| `ScarfJoint` | End profile replaced by a multi-planar stepped/hooked/tabled surface spanning the full cross-section, designed to mate with a complement. |
| `SimpleScarf` | Single diagonal (bevel) half-depth end lap for lengthening. |
| `FrenchRidgeLap` | Diagonal interlocking half-lap at a ridge with a retaining step. |

### Log construction

| Op | Recognition cue |
|----|-----------------|
| `LogHouseHalfLap` | Half-depth transverse saddle notch on a log at a crossing; seat may be cylindrical (round log). |
| `LogHouseJoint` | Corner interlocking saddle notches for log walls near member ends. |
| `LogHouseFront` | End/front interlocking notch for log courses. |

---

## B3 — Not geometrically recoverable (13) — carry from source

No finished-part geometry encodes these. The recognizer literally cannot produce them. Source = IFC/BIM property sets, fastening/assembly data, or your authoring layer.

| Op | Why it can't be recognized | Source |
|----|----------------------------|--------|
| `Marking` | Layout line; no material removed. | Authoring / BIM |
| `Text` | Text annotation. | Authoring |
| `NailContour` | Nail placement path (fastening intent). | Fastening data / BIM |
| `ScrewContour` | Screw placement path. | Fastening data / BIM |
| `CrampContour` | Cramp/staple path. | Fastening data / BIM |
| `PenContour` | Pen marking path. | Authoring |
| `PatternContour` | Repeated marking/fastening pattern. | Authoring |
| `GlueArea` | Glue zone declaration. | BIM / spec |
| `PlaningArea` | Planing/dressing zone (region intent). | BIM / spec |
| `PlasterArea` | Plaster zone. | BIM / spec |
| `InsulationArea` | Insulation zone. | BIM / spec |
| `LockoutArea` | Keep-out / lockout zone. | Authoring / safety |
| `Variant` | Parametric meta-container, not a feature. | Authoring / model meta |

---

## Where each bucket runs

- **B1 + B2** → native OCCT + AAG recognizer service (behind ConnectRPC/MCP; Hetzner or M4 host). Output: ordered op log.
- **B3** → never from the recognizer. Merge in at the op-log layer from IFC property data or the authoring UI (edge / Rust / Workers).

## Suggested build order

1. **Cavity + planar-clip primitives** (covers most of B1): stock recovery → `Cut(stock, part)` → per-reference-face 2.5D classification → the `{Pocket, Slot, Lap, Mortise, House}` and `{SawCut, JackRafterCut, ...}` label rules. This alone handles the bulk of real framing parts.
2. **Drilling + Chamfer** (trivial primitives, high frequency).
3. **B2 high-frequency joints first:** BirdsMouth, Tenon, Mortise, Lap, StepJoint — the common roof/frame set.
4. **B2 long tail:** dovetails, scarfs, Japanese, log-house — as target markets demand them.
5. **B3 pass-through:** wire IFC Psets / fastening data into the op log; no geometry involved.

## Notes

- The prismatic-stock + 6-face reference frame is what makes B1/B2 tractable — every op is a deviation anchored to a known plane, so recognition is mostly 2.5D-per-face rather than general 3D FR.
- Tenon-class ops are recognized by their **shoulders** (removed material), not the protrusion itself — a generic mechanical pocket recognizer will mislabel them.
- Contour-family tool type (saw/mill/nail/…) is not in the geometry; assign by policy or carry from source.
