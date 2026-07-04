# Why This Matters — Us vs the Incumbents

The four established timber CAD/CAM packages — **Cadwork, Dietrich's, SEMA, HSB (hsbCAD)** — already work. So the point of building anything is not to copy them. It's to do the parts they *don't*.

## What we agree on (the shared core)

All four — and us — model a part the same way:

> **A finished part = raw stock + an ordered list of operations.**
> That list is what becomes the machine file (BTL / BTLx / BVX).

On this level, we are building the same *kind* of thing. This is not our differentiation. It's table stakes.

## Where we differ (the reason to exist)

There are two real differences, and they are the entire value.

### 1. How the operations list gets filled

| | Incumbents | Us |
|---|-----------|-----|
| **Primary method** | **Author** — the designer places a joint, the software records the cut directly. It never has to look at a finished shape and reverse-engineer it. | **Recognise** — take an arbitrary 3D model and *work out* what cuts produce that shape. |
| Cadwork | Analyses geometry at export — but geometry the user made *inside Cadwork*. | — |
| HSB | Reconstructs from IFC, then hands it back to the user to confirm. | Recognition is first-class, not a confirm-it-yourself import. |
| The hard path | Largely **avoided**. | Made **first-class** (via OpenCASCADE + timber feature recognition). |

If all we did was re-author timber joints in an app, we'd be rebuilding Dietrich's — thirty years behind. Ingesting *foreign* geometry and recognising it is the thing they sidestep.

### 2. The shape of the system

| | Incumbents | Us |
|---|-----------|-----|
| **Form** | Single fat **desktop** apps. Timber-first monoliths. | Cloud-native, split system. |
| Geometry kernel | Embedded machinery inside the app. | A **separate geometry server** you call over RPC. |
| Front end | The desktop app itself. | **Light edge** (browser / Cloudflare Workers). |
| The operations list | Internal to the app. | An **open, browser-reachable, synced spine** ("op log") that many inputs write to and many machines read from. |
| Outputs | Timber CNC (BTL/BVX). | One list → **wood CNC (BTL) *and* steel roll-former (Howick)** and beyond. |

## The one-line positioning

> **We agree with the incumbents on the data model, and differ on how the list gets filled and how the system is shaped.**

The two things that make it worth building are the two things they don't do:

1. **Ingest foreign geometry and recognise it** — no re-authoring required.
2. **Keep the list as an open, browser-reachable spine** — many inputs in, many machines out, not locked to one tool or one machine.

## Why that's valuable (in plain terms)

- **No lock-in at the front:** a model from anywhere (IFC/STEP from any architect or engineer) can enter — not just parts drawn inside one vendor's desktop app.
- **No lock-in at the back:** the same list drives different machines (timber CNC today, steel framing tomorrow), because the machine format is a translation of the shared list, not the source of truth.
- **Reachable and modern:** browser-native and cloud-hosted, not a per-seat desktop install — the list is a live spine, not a file trapped in one workstation.
- **A real moat:** the recognition step (turning arbitrary geometry into timber operations) is something *very few tools do at all*. That's the defensible part — not the kernel, not the file format, but the recognition layer on top.
