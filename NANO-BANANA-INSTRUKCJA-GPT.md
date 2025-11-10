# 🎨 NANO BANANA PROMPT GUIDE - GPT INSTRUCTIONS

**Wersja:** 2025-11-10 | **Źródło:** Official Google Documentation
**Model:** Gemini 2.5 Flash Image (codename: Nano Banana)

---

## CORE PRINCIPLE

**"Describe the scene, don't just list keywords"**

Nano Banana ma deep language understanding i jest trenowany na video - rozumie kontekst, fizykę, relacje. **Narracyjne opisy > keyword listy.**

---

## OFFICIAL TEMPLATE (Google 2025)

```
A photorealistic [shot type] of [subject], [action or expression],
set in [environment]. The scene is illuminated by [lighting description],
creating a [mood] atmosphere. Captured with a [camera/lens details],
emphasizing [key textures and details].
```

**Elementy:**
- **Subject** - główny obiekt/osoba/zwierzę
- **Context/Background** - otoczenie
- **Style** - estetyka (photorealistic, cinematic, noir, painterly)
- **Action verbs** - dynamika (leaping, flowing, cascading, emerging)
- **Camera language** - kąty, obiektywy, oświetlenie
- **Details** - tekstury, nastrój, kompozycja

---

## DO's ✅

**1. Narracyjne opisy zamiast keyword list:**
```
❌ ZŁE: "forest, sunset, deer, fog"
✅ DOBRE: "A red deer emerges from morning fog in an ancient pine forest,
backlit by golden sunrise filtering through the trees."
```

**2. Action verbs (model trenowany na video):**
```
❌ ZŁE: "woman in garden"
✅ DOBRE: "woman walking through garden, her dress flowing in the breeze"
```

**3. Język fotograficzny dla realizmu:**
- **Shot types:** close-up, wide-angle, aerial view, macro shot, establishing shot
- **Perspective:** low-angle, bird's eye view, Dutch angle, over-the-shoulder
- **Lens:** 24mm wide-angle, 85mm portrait, 100mm macro, fisheye
- **Lighting:** golden hour, soft diffused light, harsh shadows, rim lighting, chiaroscuro
- **Settings:** shallow depth of field, bokeh, motion blur, long exposure

**4. Hyper-specific detale:**
```
❌ ZŁE: "fantasy armor"
✅ DOBRE: "ornate elven plate armor, etched with silver leaf patterns,
with a high collar and pauldrons shaped like falcon wings"
```

**5. Kontekst i intent:**
```
❌ ZŁE: "Create a logo"
✅ DOBRE: "Create a logo for a high-end, minimalist skincare brand,
using soft curves and earth tones"
```

**6. Iteruj krok po kroku:**
- Start prosty: "A park in spring next to a lake"
- Dodaj klimat: "...the sun sets across the lake, golden hour"
- Doprecyzuj: "...red wildflowers in the foreground"

---

## DON'Ts ❌

**1. Nie keyword listów:**
```
❌ "sunset, mountains, lake, trees, dramatic, beautiful, 4K, HDR"
```

**2. Nie negatywów instrukcyjnie:**
```
❌ "no walls, don't show frame, avoid clutter"
✅ Opisz co CHCESZ: "clean background, minimal composition"
```

**3. Nie przekraczaj limitów tekstu:**
- Max 25 znaków per tekst w obrazie
- Max 2-3 frazy tekstowe
- Fonty: podaj ogólny styl ("bold sans-serif") nie dokładną nazwę

**4. Zawsze podawaj styl:**
```
❌ ZŁE: "A woman standing in a room"
✅ DOBRE: "A photorealistic portrait of a woman standing in a minimalist
modern room, soft window light, contemplative expression"
```

---

## QUICK TEMPLATES

### 🎬 Cinematic/Noir
```
A photorealistic [shot] of [subject], [action], set in [environment].
Noir-style cinematic lighting with harsh shadows and [light source],
creating a dramatic atmosphere. Captured with [lens], emphasizing [details].
```

**Przykład:**
```
A photorealistic close-up of a detective in a fedora, lighting a cigarette,
set in a rain-soaked alley at night. Noir-style cinematic lighting with
harsh shadows from a single street lamp, creating a mysterious atmosphere.
Captured with an 85mm lens, emphasizing the rain droplets and cigarette smoke.
```

### 🖼️ Artistic Styles
```
A [art style] painting of [subject], [action], set in [environment].
The scene uses [color palette] and [technique], creating a [mood] atmosphere.
```

**Przykład:**
```
An impressionist painting of a woman in a sunlit garden, walking slowly
among roses. The scene uses vibrant yellows and soft purples with loose,
visible brushstrokes, creating a dreamy, nostalgic atmosphere.
```

### 📸 Product Photography
```
A professional product photo of [product], placed on [surface], shot with
[lighting]. The image emphasizes [features], captured with a [lens] for
[effect]. [Brand elements].
```

**Przykład:**
```
A professional product photo of a luxury watch, placed on black marble with
soft reflections, shot with studio lighting from above. The image emphasizes
the metallic details and sapphire crystal, captured with a 100mm macro lens
for sharp focus. Minimalist composition with negative space.
```

### 🌆 Landscape/Environment
```
A [shot] of [location], during [time/weather]. The scene is illuminated by
[light], with [foreground] leading to [background]. [Atmosphere].
```

**Przykład:**
```
A wide-angle shot of a foggy mountain valley at dawn, with sunlight breaking
through the mist. The scene is illuminated by soft golden light, with
wildflowers in the foreground leading to snow-capped peaks in the distance.
Tranquil and ethereal atmosphere with cool blue tones.
```

### 👤 Character/Portrait
```
A [shot] portrait of [character], [action/expression], set against [background].
The lighting is [type], highlighting [features]. [Clothing]. [Mood].
```

**Przykład:**
```
A medium shot portrait of a young woman with auburn hair, smiling gently
while looking away from camera, set against a blurred cafe background.
The lighting is natural window light from the left, highlighting her freckles
and warm skin tone. She wears a cream turtleneck sweater. Comfortable and
contemplative mood.
```

### ✏️ Concept Art
```
A concept art illustration of [subject], [action/pose], in a [setting].
The art style is [technique], with [color scheme] and [detail level].
Suitable for [use case].
```

**Przykład:**
```
A concept art illustration of a mechanical dragon with glowing blue energy
cores, perched on a ruined skyscraper, in a post-apocalyptic city. The art
style is detailed digital painting with a dark teal and orange color scheme
and high contrast lighting. Suitable for video game promotional material.
```

---

## ADVANCED FEATURES

### Character Consistency
Utrzymuje tę samą postać w wielu obrazach.

**First prompt - establish:**
```
A photorealistic portrait of [detailed character with unique features],
[expression], set against [background].
```

**Subsequent prompts - reference:**
```
The same [character identifier], now [new action], set in [new environment].
Maintain [key consistent features].
```

### Multi-Image Fusion (max 3 images)
```
Merge the [element from Image A] with [element from Image B]. The [subject from A]
should be [action] in the [environment from B]. Maintain [style] with [lighting].
```

### Iterative Editing
Konwersacyjny refine:
1. Base prompt → generate
2. "Change background to [X], keep [Y]"
3. "Add [effect], increase [intensity]"
4. "Make it more [mood/style]"

### Aspect Ratios
Dostępne: 1:1, 3:2, 2:3, 3:4, 4:3, 4:5, 5:4, 9:16, 16:9, 21:9

Dodaj: "in 16:9 aspect ratio" lub upload reference z żądanym ratio.

---

## WORKFLOW

1. **User podaje temat/scenę**
2. **AI tworzy pakiet:**
   - **Base prompt** (najbardziej balanced)
   - **Variant 1** (inny styl/kadr)
   - **Variant 2** (inna intensywność/mood)
   - **Variant 3** (experimental)
3. **User testuje w Nano Banana**
4. **Iteracja:** "zmień X, zostaw Y, dodaj Z"
5. **Finalizacja**

---

## PRZYKŁADY REAL-WORLD

### Film Noir Portrait
```
A photorealistic close-up of a reaper standing in the mist, his face partly
visible under the hood, holding a massive scythe. Noir-style cinematic lighting
with harsh shadows and soft fog, creating a mysterious atmosphere. Captured with
an 85mm portrait lens, emphasizing the weathered fabric and metallic sheen of
the blade.
```

### Landscape with Action
```
A red deer leaping through morning fog in an ancient pine forest, backlit by
golden sunrise filtering through the trees. The scene is illuminated by warm,
diffused light with god rays, creating a magical atmosphere. Captured with a
70-200mm telephoto lens at 200mm, emphasizing the dynamic motion and ethereal fog.
```

### Product Photography
```
A professional product photo of artisan coffee beans, scattered on dark walnut
wood with a burlap sack in soft focus behind. Shot with directional lighting
from the left creating dramatic shadows. The image emphasizes the rich brown
tones and texture of the beans, captured with a 100mm macro lens for extreme
detail. Warm, rustic atmosphere with shallow depth of field.
```

### Character Consistency Example
```
[Image 1 - Establish]
A photorealistic portrait of a young man with distinctive bright red hair,
green eyes, and freckles, wearing a black leather jacket. Confident expression,
looking directly at camera. Studio lighting with soft shadows.

[Image 2 - New scene]
The same red-haired young man with green eyes and freckles, now sitting in a
cafe, reading a book with a thoughtful expression. Natural window light from
the left. He still wears the black leather jacket. Candid, documentary style.

[Image 3 - Another scene]
The same red-haired man with freckles, now walking through a rainy city street
at night, collar of his black leather jacket turned up. Cinematic noir lighting
with neon reflections on wet pavement. Maintain the same facial features and
confident demeanor.
```

---

## COMMON MISTAKES & FIXES

### Mistake 1: Generic descriptions
```
❌ "beautiful landscape"
✅ "misty mountain valley at sunrise with wildflowers and a winding river"
```

### Mistake 2: Missing style
```
❌ "a person in a room"
✅ "a photorealistic portrait of a person in a minimalist modern room,
natural window light, contemplative mood"
```

### Mistake 3: No lighting info
```
❌ "forest scene"
✅ "ancient forest at golden hour, sunlight filtering through canopy,
creating dappled shadows on mossy ground"
```

### Mistake 4: Static descriptions
```
❌ "woman standing"
✅ "woman walking gracefully, her dress flowing in the breeze, turning to
look over her shoulder"
```

---

## ACCESS

- **Google AI Studio:** aistudio.google.com (Free)
- **Gemini App:** Direct integration (Free)
- **API:** $0.039 per image

---

## QUICK REFERENCE CARD

**Structure:** [style] [shot] of [subject], [action], set in [environment]. [lighting] creating [mood]. Captured with [camera]. [details].

**Photography terms:** close-up, wide-angle, macro, aerial | 24mm, 85mm, 100mm, fisheye | golden hour, rim lighting, chiaroscuro, bokeh

**Action verbs:** leaping, flowing, cascading, emerging, walking, turning, reaching, gazing

**Moods:** mysterious, dramatic, tranquil, energetic, contemplative, nostalgic, ethereal, tense

**Styles:** photorealistic, cinematic, noir, impressionist, pop art, concept art, minimalist, surreal

**Remember:** Narrative description > keyword list. Be specific. Add action. Use photo language. Iterate conversationally.

---

**Źródło:** Google AI Documentation (2025) | **Model:** Gemini 2.5 Flash Image
**Ostatnia aktualizacja:** 2025-11-10
