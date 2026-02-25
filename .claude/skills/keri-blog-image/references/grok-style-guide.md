# Grok/Aurora Style Guide for KERI.host

Reference file for Phase 4 of the `keri-blog-image` skill. Load this before
assembling the final prompt.

---

## Prompt Engineering for Aurora

### Token Weighting: Early Tokens Rule

Aurora (the model powering Grok Imagine) is autoregressive — it weighs early
tokens more heavily than late ones. This means **style must come first**.

**Rule:** Always lead with the style prefix before the subject description. Never
bury "cinematic photorealism" at the end after describing the scene. The model
will drift toward generic if style tokens come too late.

**Correct order:**
```
[style] [medium] [aesthetic], [subject + action], [environment], [light], [color], [composition], -- no [negatives]
```

**Wrong order:**
```
[subject] in [environment] with [light], [style at the end]
```

### Adjective Nouns, Not Adjective Stacks

Adjective stacking without nouns is one of the most common prompt failures.
Aurora needs concrete visual instructions, not mood words.

| Don't | Do |
|-------|-----|
| "dark moody cinematic dramatic" | "long shadows falling across worn oak boards" |
| "warm cozy atmosphere" | "amber lamplight pooling on a linen tablecloth" |
| "professional clean minimal" | "single object on raw concrete, wide negative space" |
| "interesting texture" | "raw concrete wall with visible aggregate and hairline cracks" |
| "nice lighting" | "side light from a north-facing window, diffused overcast sky" |

### Negative Prompting Syntax

Grok uses `-- no` syntax (not `--negative` or parentheses like other models).

**Format:** append at the very end, comma-separated:
```
-- no glowing screens, floating UI, blue gradients, lens flare, stock photo symmetry, ring light catchlights, corporate office backgrounds, artificial glow
```

### Single Focal Subject

Aurora performs significantly better with one dominant subject than with complex
multi-subject compositions. If the post concept requires multiple elements, choose
the one that best carries the metaphor and let the environment imply the rest.

---

## Style Modifier Library

These descriptors are tested for Aurora compatibility. Use them as building blocks.

### Lighting Descriptors

| Modifier | Effect | Best for |
|----------|--------|---------|
| `natural side light from a north-facing window` | Soft, directional, neutral | Objects, tools, still life |
| `golden hour light raking across the surface` | Warm, dramatic, texture-revealing | Physical metaphors, transitions |
| `overcast morning diffusion, no hard shadows` | Even, contemplative, slightly cool | Reflective/philosophical posts |
| `single incandescent lamp in an otherwise dark room` | Intimate, focused, high contrast | Decision moments, identity |
| `blue hour exterior, warm interior glow through windows` | Threshold, transition, liminal | Change/passage posts |
| `cross-lit from a doorway at 45 degrees` | Architectural, structural | Infrastructure, systems posts |

### Texture Descriptors

| Modifier | Surface |
|----------|---------|
| `raw concrete with visible aggregate and hairline cracks` | Concrete |
| `worn oak workbench with mineral oil finish and tool marks` | Wood (worn) |
| `aged kraft paper with fiber texture visible at edges` | Paper |
| `tanned leather with stress creasing near the fold` | Leather |
| `linen tablecloth with visible weave, slight crease` | Fabric |
| `oxidized brass with green patina at recessed edges` | Metal (aged) |
| `matte ceramic with finger-wiped glaze imperfection` | Ceramic |
| `handmade paper with irregular deckled edges` | Paper (artisan) |

### Film Aesthetic Modifiers

| Modifier | Result |
|----------|--------|
| `scanned 35mm film grain, slight halation on highlights` | Analog warmth |
| `shot on Kodak Portra 400, warm midtones, lifted shadows` | Portrait film warmth |
| `Fuji Velvia cross-process, slight color shift` | Rich, saturated but textured |
| `slightly underexposed negative, pushed in post` | Gritty, real |
| `slight lens vignette, f/1.8 field curvature` | Cinematic imperfection |
| `minor chromatic aberration at high-contrast edges` | Optical authenticity |
| `focus breathing at f/1.4, center sharp, corners fall off` | Artistic shallow DOF |

### Composition Language

| Modifier | Use when |
|----------|---------|
| `subject placed at left third, open right for text` | Hero image with title overlay |
| `subject placed at right third, open left for title` | Alternative layout |
| `centered subject, symmetrical negative space` | Bold, declarative posts |
| `foreground object out-of-focus at left edge` | Depth, layered environment |
| `high horizon line, subject in lower third` | Post about weight, systems, infrastructure |
| `low camera angle, looking slightly up at subject` | Posts about scale, importance |
| `overhead flat lay at 45 degrees` | Still life, tools, documents |
| `intimate 50mm distance, subject fills 60% of frame` | Human moments, identity posts |

---

## Composition Patterns

### For Title Overlay (Most Common)

Hero images need open space where the post title can render without fighting
the image. Always specify which side and approximately how much.

**Rule of thumb:**
- Left third open → title on left, image on right (Western reading direction)
- Top third open → for social cards / OG images
- Never center the subject AND want text overlay — one or the other

**Template addition:**
```
open left third — clear for title text overlay, minimal visual activity in that zone
```

### Foreground/Background Layering

Flat images feel like stock photos. Depth separates editorial photography.

**Three-layer structure:**
1. **Foreground:** out-of-focus element that establishes environment (edge of desk, blurred
   window frame, soft surface texture)
2. **Midground:** the focal subject, sharp
3. **Background:** simplified, slightly soft, contextual (bookshelf, concrete wall, doorway)

**Template addition:**
```
[blurred foreground element] framing a sharp [subject] against a soft [background]
```

### Negative Space Rules

- Negative space must be *textured*, not white/empty — "open concrete wall" not "white background"
- The emptier the space, the more important the texture of that space
- Dark negative space (shadow) reads more editorial; light negative space reads more
  product-like — choose based on tone

---

## Anti-Patterns: What to Avoid in Aurora Outputs

### Over-Smoothness

Aurora can trend toward AI-smooth skin, perfect gradients, and synthetic-looking
surfaces when the prompt is underspecified. Counter this with explicit imperfection:

- Add `slight imperfection` to the style prefix (already included in the KERI.host prefix)
- Specify texture at the noun level (see texture library above)
- Name specific film grain and vignette (see film aesthetic modifiers)

### Stock Photo Symmetry

Aurora learned from the internet, which is full of symmetrical, centered stock photos.
To break this:
- Always use rule-of-thirds subject placement
- Add asymmetric lighting (side-lit vs. front-lit)
- Include at least one off-balance foreground element

**Explicit counter:** add `-- no stock photo composition, no centered symmetrical subjects`
to negative prompts.

### Artificial Glow

Common failure mode: glowing edges, HDR-style halos, oversaturated highlights.
Counter with:
- `muted desaturated palette` (in style prefix)
- `no artificial glow, no HDR processing, no oversaturated highlights` in negatives
- Name shadow behavior explicitly (long and soft) rather than leaving it to the model

### Corporate Office Backgrounds

Aurora defaults to open-plan offices, glass conference rooms, and WeWork aesthetics.
Counter with:
- Always specify a non-corporate environment
- Use workshop, library, kitchen, outdoor market, garage, home office
- If abstract, use concrete, wood, or paper surfaces

---

## KERI.host Brand Vocabulary

Canonical objects and environments for KERI.host posts, with notes on usage and
previous appearances.

### Objects (Confirmed On-Brand)

| Object | Symbolic Weight | Usage Notes |
|--------|----------------|-------------|
| **Physical key** | Access, identity, control | Use for identity/authentication posts. Avoid skeleton keys (too cliché). Prefer old brass house key or industrial key. |
| **Wallet** | Value, custody, portability | Physical leather wallet, worn, slightly open. Not digital wallet UI. |
| **Door / doorway** | Threshold, access, transition | Partially open, light coming through. Good for change/passage posts. |
| **Handshake** | Trust, agreement, peer relationship | Close-up, slightly off-angle. No power-suit sleeves. Work-worn hands preferred. |
| **Letter / document** | Record, attestation, proof | Typed or handwritten on aged paper. Folded or sealed. |
| **Envelope (sealed)** | Privacy, containment, delivery | Wax seal, kraft paper, or plain white — not corporate manila. |
| **Stamp / ink mark** | Attestation, finality, authority | Rubber stamp on paper, ink still wet. |
| **Padlock** | Security, commitment | Open or closed changes meaning. Closed = secured. Open = access granted. |
| **Notebook / journal** | Record-keeping, provenance, ledger | Worn leather cover, pages visible. Not Moleskine-perfect. |
| **Blueprint** | Architecture, plan, infrastructure | Unrolled on a work surface. Slightly yellowed. |
| **Plumb line / level** | Alignment, truth, calibration | Tool metaphor for consensus/accuracy. |
| **Telegraph / old phone** | Communication infrastructure, protocol | Good for posts about messaging layers, wire protocols. |
| **Filing cabinet drawer** | State, stored history, retrieval | Slightly open, papers visible. |
| **Compass** | Navigation, orientation | On a worn map or bare surface. Physical compass, not digital. |
| **Clay / wax seal** | Identity assertion, cryptographic commitment | Close-up of seal pressed into wax. Very on-brand for KERI. |

### Environments (Confirmed On-Brand)

| Environment | Tone | Best for |
|-------------|------|---------|
| **Workshop / maker space** | Craft, intentionality, labor | Technical/infrastructure posts |
| **Home office with window** | Human, independent, thoughtful | Identity, sovereignty posts |
| **Small library / reading room** | Knowledge, continuity, record | Specification, protocol posts |
| **Outdoor market stall** | Community, exchange, trust | Economy, community posts |
| **Concrete loading dock** | Industrial, unglamorous, real | Infrastructure, gritty technical posts |
| **Kitchen table with papers** | Domestic, real, non-corporate | Human impact, practical posts |
| **Building threshold / lobby** | Transition, access, identity check | Authentication, access control posts |

### Canonical Color Palette

| Role | Color | Hex (approximate) |
|------|-------|------------------|
| Primary earth tone | Warm concrete / oatmeal | `#C8B99A` |
| Wood/leather anchor | Dark walnut | `#5C3D2E` |
| Indigo accent | Deep indigo shadow | `#2D3560` |
| Paper highlight | Aged linen | `#EDE8DF` |
| Shadow fill | Charcoal brown (not black) | `#2C2420` |

**Rule:** Never pure black shadows. Use deep brown or indigo. Never pure white highlights.
Use aged linen or warm cream.

---

## Quick Reference Checklist

Before finalizing any prompt, verify:

- [ ] Style prefix comes first, before any subject description
- [ ] One dominant focal subject, not a crowd scene
- [ ] Specific texture named at noun level (not just "concrete" but "raw concrete with visible aggregate")
- [ ] Light source direction and quality named explicitly
- [ ] Rule-of-thirds subject placement specified
- [ ] Negative space side specified with "open [left/right] third for title overlay"
- [ ] Negative prompts appended with `-- no` syntax
- [ ] No adjective stacks without nouns
- [ ] No corporate office or digital UI in the environment
- [ ] At least one brand vocabulary object or environment present
- [ ] Film grain and imperfection explicitly included
