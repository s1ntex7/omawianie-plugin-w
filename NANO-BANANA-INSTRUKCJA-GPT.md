# 🎨 NANO BANANA PROMPT GUIDE - GPT INSTRUCTIONS

**Model:** Gemini 2.5 Flash Image (Nano Banana) | **Źródło:** Google Official Docs 2025

---

## CORE PRINCIPLE

**"Describe the scene, don't just list keywords"**

Nano Banana trenowany na video - rozumie kontekst, fizykę, relacje. **Narracyjne opisy > keyword listy.**

---

## OFFICIAL TEMPLATE

```
A photorealistic [shot type] of [subject], [action/expression], set in [environment].
The scene is illuminated by [lighting], creating a [mood] atmosphere.
Captured with [camera/lens], emphasizing [details].
```

**Elementy:**
- **Subject** - główny obiekt/osoba
- **Context** - otoczenie, tło
- **Style** - photorealistic, cinematic, noir, painterly, impressionist
- **Action verbs** - leaping, flowing, cascading, emerging (dla dynamiki)
- **Camera language** - shot types, lens, lighting
- **Details** - tekstury, nastrój, kompozycja

---

## DO's ✅

**1. Narracyjne opisy:**
```
❌ "forest, sunset, deer, fog"
✅ "A red deer emerges from morning fog in ancient pine forest, backlit by golden sunrise"
```

**2. Action verbs (trenowany na video):**
```
❌ "woman in garden"
✅ "woman walking through garden, dress flowing in breeze"
```

**3. Język fotograficzny:**
- **Shots:** close-up, wide-angle, aerial, macro, establishing shot
- **Lens:** 24mm wide, 85mm portrait, 100mm macro, fisheye
- **Lighting:** golden hour, soft diffused, harsh shadows, rim light, chiaroscuro
- **Settings:** bokeh, motion blur, shallow depth of field

**4. Hyper-specific:**
```
❌ "fantasy armor"
✅ "ornate elven plate armor etched with silver leaf, falcon wing pauldrons"
```

**5. Kontekst i intent:**
```
❌ "Create a logo"
✅ "Logo for high-end minimalist skincare brand, soft curves, earth tones"
```

---

## DON'Ts ❌

**1. Nie keyword listów:**
❌ "sunset, mountains, lake, dramatic, beautiful, 4K"

**2. Nie negatywów instrukcyjnie:**
❌ "no walls, don't show frame"
✅ "clean background, minimal composition"

**3. Limity tekstu w obrazie:**
- Max 25 znaków per tekst
- Max 2-3 frazy
- Fonty: ogólny styl ("bold sans-serif") nie dokładna nazwa

**4. Zawsze podawaj styl:**
❌ "woman in room"
✅ "photorealistic portrait, minimalist room, soft window light, contemplative"

---

## QUICK TEMPLATES

### 🎬 Cinematic/Noir
```
A photorealistic [shot] of [subject], [action], set in [environment].
Noir-style cinematic lighting with harsh shadows and [light source],
creating dramatic atmosphere. Captured with [lens], emphasizing [details].
```

### 👤 Character/Portrait
```
A [shot] portrait of [character], [expression], set against [background].
Lighting is [type], highlighting [features]. [Clothing]. [Mood].
```

### 🌆 Landscape
```
A [shot] of [location], during [time/weather]. Illuminated by [light],
with [foreground] leading to [background]. [Atmosphere].
```

### 📸 Product Photo
```
Professional product photo of [product], placed on [surface], shot with [lighting].
Emphasizes [features], captured with [lens]. [Brand elements].
```

### 🖼️ Artistic
```
A [art style] painting of [subject], [action], set in [environment].
Uses [color palette] and [technique], creating [mood] atmosphere.
```

### ✏️ Concept Art
```
Concept art of [subject], [pose], in [setting]. Art style is [technique],
with [color scheme] and [detail level]. For [use case].
```

---

## IMAGE EDITING (Official Google Guidelines)

**KEY DIFFERENCE: Editing ≠ Generation**

**Generation:** Descriptive narrative (create from scratch)
**Editing:** Imperative commands + what to preserve (transform existing)

**EDITING TEMPLATE:**
```
[ACTION VERB] + [TARGET] + [TO/WITH WHAT] + [PRESERVATION CONSTRAINT]
```

**Action Verbs (Official):**
- **Replace** - backgrounds, objects ("Replace background with sunny beach. Keep person unchanged.")
- **Remove** - objects, people ("Remove door mirror")
- **Add** - elements ("Add flower beds with vibrant blooms in front")
- **Change** - attributes ("Change red ball to blue box")
- **Make** - transformations ("Make landscape snowy and mountainous")

**Preservation Keywords:**
- "Keep [X] the same"
- "Preserve [X]"
- "Maintain [X]"
- "Do not alter [X]"

**Multi-Turn Editing Workflow:**
```
Turn 1: Major change (background/subject)
Turn 2: Lighting/atmosphere
Turn 3: Details/refinement
Turn 4: Polish
```

**Editing Examples:**
```
✅ "Replace background with foggy graveyard at night. Keep person's face and clothing identical."
✅ "Remove stain from t-shirt. Preserve fabric texture and lighting."
✅ "Change walls to blue. Maintain all furniture and lighting unchanged."
✅ "Blur background with f/1.8 bokeh. Keep subject in sharp focus."
```

**Common Editing Mistakes:**
```
❌ "Change background" (co z osobą?)
✅ "Replace background with beach. Keep person, pose, clothing same."

❌ "Make it better" (jak?)
✅ "Increase contrast 30%, add warm golden hour lighting from left"

❌ "Don't change the face" (negatyw)
✅ "Preserve face, skin tone, features. Change only background."
```

**Pro Tips:**
- Break complex edits into steps (not all at once)
- Always specify what to preserve
- Use technical language (f/1.8, 6500K, bokeh)
- Save version copies at each major iteration
- Review results before next turn

---

## ADVANCED FEATURES

**Character Consistency:** Utrzymuje tę samą postać w wielu obrazach.
- First: "A photorealistic portrait of [unique features]..."
- Next: "The same [character], now [new action] in [new scene]. Maintain [features]."

**Multi-Image Fusion (max 3):** "Merge [element A] with [element B]. Subject from A should be [action] in environment from B."

**Aspect Ratios:** 1:1, 3:2, 2:3, 3:4, 4:3, 4:5, 5:4, 9:16, 16:9, 21:9. Dodaj: "in 16:9 aspect ratio"

---

## WORKFLOW

1. User podaje temat/scenę
2. AI tworzy: Base prompt + Variant 1 (inny styl) + Variant 2 (inna intensywność) + Variant 3 (experimental)
3. User testuje w Nano Banana
4. Iteracja: "zmień X, zostaw Y, dodaj Z"

---

## PRZYKŁADY

**Film Noir:**
```
A photorealistic close-up of a reaper in mist, face partly visible under hood,
holding massive scythe. Noir-style lighting with harsh shadows and soft fog,
mysterious atmosphere. 85mm portrait lens, emphasizing weathered fabric and
metallic blade sheen.
```

**Landscape:**
```
A red deer leaping through morning fog in ancient pine forest, backlit by golden
sunrise filtering through trees. Warm diffused light with god rays, magical
atmosphere. 70-200mm telephoto at 200mm, emphasizing dynamic motion and ethereal fog.
```

**Portrait:**
```
A medium shot of young woman with auburn hair, smiling gently looking away,
blurred cafe background. Natural window light from left, highlighting freckles
and warm skin tone. Cream turtleneck sweater. Comfortable contemplative mood.
```

---

## COMMON MISTAKES

❌ "beautiful landscape" → ✅ "misty mountain valley at sunrise with wildflowers and winding river"
❌ "person in room" → ✅ "photorealistic portrait in minimalist modern room, natural window light, contemplative"
❌ "forest scene" → ✅ "ancient forest at golden hour, sunlight filtering through canopy, dappled shadows on moss"
❌ "woman standing" → ✅ "woman walking gracefully, dress flowing, turning to look over shoulder"

---

## QUICK REFERENCE CARD

**Structure:** [style] [shot] of [subject], [action], set in [environment]. [lighting] creating [mood]. Captured with [camera]. [details].

**Photo terms:** close-up, wide-angle, macro, aerial | 24mm, 85mm, 100mm | golden hour, rim light, bokeh

**Action verbs:** leaping, flowing, cascading, emerging, walking, turning, reaching, gazing

**Moods:** mysterious, dramatic, tranquil, energetic, contemplative, nostalgic, ethereal

**Styles:** photorealistic, cinematic, noir, impressionist, pop art, concept art, minimalist

**Remember:** Narrative > keywords. Be specific. Add action. Use photo language. Iterate.

---

**Access:** Google AI Studio (aistudio.google.com) Free | API $0.039/image
