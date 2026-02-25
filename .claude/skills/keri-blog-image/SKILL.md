---
name: keri-blog-image
description: >
  Generate a Grok/Aurora-optimized hero image prompt for a blog post, using a
  multi-phase ideation process grounded in a specific brand aesthetic: cinematic
  photorealism, warm earth tones, physical metaphors, anti-corporate tone.
  Use this skill whenever the user wants to create a hero image, banner image,
  or cover image for a blog post, article, or written piece — even if they just
  say "make me an image for this post" or "what would a good hero image be for
  this article". Always use this skill when blog post content is provided
  alongside an image request.
user_invocable: true
command: keri-blog-image
---

# Blog Hero Image Prompt Generator

Generates a polished, brand-consistent Grok image prompt for a blog post in
four phases: **Extract → Ideate → Refine → Synthesize**.

Work through all four phases sequentially. Show your reasoning at each phase
so the user can redirect before you finalize.

---

## Brand DNA (internalize this before starting)

**Voice**: "Infrastructure for the rest of us" — human, grounded, anti-corporate.
**Aesthetic**: Minimal Mistakes "air" skin — light, clean, lots of white space.
**Stance**: Anti-hype. Physical metaphors over digital abstraction. Philosophical about technology and human dignity.

**Never use**: glowing orbs, floating UI, blue-gradient tech abstractions, holographic
hands, circuit board overlays, lens flares on screens, robots with human emotions,
or any image that says "startup deck slide."

**Always favor**: things you can hold, worn surfaces, rooms with windows, tools with
patina, objects with a history.

---

## Style Prefix (append verbatim to every final prompt)

```
Cinematic photorealism, physical tangible objects, warm earth tones with deep
indigo accent, natural directional light casting long shallow shadows, 35mm film
grain, shallow depth of field f/1.8, muted desaturated palette, raw concrete or
worn wood surfaces, slight imperfection —
```

Read `references/grok-style-guide.md` for the full library of Grok-specific
modifiers, composition patterns, and what to avoid. Load it before Phase 4.

---

## Phase 1: Content Extraction

Read the blog post carefully. Extract a structured content brief:

```json
{
  "topic": "one sentence — what the post is actually about",
  "thesis": "the core argument or insight the author is making",
  "tone": "one of: [reflective, technical, cautionary, optimistic, philosophical, instructional]",
  "audience": "who this is written for and what they care about",
  "keyMetaphors": ["any vivid metaphors or analogies the author already uses"],
  "emotionalGoal": "what feeling should the reader leave with",
  "avoidVisually": ["anything in the post that would be wrong to show literally"]
}
```

Show this brief to the user before proceeding. It's the foundation everything
else rests on — if it's wrong, the image will be wrong.

---

## Phase 2: Visual Concept Ideation

Generate **3 distinct visual concepts** using the content brief. Each concept
must use a different visual strategy from this list:

| Strategy | When to use |
|---|---|
| **Symbolic object** | Post is abstract — find one physical thing that holds the whole idea |
| **Human moment** | Post is about people, relationships, dignity, labor |
| **Environmental tension** | Post is about contrast, transition, or systems |
| **Still life** | Post is instructional or about craft — tools, materials, process |
| **Threshold/passage** | Post is about change, decisions, beginning/ending |

For each concept write:
- **Scene**: What's physically in the frame, where, how lit
- **Why it works**: 1–2 sentences connecting it back to the thesis
- **Emotion**: One word
- **Risk**: What could go wrong or read ambiguously

Score each concept 1–10 on: **Brand fit / Emotional resonance / Compositional clarity**.

Present all three. Ask the user to pick one, or note which you'd recommend and why.

---

## Phase 3: Composition Refinement

Take the selected concept and add specificity across five dimensions:

**Composition**
- Subject placement: rule of thirds, off-center left or right?
- Foreground / midground / background layers
- Negative space for title text overlay (which side, how much)
- Aspect ratio: always 16:9 for hero images unless specified

**Light**
- Source direction (side light, backlight, window light)
- Quality: golden hour / overcast morning / blue hour / interior lamp
- Shadow behavior: long and soft, short and hard, diffused
- Never: overhead studio light, ring light catchlights, artificial green tones

**Color**
- Earth tone anchor: concrete / leather / wood / aged paper / linen
- Indigo accent: where does it appear (shadow edge, background wall, object detail)?
- What to desaturate vs. allow to be rich

**Film aesthetic**
- Lens: 35mm or 50mm prime (never wide angle distortion)
- Depth of field: f/1.4–2.8 shallow, subject sharp, background painterly
- Grain: visible but not distracting — "scanned 35mm" not "Instagram filter"
- Slight imperfection: lens vignette, minor chromatic aberration, not clinical

**Metaphor grounding**
- Name the specific object or scene element that carries the meaning
- How does it connect to the brand vocabulary?
  (Keys, wallets, doors, handshakes, paper, physical infrastructure — things you can hold)

---

## Phase 4: Grok Prompt Synthesis

Load `references/grok-style-guide.md` now.

Assemble the final prompt using this structure:

```
[STYLE PREFIX] [SUBJECT + ACTION/STATE], [ENVIRONMENT], [LIGHT DESCRIPTION],
[COLOR NOTES], [COMPOSITION NOTES], [NEGATIVE SPACE NOTE], [FILM AESTHETIC],
[WHAT TO AVOID]
```

**Grok-specific rules:**
- Lead with the style prefix *before* the subject (Aurora weights early tokens heavily)
- Single focal subject performs better than multi-subject compositions
- Be explicit about negative space: *"open left third for title overlay"*
- Negative prompting in Grok: append `-- no [thing], [thing], [thing]` at the end
- Avoid adjective stacking without nouns — "dark moody cinematic dramatic" does nothing; "long shadows falling across a worn oak workbench" does everything
- Specify texture explicitly: *"raw concrete wall, visible aggregate"* not just *"concrete"*
- Aurora handles imperfection well — lean into it: *"slight lens vignette, visible grain"*

**Output format:**

---
### Hero Image Prompt

> [Final prompt — 60–120 words]

**Negative prompts:** `no glowing screens, no UI overlays, no blue gradients, no lens flare, no stock photo composition, no artificial lighting`

**Aspect ratio:** `16:9`

**Optional variants:**
- Tighter crop version (for mobile / social thumbnail)
- Alternative metaphor if first is too abstract

---

After presenting the prompt, briefly explain the one key creative decision made
— why this image and not the others — in 2–3 sentences. This helps the user
understand whether the interpretation landed right.
