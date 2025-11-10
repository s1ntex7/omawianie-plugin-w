# 🗿 SEGMENT FIGURKI 3D - KOMENDY & STYLE REFERENCES

**Ostatnia aktualizacja:** 2025-11-10
**Status:** 📚 RESEARCH DONE - Ready for production!

---

## 📊 STATUS PROJEKTU

**3D Figurine Generation:**
- ✅ Research platform (Tencent Hunyuan 3D = najlepszy!)
- ✅ Image reference workflow ready
- ✅ Style reference library planned
- ✅ Pure text prompts (rzeźbiarskie style)
- ✅ 3D printing optimization
- ⏳ Czekam na ir-3d.jpg (pijak z kuflem - main style reference)

**Main Workflow:**
- **Platform:** Tencent Hunyuan 3D (https://3d.hunyuan.tencent.com/)
- **Input:** Zdjęcie osoby X + Style reference (ir-3d.jpg) + Text prompt
- **Output:** 3D model do druku
- **Generation time:** 10-25 seconds
- **Post-process:** FluxContext (optional 2D editing)

---

## 🎯 QUICK MENU - DWA PODEJŚCIA

### A. **IMAGE REFERENCE WORKFLOW** (Główny sposób)
**Koncept:**
```
STYLE REFERENCE: ir-3d.jpg (pijak z kuflem) = styl artystyczny
SUBJECT: Zdjęcie osoby X = kogo przedstawiamy
OUTPUT: Osoba X w stylu pijaka
```

**Use case:** Spójna seria figurek w tym samym stylu rzeźbiarskim

---

### B. **PURE TEXT PROMPTS** (Bez image reference)
**Koncept:**
```
Tylko tekst → 3D model
Różne style: Barokowy, Michał Anioł, Rodin, Art Nouveau, etc.
```

**Use case:** "Fajne komendy na poważnie rzeźbiarskie" - standalone pieces

---

## 📸 IMAGE REFERENCE WORKFLOW

### Main Style Reference: ir-3d.jpg

**Opis stylu (pijak z kuflem):**
- **Estetyka:** Stylizowany, rzeźbiarski, lekko karykaturalny
- **Proporcje:** Realistyczne z akcentem na plastykę ciała
- **Twarz:** Stylizowana, ale zachowująca podobieństwo
- **Tło:** Neutralne, jasno szare, bez scenografii
- **Detale:** Rzeźbiarska faktura ubioru i gestów
- **Mood:** Z humorem ale z klasą

**Status:** ⏳ Do dodania przez użytkownika

---

### Prompt Template (z Image Reference):

**PODSTAWOWY SZABLON:**
```
[Generate a stylized 3D figurine of the person in the photo,
matching the artistic style of the drunk man holding a beer mug].

The character is imagined as [ROLE/CHARACTER].

[DESCRIPTION OF COSTUME/PROPS/POSE]

[EMPHASIZE specific features for clarity]

[Maintain grounded, physically connected design throughout.]
```

---

### PRZYKŁAD PEŁNY (Jedi):

```
[Generate a stylized 3D figurine of the man in the photo,
matching the artistic style of the drunk man holding a beer mug].

The character is imagined as a Jedi warrior.

He wears a dark Jedi robe and holds a blue lightsaber in his right hand.
His left arm is raised high, forming a strong, clear silhouette with a
distinct black boxing glove.

[Emphasize the shape and volume of the glove so it's easily recognized,
even from a distance – round, padded, slightly shiny like real boxing gear.
Maintain grounded, physically connected design throughout.]

Style: Neutral light gray background, no scenery, full figure with visible base.
```

---

### PRZYKŁAD 2 (Pirat):

```
[Generate a stylized 3D figurine of the woman in the photo,
matching the artistic style of the drunk man holding a beer mug].

The character is imagined as a Caribbean pirate captain.

She wears a weathered leather coat with brass buttons, tricorn hat
tilted at a rakish angle, and holds a cutlass in her right hand.
Her left hand rests on her hip where a flintlock pistol is holstered.

[Emphasize the coat's flowing fabric with sculptural folds, the
gleaming cutlass blade, and the confident pirate stance. Ensure
all elements connect physically - coat flows to ground, sword held
firmly, pistol secured to belt.]

Style: Neutral light gray background, no scenery, full figure with
circular base featuring compass rose engraving.
```

---

### KLUCZOWE ZASADY:

**✅ ZAWSZE:**
- Wspomnij "[matching artistic style of drunk man with beer mug]"
- Opisz całą postać (full body, not just torso)
- Podaj "neutral light gray background, no scenery"
- Dodaj "full figure with visible base"
- "Physically connected design" - no floating elements

**❌ UNIKAJ:**
- Pływających elementów (smoke, aura, magic energy)
- Skomplikowanej scenografii (dodasz później w FluxContext)
- Thin fragile parts (łamią się przy druku)
- Overhangs >45° bez supportu

---

## 🎨 STYLE REFERENCE LIBRARY (Do stworzenia)

**Pomysł:** Stwórz kilka style references dla różnych estetyk

### Sugerowane Style References:

**1. ir-3d-classic.jpg** (OBECNY - pijak)
- Styl: Rzeźbiarski, karykaturalny, realistic proportions
- Mood: Humorystyczny z klasą
- Use: Domyślny styl projektu

**2. ir-3d-chibi.jpg** (DO STWORZENIA)
- Styl: Cute, big head (1/3 proportions), Nendoroid-like
- Mood: Słodki, kolekcjonerski
- Use: Anime/manga characters, kawaii aesthetic

**3. ir-3d-realistic.jpg** (DO STWORZENIA)
- Styl: Photorealistic, detailed, porcelain quality (Lladro-style)
- Mood: Elegancki, delikatny
- Use: Serious portraits, heirloom pieces

**4. ir-3d-lowpoly.jpg** (DO STWORZENIA)
- Styl: Geometric, faceted, minimal triangles
- Mood: Modern, abstract
- Use: Contemporary art pieces, minimalist aesthetic

**5. ir-3d-designer.jpg** (DO STWORZENIA)
- Styl: Urban vinyl, Kidrobot/KAWS aesthetic, thick contours
- Mood: Pop culture, street art
- Use: Designer collectibles, art toys

**6. ir-3d-baroque.jpg** (DO STWORZENIA)
- Styl: Classical sculpture, dramatic folds, ornate details
- Mood: Theatrical, romantic
- Use: Historical figures, dramatic poses

---

### Jak Używać Multiple Style References:

**Workflow:**
```
1. Wybierz style reference dla projektu:
   - Classic (pijak) - domyślny
   - Chibi - dla cute characters
   - Realistic - dla elegant portraits
   - Designer - dla pop culture icons

2. Wyślij do Sora.com:
   - Style reference image (np. ir-3d-chibi.jpg)
   - Subject photo (osoba X)
   - Text prompt z "[matching style of ...]"

3. Generate w Tencent Hunyuan 3D

4. Post-process jeśli potrzeba
```

---

## 📝 PURE TEXT PROMPTS (Bez Image Reference)

### Kiedy używać:
- Nie potrzebujesz spójności z innymi figurkami
- Chcesz specyficzny styl rzeźbiarski (Michał Anioł, Rodin)
- Standalone art piece
- Eksperymentowanie ze stylami

---

## 🗿 STYLE LIBRARY - KOMPLETNA LISTA

### SEKCJA 1: GENERAL STYLES (19 stylów)

#### **1. Klasyka** (domyślny - z image reference)
**Opis:** Stylizowany, rzeźbiarski, lekko karykaturalny. Realistyczne proporcje z rzeźbiarską plastyką. Twarz stylizowana ale zachowuje podobieństwo. Neutralne jasno szare tło, brak scenografii.

**Prompt (pure text):**
```
"A stylized sculptural figurine of [character], realistic proportions
with slightly caricatured features, matte painted finish, standing on
neutral gray base, full body visible, no scenery, physically grounded
design, optimized for 3D printing"
```

---

#### **2. Realistyczno-klasyczny (porcelanowa dama)**
**Opis:** Delikatna estetyka Lladro/Rosenthal. Subtelne rysy, pastelowe kolory, realistyczna sylwetka, miękkie formy, naturalna poza.

**Prompt:**
```
"A delicate porcelain-style figurine of [character] inspired by Lladro
aesthetics, subtle facial features, pastel color palette, realistic
proportions with soft flowing forms, natural graceful pose, matte finish
with gentle sheen, standing on oval base, no scenery, museum-quality
sculptural detail"
```

---

#### **3. Chibi / Super Deformed (SD)**
**Opis:** Uproszczony, słodki styl z dużą głową (1/3 proporcji). Duże oczy, przerysowana mimika, zaokrąglone kształty. Styl japońskich figurek.

**Prompt:**
```
"A chibi-style 3D figurine of [character], oversized head approximately
1/3 of total height, large expressive eyes, simplified rounded body
proportions, cute exaggerated expressions, smooth minimalist textures,
standing on round base, Nendoroid aesthetic, no scenery"
```

---

#### **4. Styl barokowy / rzeźba z pałacu**
**Opis:** Bogato zdobiony, dynamiczne fałdy tkanin, ekspresja, światłocień. Klasyczne rzeźby baroku - romantyzm, elegancja, kunszt.

**Prompt:**
```
"A baroque-style sculptural figurine of [character], richly detailed
with dramatic flowing fabric folds, expressive dynamic pose, chiaroscuro
lighting effects in sculptural form, ornate decorative elements, romantic
elegance, classical palace sculpture aesthetic, standing on decorated
pedestal base, marble-like finish"
```

---

#### **5. Disney/Pixar 3D look**
**Opis:** Styl nowoczesnych animacji 3D - duże wyraziste oczy, gładka skóra, bajkowa atmosfera. Łagodne kontury, ekspresyjne emocje.

**Prompt:**
```
"A 3D figurine in Disney/Pixar animation style of [character], large
expressive eyes with detailed iris, smooth stylized skin, gentle rounded
contours, charming personality in pose, vibrant colors, friendly approachable
aesthetic, standing on simple round base, no scenery"
```

---

#### **6. Low-poly geometryczny**
**Opis:** Uproszczone geometryczne formy - trójkąty, proste krawędzie, płaszczyzny. Minimalistyczny, inspirowany grafiką komputerową.

**Prompt:**
```
"A low-poly geometric figurine of [character], constructed from simplified
triangular facets, sharp edges and clean planar surfaces, minimalist modern
aesthetic, visible polygon structure, matte monochrome or gradient colors,
standing on hexagonal base, contemporary art style"
```

---

#### **7. Vinyl designer toy (Urban style)**
**Opis:** Estetyka Kidrobot, Bearbrick, Dunny. Mocno stylizowana forma - uproszczona twarz, wyraźne kontury, grubsze proporcje.

**Prompt:**
```
"A designer vinyl toy figurine of [character] in urban collectible style,
simplified bold contours, thick proportions, graphic elements, clean matte
finish, Kidrobot/KAWS aesthetic, standing on branded circular base, pop
culture art toy vibe, no scenery"
```

---

#### **8. Art Deco figurine**
**Opis:** Lata 20./30. XX w. - smukłe sylwetki, elegancka geometria, luksusowe wykończenia (złoto, czerń, marmur).

**Prompt:**
```
"An Art Deco figurine of [character] inspired by 1920s-1930s aesthetic,
sleek elongated silhouette, elegant geometric patterns, luxurious metallic
gold and black accents, symmetrical rhythmic lines, marble-like base,
sophisticated jazz-age elegance, static poised stance"
```

---

#### **9. Ceramiczna prymitywna rzeźba (folk style)**
**Opis:** Rzemiosło ludowe - gliniane, lekko nieforemne kształty, uproszczona twarz, faktura rękodzielnicza. Prosty, swojski.

**Prompt:**
```
"A folk-style ceramic figurine of [character], handcrafted clay aesthetic,
slightly asymmetric organic shapes, simplified facial features, visible
handmade texture, earthy matte finish, symbolic decorative elements,
rustic charm, standing on simple hand-formed base"
```

---

#### **10. Anime Bishoujo figure**
**Opis:** Styl anime - smukła postać, duże oczy, dynamiczna poza, wiatr we włosach. Detale precyzyjne. Z gier i japońskich animacji.

**Prompt:**
```
"An anime bishoujo-style figurine of [character], slender proportions,
large detailed eyes with highlights, dynamic wind-swept hair, flowing
dress or outfit with precise fabric details, graceful feminine pose,
glossy painted finish, standing on decorative base, Japanese anime
collectible aesthetic"
```

---

#### **11. LEGO Minifig custom**
**Opis:** Uproszczona blokowa forma, twarz ograniczona do prostych rysów. Elementy jako printy, ręce w typowym kształcie LEGO.

**Prompt:**
```
"A LEGO minifigure-style 3D model of [character], simplified blocky
cylindrical head and torso, basic facial features (dots for eyes, simple
smile), decorative elements as printed designs on chest and legs, classic
LEGO hand shape, yellow skin tone or appropriate color, standing on LEGO
stud base plate"
```

---

#### **12. Papier-mâché (kartonowy styl)**
**Opis:** Ręcznie robiona rzeźba z warstw papieru - nieidealna tekstura, faktura warstw. Artystyczny, kolorowy, ręczna robota.

**Prompt:**
```
"A papier-mâché style figurine of [character], layered paper texture
visible, slightly imperfect handcrafted appearance, colorful painted
surface with artistic brushstrokes, folk art aesthetic, organic irregular
edges, standing on hand-formed base, artisanal charm"
```

---

#### **13. Styl FromSoftware (Soulsborne)**
**Opis:** Gotycki, mroczny z silną fakturą. Estetyka Dark Souls, Bloodborne, Elden Ring. Wydłużona postać, skomplikowana faktura, patyna.

**Prompt:**
```
"A gothic Soulsborne-style figurine of [character] inspired by Dark Souls
aesthetic, intricate weathered armor with complex surface textures, elongated
proportions, dramatic worn patina, roots and organic corruption details,
dark fantasy atmosphere, standing on cracked stone base, FromSoftware game
character design"
```

---

#### **14. Pop-art / KAWS-style designer toy**
**Opis:** Kolekcjonerski z mocnym konturem, uproszczoną twarzą, graficznymi elementami. Kolorowe detale, kształty jak z kreskówki.

**Prompt:**
```
"A pop-art KAWS-style designer toy of [character], bold graphic outlines,
simplified cartoon-like face with X-shaped eyes, oversized gloved hands,
vibrant flat colors, street art aesthetic, glossy finish, standing on
branded base with graffiti elements, contemporary collectible art style"
```

---

#### **15. Hollow sculpture (wydrążona forma)**
**Opis:** Ażurowa struktura - koronki lub siatki. Ciało i sukienka perforowane lub z dekoracyjnych wzorów. Lekki, przestrzenny.

**Prompt:**
```
"A hollow lace-like sculptural figurine of [character], intricate
perforated patterns throughout body and clothing, decorative filigree
structure, see-through latticework design, delicate spatial composition,
white or metallic finish, standing on minimal base, architectural
sculpture aesthetic"
```

---

#### **16. Styl The Sims / Second Life**
**Opis:** Prosty model 3D o realistycznych proporcjach, miękkie światło, czysty design. Brak teksturowania, ograniczone detale.

**Prompt:**
```
"A virtual character figurine of [character] in The Sims style, realistic
proportions with simplified clean geometry, soft ambient lighting baked
into model, minimal surface details, smooth plastic-like finish, standing
in neutral pose, video game character aesthetic, simple round base"
```

---

#### **17. Art Brut / outsider art**
**Opis:** Surowy, prymitywny, często ekspresyjny, zniekształcony. Artystyczna nieperfekcyjność, charakter, emocjonalność. Naiwne lub dziwaczne.

**Prompt:**
```
"An art brut outsider art figurine of [character], raw expressive form
with intentional imperfections, naive proportions, emotionally distorted
features, rough handmade aesthetic, bold simple colors, unrefined charm,
standing on irregular hand-shaped base, primitive artistic energy"
```

---

#### **18. Styl 3D print aesthetic (FDM/Resin look)**
**Opis:** Wygląda jak świeżo wydrukowana z drukarki 3D - widoczne warstwy, linie, czasem podpory. Forma surowa ale estetyczna.

**Prompt:**
```
"A figurine of [character] with visible 3D printing aesthetic, layer
lines showing FDM texture, technical raw appearance, support marks visible
as artistic choice, matte single-color filament look, standing on 3D
printed raft base, maker culture aesthetic, unfinished but intentional"
```

---

#### **19. Zabawka retro lat 60–70**
**Opis:** Vintage - gładka twarz, przerysowane oczy, uproszczona mimika. Fryzura i ubiór jak z dawnych lalek. Styl epoki.

**Prompt:**
```
"A retro 1960s-70s plastic toy figurine of [character], smooth simplified
face with large painted eyes, period-appropriate hairstyle and clothing,
vintage toy aesthetic, glossy plastic finish in pastel colors, standing
on simple circular base, nostalgic collectible doll style"
```

---

## 🎨 SEKCJA 2: SERIOUS SCULPTURAL STYLES (13 stylów)

### "Fajne komendy - style na poważnie rzeźbiarskie"

Kiedy poprosisz: **"daj mi te fajne komendy, style na poważnie rzeźbiarskie"** - dostajesz te poniżej.

---

#### **1. Styl barokowy / dramatyczna rzeźba**

**Prompt:**
```
"A baroque sculptural masterpiece of [character], highly dramatic
composition with swirling drapery and dynamic movement, intense emotional
expression, deep chiaroscuro effects carved into form, richly detailed
ornamental elements, theatrical grandeur, white marble or bronze patina
finish, mounted on classical decorated pedestal"
```

---

#### **2. Styl „jakby zrobił to Michał Anioł"**

**Prompt:**
```
"A Renaissance sculpture of [character] in the manner of Michelangelo,
heroic idealized human form with perfect anatomical detail, contrapposto
stance, marble-like surface revealing masterful understanding of muscle
and bone structure, sublime expression of human dignity, classical nude
or draped study, mounted on simple cubic marble base"
```

---

#### **3. Styl realistyczno-klasyczny (porcelanowy Lladro/Rosenthal)**

**Prompt:**
```
"A refined porcelain-style figurine of [character] in Lladro/Rosenthal
tradition, exquisite delicate craftsmanship, soft pastel glazes, graceful
naturalistic pose, flowing fabric rendered with supreme technical skill,
serene elegant expression, glossy smooth finish with subtle color
transitions, oval porcelain base"
```

---

#### **4. Styl neoklasycyzmu rzymskiego**

**Prompt:**
```
"A neoclassical Roman-style sculpture of [character], idealized heroic
proportions following Greco-Roman canon, calm noble expression, draped
in classical toga or tunic, white marble finish with subtle veining,
symmetrical balanced composition, standing contrapposto on cylindrical
marble column base"
```

---

#### **5. Styl francuskiego romantyzmu (XIX w.)**

**Prompt:**
```
"A French Romantic sculpture of [character] in 19th century manner,
passionate emotional expression, flowing dramatic gestures, detailed
period costume with naturalistic fabric folds, bronze patina with green
oxidation, dynamic asymmetric composition, narrative storytelling quality,
mounted on inscribed bronze base"
```

---

#### **6. Styl klasycznej rzeźby greckiej**

**Prompt:**
```
"A classical Greek sculpture of [character], idealized beauty following
ancient Greek aesthetic principles, serene contemplative expression,
harmonious balanced proportions, draped in flowing chiton or himation,
white marble with traces of ancient polychrome, standing in relaxed
contrapposto, simple marble plinth base"
```

---

#### **7. Styl secesyjny / Art Nouveau**

**Prompt:**
```
"An Art Nouveau sculptural figure of [character], elegant flowing organic
lines inspired by nature, sinuous curves and whiplash motifs, decorative
floral and plant elements integrated into design, graceful elongated
proportions, metallic bronze or pewter finish with verdigris accents,
ornate base with nature-inspired patterns"
```

---

#### **8. Styl Auguste Rodin (realizm ekspresyjny)**

**Prompt:**
```
"A sculpture of [character] in Auguste Rodin's expressive realist style,
powerful emotional intensity, rough textured surface suggesting artist's
hand, partial figure or fragmented form acceptable, bronze with rich dark
patina, psychological depth in gesture and expression, mounted on simple
rough-hewn stone base"
```

---

#### **9. Styl brutalizmu rzeźbiarskiego (XX w.)**

**Prompt:**
```
"A brutalist sculpture of [character], raw geometric masses and angular
forms, rough unfinished concrete or cor-ten steel appearance, monumental
simplified volumes, honest exposed materials, powerful imposing presence,
minimal detail emphasizing pure form, integrated rectangular base block"
```

---

#### **10. Styl włoskiego futuryzmu (Boccioni)**

**Prompt:**
```
"A Futurist sculpture of [character] inspired by Umberto Boccioni, dynamic
sense of motion and speed, interpenetrating geometric planes suggesting
movement through space, bronze with industrial patina, angular fragmented
forms, energy and dynamism captured in static form, abstract geometric base"
```

---

#### **11. Styl niemieckiego ekspresjonizmu (Barlach)**

**Opis:** Ernst Barlach - emocjonalny, uproszczone formy, drewno rzeźbione, duchowość.

**Prompt:**
```
"A German Expressionist sculpture of [character] in Ernst Barlach manner,
simplified powerful forms with intense emotional content, carved wood
aesthetic with visible tool marks, spiritual introspective quality,
elongated medieval-influenced proportions, dark wood or bronze finish,
standing on simple rectangular base"
```

---

#### **12. Styl minimalistycznej rzeźby nowoczesnej (Judd, Andre)**

**Prompt:**
```
"A minimalist sculptural interpretation of [character], reduced to essential
geometric forms, clean precise edges, industrial materials aesthetic,
monochromatic matte finish, pure volume and space relationships, no
decorative elements, simple rectangular or cubic base, Donald Judd inspired"
```

---

#### **13. Styl japońskiego rzemiosła rzeźbiarskiego (netsuke scaled up)**

**Prompt:**
```
"A figurine of [character] inspired by Japanese netsuke craft at larger
scale, exquisite intricate detail, balanced compact composition, smooth
ivory or boxwood finish, storytelling element integrated, masterful
technical carving, traditional Japanese aesthetic principles, mounted
on simple black lacquered base"
```

---

## 🛠️ TECHNICAL REQUIREMENTS (3D PRINTING)

### Kluczowe Keywords dla Printability:

**ZAWSZE DODAWAJ:**
```
"Standing on wide stable base"
"No floating elements"
"Physically grounded design"
"Optimized for minimal supports"
"Watertight geometry"
"No overhangs greater than 45 degrees"
"Arms close to body"
"Single solid piece"
"Thick structural supports on limbs"
```

---

### Prompt Template z Printability:

```
[CHARACTER DESCRIPTION]

Standing on [base description - 60mm circular, 4mm thick],
physically grounded design with no floating elements,
arms positioned close to body to minimize overhangs,
all props and accessories firmly connected to main figure or base,
thick structural supports on extended limbs,
optimized for FDM/resin 3D printing,
watertight single-piece geometry.
```

---

### Base Design Suggestions:

**Circular Bases:**
```
"60mm diameter circular base, 4mm thick, with [engraved pattern] detail"
```

**Themed Bases:**
```
- Tavern floor (wood planks, beer stains)
- Compass rose (for pirate/nautical)
- Runes circle (for fantasy characters)
- Cracked pavement (for urban/modern)
- Marble plinth (for classical sculptures)
```

---

## 📋 WORKFLOW CHECKLIST

### PRE-GENERATION:

**Image Reference Method:**
- [ ] Wybierz style reference (ir-3d-classic.jpg lub inny)
- [ ] Przygotuj zdjęcie osoby X (subject)
- [ ] Zdecyduj character role (Jedi, Pirate, etc.)
- [ ] Draft prompt z "[matching style of...]"
- [ ] Specify base design
- [ ] Add printability keywords

**Pure Text Method:**
- [ ] Wybierz styl z library (1-32)
- [ ] Draft detailed character description
- [ ] Specify pose and props
- [ ] Add printability keywords
- [ ] Describe base

---

### GENERATION (Tencent Hunyuan 3D):

- [ ] Upload style reference image (if using)
- [ ] Upload subject photo (if using)
- [ ] Input optimized text prompt
- [ ] Generate 5-10 variations
- [ ] Evaluate for:
  - Physical stability (no floating)
  - Printability (overhangs <45°)
  - Character accuracy
  - Style consistency
  - Base presence

---

### POST-PROCESSING:

- [ ] Auto-retopology if needed (Meshy AI, Tripo AI)
- [ ] UV unwrapping (Hunyuan 3D Studio - 20s auto)
- [ ] Watertight mesh verification
- [ ] Export to STL
- [ ] Test in slicer (Cura, PrusaSlicer)
- [ ] Add supports as needed
- [ ] Optional: FluxContext 2D editing
- [ ] Print test at 50% scale first

---

## 🎓 BEST PRACTICES

### DO's ✅

**1. Be Specific:**
```
❌ "A warrior"
✅ "A medieval knight in full plate armor, holding longsword pointed down,
    helmet under left arm, standing in attention stance"
```

**2. Physical Grounding:**
```
❌ "Floating magic orb above hand"
✅ "Holding crystal orb firmly in both hands against chest"
```

**3. Material Clarity:**
```
❌ "Nice finish"
✅ "Matte painted PVC finish with subtle metallic accents on armor"
```

**4. Complete Description:**
```
Include: Face, hair, clothing, pose, props, base, finish
```

**5. Style Consistency:**
```
For series: Always reference same style image or use same text style prompt
```

---

### DON'Ts ❌

**1. Abstract Concepts:**
```
❌ "Aura", "energy field", "smoke trail", "magic glow"
```

**2. Conflicting Styles:**
```
❌ "Realistic AND cartoon" (unless intentional hybrid)
```

**3. Vague Descriptions:**
```
❌ "Cool robot"
✅ "Sleek metallic humanoid robot with LED eyes, articulated joints,
    chrome plated finish"
```

**4. Complex Scenery:**
```
❌ "Standing in detailed forest with trees and waterfall"
✅ "Standing on circular base with subtle forest floor texture"
```

**5. Thin Fragile Parts:**
```
❌ "Thin wire antennae", "delicate butterfly wings extended"
✅ "Thick antenna rods", "folded wings close to body"
```

---

## 🔗 RESOURCES & PLATFORMS

### Main Platform:
- **Tencent Hunyuan 3D:** https://3d.hunyuan.tencent.com/
  - Best: Speed (10-25s), quality, free, open source
  - Supports: Text-to-3D, Image-to-3D
  - Output: Up to 1.5M faces, PBR textures

### Alternative Platforms:
- **Rodin AI:** Highest quality, $0.75/gen
- **Meshy AI:** Best for teams, $20/month
- **Luma Genie:** Completely free, 5-6/day limit

### Post-Processing Tools:
- **Hunyuan 3D Studio:** Auto UV unwrap (20s)
- **Meshy AI / Tripo AI:** Auto-retopology
- **Ministry of Flat:** Fully automatic UV unwrapping
- **InstaMAT Studio:** AI-assisted texturing
- **Blender:** Manual cleanup/editing (free)

### Communities:
- **Reddit:** r/StableDiffusion, r/3Dprinting, r/DesignerToys
- **X/Twitter:** @DeemosTech (Rodin), @Alpha3D_io
- **GitHub:** Awesome-Text-to-3D, Hunyuan3D-1 repo

---

## 💡 QUICK EXAMPLES

### Example 1: Image Reference + Jedi

```
INPUT:
- Style: ir-3d-classic.jpg (pijak)
- Subject: Photo of John
- Character: Jedi Master

PROMPT:
"Generate a stylized 3D figurine of the man in the photo, matching the
artistic style of the drunk man holding a beer mug. The character is
imagined as a wise Jedi Master. He wears flowing beige Jedi robes with
brown leather belt, holds a green lightsaber ignited in right hand
pointing forward, left hand raised in Force gesture. Gray beard and calm
expression. Standing on 60mm circular base with Jedi Order symbol engraved.
Neutral light gray background, no scenery, physically grounded design,
optimized for 3D printing."
```

---

### Example 2: Pure Text + Baroque Style

```
NO IMAGE REFERENCE

PROMPT:
"A baroque sculptural masterpiece of a female warrior, highly dramatic
composition with swirling cape and dynamic forward stride, intense
determined expression, ornate decorated armor with flowing fabric elements,
holding ornate longsword raised high, deep chiaroscuro effects carved into
form, richly detailed ornamental elements on armor and sword hilt, theatrical
grandeur, white marble finish with subtle veining, mounted on classical
decorated pedestal with relief carvings, physically grounded design,
optimized for resin 3D printing."
```

---

### Example 3: Image Reference + Pirate

```
INPUT:
- Style: ir-3d-classic.jpg (pijak)
- Subject: Photo of Sarah
- Character: Pirate Captain

PROMPT:
"Generate a stylized 3D figurine of the woman in the photo, matching the
artistic style of the drunk man holding a beer mug. The character is
imagined as a swashbuckling pirate captain. She wears weathered brown
leather coat with brass buttons, tricorn hat with feather, red bandana,
holds cutlass in right hand, left hand on hip where flintlock pistol is
holstered. Confident smirk expression. Standing on 60mm circular wooden
base with compass rose engraving and rope border detail. Neutral light
gray background, no scenery, all elements physically connected, optimized
for 3D printing."
```

---

## 🔥 NASTĘPNE KROKI

### Co teraz:

1. ⏳ **Dodaj ir-3d.jpg** (pijak z kuflem) - main style reference
2. ⏳ **Opcjonalnie: Stwórz więcej style references** (chibi, realistic, designer)
3. ⏳ **Wybierz tryb:**
   - Image Reference (spójna seria)
   - Pure Text (standalone pieces)
4. ⏳ **Generate w Hunyuan 3D**
5. ⏳ **Post-process:** UV unwrap, watertight check, STL export
6. ⏳ **Print test** at 50% scale first

### Gotowe do produkcji:
- ✅ Image reference workflow explained
- ✅ Style reference library planned (6 variants)
- ✅ 32 style prompts ready (19 general + 13 sculptural)
- ✅ 3D printing optimization keywords
- ✅ Complete workflow checklist
- ✅ Platform comparison
- ✅ Best practices & examples

**Ready to create some 🗿 figurines!**

---

**Ostatnia aktualizacja:** 2025-11-10
**Status:** 📚 Complete 3D figurine knowledge base - czekam na ir-3d.jpg + ready for production!
