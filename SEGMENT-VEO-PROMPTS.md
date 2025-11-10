# 🎬 SEGMENT GOOGLE VEO - FILM GENERATION MASTERY

**Ostatnia aktualizacja:** 2025-11-10
**Status:** 📚 RESEARCH DONE - Ready for production!

---

## 📊 STATUS PROJEKTU

**Google Veo - AI Video Generation:**
- ✅ Research z oficjalnych źródeł Google completed
- ✅ JSON prompting structure mapped
- ✅ First + Last Frame guide ready
- ✅ Ingredients Mode full breakdown
- ✅ X (Twitter) eksperci + przykłady
- ✅ Templates gotowe do użycia

**Co to jest Google Veo:**
- **Developer:** Google DeepMind
- **Current Version:** Veo 3.1 (October 2025)
- **Capabilities:** Text-to-video, Image-to-video, First+Last frame, Ingredients, Native audio
- **Resolution:** 720p, 1080p HD (up to 4K in Veo 2)
- **Length:** 4-60 seconds (extendable to 148s)
- **Access:** Gemini API, Vertex AI, Google Labs Flow

---

## 🎯 QUICK MENU - VEO VERSIONS

### Timeline (Wersje modelu):

**Veo 1** (May 2024)
- Legacy model
- 1080p, 1+ minute videos
- Basic text-to-video

**Veo 2** (Dec 2024, GA Feb 2025)
- ✅ 4K resolution (4096x2160)
- ✅ **First + Last Frame** dodane
- ✅ 2+ minute clips
- Status: Available in VideoFX

**Veo 3** (May 2025) 🔥
- ✅ **NATIVE AUDIO** generation (dialogue, SFX, ambient)
- ✅ 720p/1080p, 8-second clips
- ✅ "Silent era of video generation" ended - Demis Hassabis
- ✅ 73 countries
- Status: Current production model

**Veo 3.1** (Oct 2025) 🚀
- ✅ **Up to 60 seconds** continuous footage
- ✅ Extendable to **148 seconds** total
- ✅ Enhanced audio (natural conversations)
- ✅ **4 reference images** support (was 3)
- ✅ 24fps, 30fps, 60fps
- Status: **NAJNOWSZY** - paid preview

**Veo 3 Fast / Veo 3.1 Fast**
- Szybsze generowanie
- Niższa cena
- Idealne do iteracji

---

## 📝 DWA RODZAJE PROMPTÓW

### 1. NATURAL LANGUAGE (Prosty tekst)

**Official Google Formula:**
```
[Cinematography] + [Subject] + [Action] + [Context] + [Style]
```

**Długość:** 3-6 zdań / 100-150 słów

**Przykład (oficjalny Google):**
```
A cinematic shot of a female doctor in a dark yellow hazmat suit,
illuminated by harsh fluorescent light, with the camera slowly
zooming in on her face as she peers intently into a microscope.
```

**Drugi przykład:**
```
The camera floats gently through rows of pastel-painted wooden
beehives, settling on a farmer in a white beekeeping suit who
lifts a jar of honey, with tall sunflowers swaying in the background.
```

---

### 2. JSON FORMAT (Zaawansowany - "Przejebany")

**Discovery:** Community discovery (July 2025) - nie oficjalny ale mega działa!

**Korzyści:**
- "Surgical-precision video generation"
- Dramatyczna poprawa kontroli vs tekst
- Precyzyjna kontrola każdego elementu: visual, sound, environment
- Reusable templates
- Brand consistency

**Cytat eksperta (@mikefutia):**
> "VEO 3 + JSON prompting is pretty wild. This JSON prompting technique
> will take any generic VEO3 prompt.. And turn it into surgical-precision
> video generation with brand consistency."

**WAŻNE:** To nie jest oficjalny format ale Veo 3 reaguje na niego świetnie!

---

## 🔥 FIRST + LAST FRAME FEATURE

### Kiedy dodano:
- **Veo 2** (December 2024) - first frame
- **Veo 3.1** (October 2025) - **last frame support dodany**

### Jak działa:

**Koncept:**
Generujesz **płynny transition** między 2 obrazami (start + koniec) + prompt opisujący przejście.

**Upload:**
1. First frame image (obowiązkowy)
2. Last frame image (opcjonalny)
3. Text prompt (opis transicji)

**Veo tworzy:** Natural video przejście między klatkami z audio

---

### Użycie w Google Flow:

**Kroki:**
1. Otwórz Flow (labs.google/fx/tools/flow)
2. Wybierz **"Frames to Video"** mode
3. Upload **first frame** image
4. Upload **last frame** image (optional)
5. Napisz prompt: "A smooth camera movement revealing the transformation"
6. Generate video

---

### JSON Format (API):

```json
{
  "instances": [{
    "prompt": "A smooth camera movement revealing the transformation",
    "image": {
      "bytesBase64Encoded": "BASE64_FIRST_FRAME",
      "mimeType": "image/jpeg"
    },
    "lastFrame": {
      "bytesBase64Encoded": "BASE64_LAST_FRAME",
      "mimeType": "image/jpeg"
    }
  }],
  "parameters": {
    "durationSeconds": 8,
    "aspectRatio": "16:9",
    "resolution": "1080p"
  }
}
```

---

### Use Cases:

✅ **Product reveals** - Close-up → Full product view
✅ **Character transformations** - Before → After
✅ **Scene transitions** - Day → Night, Indoor → Outdoor
✅ **Camera movements** - Specific start/end perspectives
✅ **Action sequences** - Precise beginning/ending poses

**Przykład prompt:**
```
"Dynamic camera orbit around the product, starting with a close-up
of metallic details and ending with a full 360-degree view against
a gradient background."
```

---

### Ograniczenia:

⚠️ Niektórzy użytkownicy raportują że video **zaczyna i kończy** się dokładnie na podanych frames, ale **środek transicji** nie zawsze jest naturalny - jest pole do poprawy.

**Oficjalna dokumentacja:**
https://cloud.google.com/vertex-ai/generative-ai/docs/video/generate-videos-from-first-and-last-frames

---

## 🧩 INGREDIENTS MODE (Składniki)

### Co to jest:

**"Ingredients"** = modular control feature w Google Flow.

Generujesz **osobne elementy** (characters, objects, backgrounds, styles) i **łączysz je** w spójne sceny dla **consistent visual storytelling**.

---

### Ile składników:

- **Veo 2 & Veo 3:** Do **3 reference images** jako "asset ingredients"
- **Veo 3.1:** Do **4 reference images** (upgraded!)
- **ALT:** 1 style reference image (zamiast assets)

---

### Co możesz łączyć:

**OPTION 1: Multiple Assets (Postaci/Obiekty)**
```
Person + Object + Scenery
2 People + Vehicle
Character + Prop + Background
3 Separate Characters
```

**OPTION 2: Style Reference**
```
1 Style Image + Text Prompt
```

**OPTION 3: Mixed Approach**
```
Character (Image 1) + Environment (Image 2) + Style (Image 3)
```

---

### Przykład Real-World:

**Inputs:**
- Image 1: A woman (osoba)
- Image 2: A lava lamp (obiekt)
- Image 3: A foggy street (sceneria)

**Prompt:**
```
"The woman, whose torso is the lava lamp, walks down the foggy street."
```

**Output:** Video łączący wszystkie 3 elementy w spójną scenę!

---

### Jak używać w Flow:

**Kroki:**
1. Otwórz projekt w Google Flow
2. W prompt box wybierz **"Ingredients to Video"**
3. Kliknij **"Add"** → upload ingredient images (do 3/4)
4. Napisz text prompt opisujący jak ingredients mają interaktować
5. Generate video

---

### Tworzenie Ingredients:

**Method 1:** Upload własne zdjęcia
**Method 2:** Generate z **Imagen** (Flow's text-to-image)
**Method 3:** Save perfect frames z wygenerowanych videos jako assets

**Pro Tip:** Używaj **plain backgrounds** dla subjects/products. Location i style references nie powinny zawierać dodatkowych subjects.

---

### JSON Format:

```json
{
  "referenceImages": [
    {
      "bytesBase64Encoded": "CHARACTER_IMAGE_BASE64",
      "mimeType": "image/jpeg"
    },
    {
      "bytesBase64Encoded": "OBJECT_IMAGE_BASE64",
      "mimeType": "image/jpeg"
    },
    {
      "bytesBase64Encoded": "BACKGROUND_IMAGE_BASE64",
      "mimeType": "image/jpeg"
    }
  ],
  "prompt": "Description of how these elements interact in the scene"
}
```

---

### Use Cases:

✅ **Character consistency** across multiple shots (serial content)
✅ **Brand consistency** in commercial videos (logo, colors, style)
✅ **Product demonstrations** with consistent styling
✅ **Series production** - recurring characters/elements
✅ **Marketing campaigns** - unified visual identity

**Przykład - Product Video:**
```
Ingredients:
- Product (iPhone) - Image 1
- Brand colors (gradient) - Image 2
- Lifestyle setting (minimalist desk) - Image 3

Prompt: "The iPhone rotates slowly on the desk with gradient lighting
reflecting off its metallic surface, revealing all angles."
```

---

### Best Practices:

✅ Maintain **consistent look and feel** across all ingredient images
✅ Use **segmented backgrounds** for better blending
✅ **Plain backgrounds** dla subjects/products
✅ Location/style references **bez extra subjects**
✅ Test seeds for reproducibility

---

## 📂 INNE TRYBY GENEROWANIA

### 1. TEXT-TO-VIDEO (Bez obrazu)

**Opis:** Czysty text prompt → video

**Kiedy używać:** Najbardziej elastyczny, najmniej kontrolowany

**Przykład:**
```
"A bustling dystopian sprawl with bright neon signs, rain-slicked streets,
and hover cars zipping past towering skyscrapers at night."
```

---

### 2. IMAGE-TO-VIDEO (Jedno zdjęcie)

**Dodano:** Veo 3 Fast (July 2025), enhanced w 3.1

**Opis:** Animate static image + optional text prompt

**Użycie:**
- Imagen-generated images lub uploads
- Horizontal (16:9) i vertical (9:16) support
- Maintains first frame consistency

**API Format:**
```json
{
  "instances": [{
    "prompt": "The coffee swirls and steam rises slowly",
    "image": {
      "bytesBase64Encoded": "BASE64_IMAGE_DATA",
      "mimeType": "image/jpeg"
    }
  }],
  "parameters": {
    "resizeMode": "pad",
    "durationSeconds": 8
  }
}
```

---

### 3. VIDEO EXTENSION (Video-to-Video)

**Feature Name:** "Scene Extension" / "Extend"

**Opis:** Extend poprzednio wygenerowane Veo videos do **148 seconds total**

**Jak działa:**
1. Generate initial video (8s)
2. Select **"Extend"** option
3. Provide continuation prompt
4. Model uses **last second** jako transition point
5. Output = original + extended video

**Limitacja:** Audio continuity ograniczona do ostatniej sekundy

**Use case:** Długie sekwencje, storytelling, multi-shot videos

---

### 4. VIDEO EDITING Z MASKAMI (Preview)

**Status:** Preview stage

**Możliwości:**
- **Add objects/characters** do existing scenes
- **Remove objects** z background reconstruction
- Google automatycznie obsługuje shadows, lighting, integration

**Coming soon:** Full release

---

## 🎨 JSON PROMPT STRUCTURE - FULL BREAKDOWN

### Podstawowa filozofia:

> "Think of JSON prompting like giving the AI a shot list
> and creative brief in one."

---

### COMPLETE FIELD LIST:

#### **TOP-LEVEL**

```json
{
  "version": 3,
  "prompt": "Main scene description",
  "negative_prompt": "Avoid...",
  "config": {},
  "output": {},
  "seed": 12345,
  "global_style": "Cinematic"
}
```

---

#### **CAMERA**

```json
{
  "camera": {
    "motion": "slow tracking shot",
    "type": "dolly shot",
    "angle": "low angle",
    "lens_type": "50mm",
    "lens": "35mm wide-angle",
    "focalLength": "medium",
    "model": "handheld",
    "framing": "medium close-up",
    "shot": "wide establishing",
    "camera_movement": "360-degree orbit"
  }
}
```

**Camera Movement Options:**
- Dolly, tracking, crane shots
- Aerial views, slow pans, push-in
- POV, handheld, static, tripod
- 360-degree orbit, circular reveal

**Camera Angles:**
- Eye-level, low angle, high angle
- Dutch angle, bird's eye, worm's eye
- Over-the-shoulder

**Lens Types:**
- 18mm (ultra wide), 24mm, 35mm
- 50mm (standard), 85mm (portrait)
- Macro, telephoto
- Shallow depth of field, deep focus

---

#### **SUBJECT/CHARACTER**

```json
{
  "subject": {
    "primary": "A 30-year-old woman with curly hair",
    "secondary": "A golden retriever",
    "emotion": "contemplative",
    "pose": "standing with arms crossed",
    "appearance": {
      "age": "30s",
      "hair": "curly black",
      "clothing": "business casual",
      "style": "minimal"
    },
    "actions": ["walking", "speaking", "gesturing"]
  }
}
```

---

#### **ENVIRONMENT/SETTING**

```json
{
  "scene": "Modern coffee shop interior",
  "environment": {
    "location": "urban cafe",
    "details": "exposed brick walls, vintage furniture",
    "time_of_day": "late afternoon"
  },
  "setting": "Bustling downtown street corner"
}
```

---

#### **LIGHTING**

```json
{
  "lighting": {
    "mood": "soft and warm",
    "time_of_day": "golden hour",
    "type": "natural window light",
    "quality": "high contrast",
    "effects": "rim lighting, backlit"
  }
}
```

**Lighting Options:**
- **Natural:** Golden hour, blue hour, midday, overcast
- **Artificial:** Neon, fluorescent, candlelight
- **Mood:** Soft, harsh, dramatic, moody
- **Effects:** Backlit, rim lighting, silhouette
- **Quality:** High key, low key, high contrast

---

#### **AUDIO** (Veo 3+ ONLY)

```json
{
  "audio": {
    "primary": "dialogue",
    "dialogue": "Character says, 'Hello world'",
    "ambient": ["traffic sounds", "light cafe music"],
    "sound_effects": ["footsteps", "door closing"],
    "music": "upbeat jazz"
  },
  "generate_audio": true
}
```

**Audio Types:**
- **Dialogue** - use quotation marks for specific words
- **Sound effects** - specific actions
- **Ambient** - background atmosphere
- **Music** - genre/mood description

**Pro Tip:** Veo 3+ has NATIVE AUDIO - zawsze opisuj sounds dla best results!

---

#### **STYLE & COLOR**

```json
{
  "style": "Cinematic realism",
  "base_style": "Film noir aesthetic",
  "color_palette": "Warm oranges and deep blues",
  "colorGrading": "Desaturated with teal shadows",
  "fx": {
    "effects": "light bloom",
    "particles": "dust motes in sunbeams"
  }
}
```

---

#### **TECHNICAL OUTPUT**

```json
{
  "config": {
    "duration_seconds": 8,
    "aspect_ratio": "16:9",
    "resolution": "1080p",
    "fps": 24,
    "generate_audio": true,
    "quality": "high"
  }
}
```

**Aspect Ratios:** 16:9 (landscape), 9:16 (portrait)
**Resolutions:** 720p, 1080p (4K w Veo 2)
**FPS:** 24 (cinematic), 30 (standard), 60 (smooth - Veo 3.1)
**Duration:** 4, 6, 8 seconds (Veo 3), up to 60s (Veo 3.1)

---

### COMPLETE WORKING EXAMPLES:

#### **EXAMPLE 1: Simple Product Demo**

```json
{
  "prompt": "Barista making latte art in a cozy coffee shop",
  "camera": {
    "motion": "slow tracking shot from side angle",
    "lens_type": "50mm",
    "framing": "medium close-up of hands and cup"
  },
  "lighting": {
    "mood": "warm and inviting",
    "time_of_day": "morning light through windows"
  },
  "audio": {
    "primary": "milk frothing sounds",
    "ambient": ["quiet cafe chatter", "light jazz music"]
  },
  "config": {
    "duration_seconds": 8,
    "aspect_ratio": "16:9",
    "resolution": "1080p",
    "generate_audio": true
  }
}
```

---

#### **EXAMPLE 2: Cinematic Scene**

```json
{
  "scene": "A lone astronaut stands on the Martian surface, gazing at Earth",
  "style": "Cinematic science fiction",
  "camera": {
    "shot": "Wide establishing shot",
    "movement": "slow zoom-in on astronaut",
    "lens": "35mm wide-angle"
  },
  "lighting": {
    "mood": "Soft, ambient glow from distant sun",
    "quality": "High contrast with deep shadows"
  },
  "audio": {
    "ambient": ["gentle wind", "soft electronic hum"],
    "sound_effects": ["breathing in helmet"]
  },
  "color_palette": "Red and orange Martian hues with blue Earth",
  "config": {
    "duration_seconds": 8,
    "aspect_ratio": "16:9",
    "fps": 24
  }
}
```

---

#### **EXAMPLE 3: Product Showcase (360° Orbit)**

```json
{
  "metadata": {
    "prompt_name": "iPhone Product Launch",
    "base_style": "Modern commercial"
  },
  "subject": {
    "primary": "Sleek black smartphone on minimalist desk",
    "appearance": {
      "color": "matte black",
      "finish": "premium metallic"
    }
  },
  "camera": {
    "type": "Dynamic product showcase",
    "lens": "50mm macro",
    "movement": "360-degree orbital reveal with close-up details"
  },
  "lighting": {
    "mood": "Dramatic studio lighting",
    "effects": ["rim lighting", "gradient backdrop"]
  },
  "environment": {
    "location": "Minimalist studio",
    "details": "Gradient background charcoal to white"
  },
  "audio": {
    "primary": "Subtle tech sounds",
    "ambient": ["quiet electronic hum"],
    "music": "Modern minimal electronic"
  },
  "color_grading": "High contrast, desaturated background, vibrant product",
  "config": {
    "duration_seconds": 8,
    "resolution": "1080p",
    "fps": 24,
    "quality": "lossless"
  }
}
```

---

#### **EXAMPLE 4: Multi-Shot Sequence (Timestamps)**

```json
{
  "prompt": "Coffee brewing sequence from start to finish",
  "shots": [
    {
      "timestamp": "00:00-00:02",
      "description": "Close-up of coffee grounds poured into filter",
      "camera": "Overhead macro shot",
      "audio": "Pouring sound, paper rustling"
    },
    {
      "timestamp": "00:02-00:04",
      "description": "Water pouring over grounds, bloom develops",
      "camera": "Side angle close-up",
      "audio": "Water pouring, gentle bubbling"
    },
    {
      "timestamp": "00:04-00:06",
      "description": "Coffee dripping into carafe",
      "camera": "Low angle watching stream",
      "audio": "Dripping intensifies"
    },
    {
      "timestamp": "00:06-00:08",
      "description": "Steam rising from freshly poured cup",
      "camera": "Medium shot, slight pull-back",
      "audio": "Final drips, ambient cafe sounds"
    }
  ],
  "global_style": "Warm cinematic documentary",
  "lighting": {
    "mood": "Natural morning light",
    "quality": "Soft with steam backlit"
  },
  "config": {
    "duration_seconds": 8,
    "aspect_ratio": "16:9",
    "resolution": "1080p"
  }
}
```

---

## 🌐 X (TWITTER) EXPERTS - REAL EXAMPLES

### Top Veo Eksperci:

**@mikefutia** - Mike Futia
- AG1 product videos entirely with Veo 3 + JSON
- Quote: "surgical-precision video generation with brand consistency"
- https://x.com/mikefutia/status/1951282585235066933

**@Diesol** - Dave Clark
- Comprehensive Veo 3 prompting guide
- Character consistency techniques (Midjourney + Veo)
- https://x.com/Diesol/status/1926300507397599412

**@CurieuxExplorer** - Dev Khanna
- Complex physics simulations (ferrofluid)
- Mumbai street scenes with cultural specificity
- https://x.com/CurieuxExplorer/status/1948391932490908057

**@RichKleinAI** - Rich Klein
- Cartoon animation JSON prompts
- https://x.com/RichKleinAI/status/1965867367851864079

**@azed_ai** - Amira Zairi
- Product showcase (vehicles), 360° orbits
- https://x.com/azed_ai/status/1949139427156279335

**@icreatelife** - Kris Kashtanova
- Tutorial JSON from scratch
- Commercial/ad video creation
- https://x.com/icreatelife/status/1949172310864281719

**@Veo3JSONPrompt** - Dedicated Resource
- Toolbox for structured prompts
- Website: veo3jsonprompt.com

---

### Real JSON Example z X (Dev Khanna):

**Mumbai Monsoon Scene:**
```json
{
  "meta": {
    "styleName": "MumbaiMonsoonRealism",
    "aspectRatio": "16:9",
    "seed": 83291
  },
  "camera": {
    "model": "static waist-height tripod",
    "lens": "35mm",
    "focalLength": "medium"
  },
  "subject": {
    "primary": "A stylish Indian woman in traditional saree walking through rain"
  },
  "setting": "Busy Mumbai street during monsoon season",
  "lighting": "Overcast natural light with neon reflections on wet pavement",
  "fx": "Heavy rain, puddles, ambient street sounds"
}
```

---

### Real JSON Example (Amira - Product):

```json
{
  "shot": {
    "composition": "dynamic product showcase with close-ups and wide reveals, ending in full 360-degree orbit",
    "lens": "50mm",
    "frame_rate": "24fps",
    "camera_movement": "360-degree rotating reveal"
  }
}
```

---

## 🎓 BEST PRACTICES

### Official Google Guidance:

**Core Formula:**
```
Subject + Context + Action + Style + Cinematography
```

**Optimal Length:** 3-6 sentences, 100-150 words

---

### Audio Integration (Veo 3+):

✅ **Explicitly define sounds** you want
✅ Use **quotation marks** for specific dialogue
✅ Describe **sound effects** clearly: "thunder cracks"
✅ Define **ambient soundscape**: "quiet hum of starship bridge"
✅ Pair audio with visual for **multi-sensory** experience

**Przykład:**
```json
{
  "audio": {
    "dialogue": "The captain says, 'Engage warp drive'",
    "sound_effects": ["console beeps", "engines powering up"],
    "ambient": ["low hum of spaceship", "distant stars"]
  }
}
```

---

### Camera Control:

**Framing:**
- Wide shots, close-ups, extreme close-ups
- Low angles, high angles, eye-level
- Two-shots, over-the-shoulder

**Movement Specifics:**
- "Slow tracking shot following the subject"
- "Crane shot revealing the landscape"
- "Handheld documentary style"

**Lens:**
- "18mm lens" for wide shots
- "50mm" for natural perspective
- "Shallow depth of field" for isolation
- "Macro lens" for extreme detail

---

### Character Consistency Hack (Dave Clark):

**Workflow:**
1. Create character reference w **Midjourney**
2. Use as **first frame** w Veo 3
3. Add **"End Scene Immediately"** at start of prompt
4. Maintains character throughout video
5. Use same character across multiple generations

---

### Negative Prompting:

**Less Effective:**
```
"No buildings"
```

**More Effective:**
```
"A desolate landscape stretching to horizon with only natural rock formations"
```

**Zasada:** Opisz co CHCESZ zamiast co NIE CHCESZ.

---

### Iterative Process:

**Official Recommendation:**
> "The first prompt rarely yields a perfect result."

**Workflow:**
1. Start with core concept
2. Generate initial version
3. Analyze output
4. Add progressive detail layers
5. Use **seed** for controlled experiments
6. Refine until satisfied

---

### Technical Optimization:

**For Best Quality:**
- 1080p resolution (when available)
- 24fps for cinematic feel
- 16:9 aspect ratio (most compatible)
- Enable audio generation (Veo 3+)
- Use "lossless" compression for final

**For Fast Iteration:**
- Veo 3 Fast / Veo 3.1 Fast
- 720p resolution
- 4-6 second duration for testing

---

### Common Pitfalls:

❌ **Vague:** "Nice lighting"
✅ **Specific:** "Warm golden hour sunlight streaming through windows"

❌ **Missing Audio:** Not describing sound (Veo 3+)
✅ **With Audio:** "Dialogue, footsteps, ambient cafe music"

❌ **Overcomplicated First Try:** All details at once
✅ **Simple Start:** Core concept → add layers

❌ **No Cinematography:** Generic description
✅ **Camera Work:** "Slow dolly shot, 50mm lens, low angle"

---

## 📈 PRICING & ACCESS

### Pricing Breakdown:

**Gemini API:**
- Veo 2: $0.35/second
- Veo 3: $0.75/second (video + audio)
- Veo 3.1: $0.75/second

**Vertex AI:**
- Veo 2: $0.50/second
- Veo 3: $0.50/sec (video only), $0.75/sec (with audio)
- Veo 3.1: Same as Veo 3

**Subscription Plans:**
- **Google AI Pro:** $19.99/month - 90 Veo 3.1 Fast videos
- **Google AI Ultra:** $249.99/month - ~2,500 Veo 2 or ~1,250 Veo 3.1 Fast

---

### Access Methods:

**1. Google Labs (Flow/VideoFX)**
- Consumer access
- Requires Google AI Pro/Ultra subscription
- Browser-based filmmaking
- US only initially (expanding)

**2. Gemini API**
- Developer access via Google AI Studio
- Get API key at: ai.google.dev
- Python: `pip install google-generativeai`

**3. Vertex AI**
- Enterprise access
- Google Cloud Platform required
- Production-scale deployment

**4. Gemini App**
- Quick video generation
- 73 countries
- Integrated into Gemini chat
- Subscription required for Veo 3+

---

## 🔗 RESOURCES & LINKS

### Official Documentation:
- **DeepMind Veo:** https://deepmind.google/models/veo/
- **Prompt Guide:** https://deepmind.google/models/veo/prompt-guide/
- **Vertex AI Docs:** https://cloud.google.com/vertex-ai/generative-ai/docs/video/overview
- **Gemini API:** https://ai.google.dev/gemini-api/docs/video
- **Ultimate Guide:** https://cloud.google.com/blog/products/ai-machine-learning/ultimate-prompting-guide-for-veo-3-1

### Tools:
- **Google Labs Flow:** https://labs.google/fx/tools/flow
- **Google AI Studio:** https://aistudio.google.com
- **Veo JSON Tool:** veo3jsonprompt.com

### Community:
- **GitHub Prompting:** https://github.com/snubroot/Veo-3-Prompting-Guide
- **GitHub JSON Examples:** https://github.com/snubroot/Veo-JSON
- **X Eksperci:** Zobacz sekcję X (Twitter) Experts powyżej

---

## ✅ WORKFLOW - JAK PRACUJEMY

### Proces tworzenia video:

**1. WYBIERZ TRYB:**
- Text-to-video (czysty prompt)
- Image-to-video (animate 1 foto)
- First + Last Frame (2 obrazy + transition)
- Ingredients (2-4 składniki + kombinacja)
- Video Extension (extend existing)

**2. PRZYGOTUJ MATERIAŁY (jeśli potrzeba):**
- First/last frame images
- Ingredient assets (characters, objects, style)
- Reference images

**3. STWÓRZ PROMPT:**

**Natural Language:**
```
[Cinematography] + [Subject] + [Action] + [Context] + [Style]
```

**JSON Format:**
```json
{
  "prompt": "Core description",
  "camera": {...},
  "subject": {...},
  "lighting": {...},
  "audio": {...},
  "config": {...}
}
```

**4. GENERATE:**
- Upload do Flow / API call
- Wait for generation (usually <2 min)
- Review output

**5. ITERUJ (jeśli trzeba):**
- Use same **seed** for consistency
- Adjust specific parameters
- Add progressive detail
- Test variations

**6. EXTEND (jeśli potrzeba):**
- Use "Scene Extension"
- Provide continuation prompt
- Extend up to 148s total

---

## 🎯 QUICK TEMPLATES LIBRARY

### Template 1: Product Showcase

```json
{
  "prompt": "[PRODUCT NAME] rotating on minimalist surface",
  "camera": {
    "movement": "360-degree orbit",
    "lens": "50mm macro"
  },
  "lighting": {
    "mood": "Dramatic studio lighting",
    "effects": ["rim lighting"]
  },
  "audio": {
    "ambient": ["subtle tech sounds"]
  },
  "config": {
    "duration_seconds": 8,
    "aspect_ratio": "16:9",
    "resolution": "1080p"
  }
}
```

### Template 2: Character Introduction

```json
{
  "prompt": "[CHARACTER] walks into frame with confident stride",
  "camera": {
    "shot": "Medium tracking shot",
    "movement": "Follow character"
  },
  "lighting": {
    "type": "Natural daylight",
    "mood": "Bright and energetic"
  },
  "audio": {
    "sound_effects": ["footsteps"],
    "ambient": ["city sounds"]
  },
  "config": {
    "duration_seconds": 8,
    "aspect_ratio": "16:9"
  }
}
```

### Template 3: Landscape Reveal

```json
{
  "prompt": "Sweeping vista of [LOCATION] at [TIME OF DAY]",
  "camera": {
    "shot": "Wide aerial establishing",
    "movement": "Slow drone push forward"
  },
  "lighting": {
    "time_of_day": "golden hour",
    "mood": "Warm and epic"
  },
  "audio": {
    "ambient": ["wind", "nature sounds"],
    "music": "Cinematic orchestral"
  },
  "config": {
    "duration_seconds": 8,
    "aspect_ratio": "16:9"
  }
}
```

---

## 🔥 NASTĘPNE KROKI

### Co teraz:

1. ⏳ **Wybierz use case** (product, character, landscape, etc)
2. ⏳ **Wybierz tryb** (text, image, frames, ingredients)
3. ⏳ **Stwórz prompt** (natural lub JSON)
4. ⏳ **Generate w Flow** lub via API
5. ⏳ **Iteruj** based on output
6. ⏳ **Extend** if needed (up to 148s)

### Gotowe do produkcji:
- ✅ All modes explained (5 total)
- ✅ First + Last Frame guide
- ✅ Ingredients Mode breakdown
- ✅ JSON structure complete
- ✅ X experts + examples
- ✅ Templates ready to use
- ✅ Best practices from Google

**Ready to create some 🎬 videos!**

---

**Ostatnia aktualizacja:** 2025-11-10
**Status:** 📚 Complete Veo knowledge base - ready for production use!
