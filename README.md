<div align="center">

<img alt="Lumen Motion" src="docs/lumen/icon.png" width="120">

# Lumen Motion

**Gráficos en movimiento, animación y efectos visuales para Windows: rápido, nativo, gratuito y sin suscripciones.**
Capas con fotogramas clave, máscaras, formas, textos animados, 3D, expresiones y más de 300 efectos, y herramientas propias para quien crea contenido:
**animar un texto de un clic, títulos y rótulos listos, transiciones rápidas, copias verticales para Reels y exportación a todas las redes a la vez.**

Hecho por [**SifG22**](https://sifg.net).

[![Última versión](https://img.shields.io/github/v/release/SifG22/lumen_motion?label=versi%C3%B3n&color=14a6b3&style=for-the-badge)](https://github.com/SifG22/lumen_motion/releases/latest)
[![Descargas](https://img.shields.io/github/downloads/SifG22/lumen_motion/total?label=descargas&color=0e9aa8&style=for-the-badge)](https://github.com/SifG22/lumen_motion/releases)

![Windows 10 y 11](https://img.shields.io/badge/Windows-10%20%7C%2011-0c0e13)
![x64 y ARM64](https://img.shields.io/badge/arquitectura-x64%20%7C%20ARM64-0c0e13)
![Rust](https://img.shields.io/badge/hecho%20en-Rust-0c0e13?logo=rust)
![Sin conexión](https://img.shields.io/badge/funciona-sin%20conexi%C3%B3n-0c0e13)

### [⬇ Descargar para Windows](https://github.com/SifG22/lumen_motion/releases/latest) · [sifg.net](https://sifg.net) · [Términos](assets/legal/TERMINOS.txt) · [Novedades](packaging/lumen/NOVEDADES.md)

</div>

---

## 📑 Contenido

- [Solo en Lumen Motion: el menú Lumen](#-solo-en-lumen-motion-el-menú-lumen)
- [Cómo se usa cada herramienta](#-cómo-se-usa-cada-herramienta)
- [Todo lo que puedes hacer](#-todo-lo-que-puedes-hacer)
- [Archivos de Lumen Motion](#-archivos-de-lumen-motion)
- [Descargar e instalar](#️-descargar-e-instalar)
- [Actualizaciones automáticas](#-actualizaciones-automáticas)
- [Preguntas frecuentes](#-preguntas-frecuentes)
- [Compilar desde el código](#️-compilar-desde-el-código)
- [Términos, licencias y créditos](#-términos-licencias-y-créditos)

---

## 🚀 Solo en Lumen Motion: el menú **Lumen**

Herramientas propias de SifG22 para hacer en un clic lo que en otros programas lleva muchos pasos. Están juntas en el menú **Lumen**, entre «Animación» y «Ver». Cada una es **un solo paso de deshacer** (Ctrl + Z).

| | |
|---|---|
| ✍️ **Animar texto** | Máquina de escribir, aparecer subiendo, rebote, letras que se juntan, desenfoque o líneas que caen. Sobre los textos seleccionados, o crea uno nuevo si no hay ninguno. |
| 🎬 **Título animado** | Título y subtítulo centrados que entran y salen solos, al tamaño de tu composición. |
| 🪪 **Rótulo inferior** | Una barra con nombre y cargo que entra deslizándose y se va 5 segundos después. |
| 🔀 **Transición rápida** | Fundido, deslizar desde la izquierda, deslizar hacia arriba, zoom o pop en la entrada y la salida de las capas seleccionadas. |
| 📈 **Curvas de velocidad** | Suave, frenar al llegar, arrancar despacio, enérgico, lineal, rebote elástico y rebote, con nombres que dicen lo que hacen. Probar una sustituye a la anterior. |
| 📱 **Copias vertical, cuadrada, retrato y horizontal** | 9:16, 1:1, 4:5 o 16:9 con tu composición entera encima de un fondo desenfocado de sí misma: nada se corta. |
| 📤 **Exportar para redes sociales** | Un clic y se exportan en MP4 tu composición y sus versiones 9:16, 1:1 y 4:5, cada una en su archivo. |
| 🫥 **Exportar con transparencia** | WebM (VP9) con canal alfa, para usar tus títulos encima de cualquier vídeo. |
| ⚡ **Previsualización rápida** | Media resolución y previsualización adaptativa en un clic, para trabajar con fluidez; otro clic y vuelves a la calidad completa. |
| 🧩 **Plantillas de proyecto** | Lower third, tarjeta de título, revelado de logo, tipografía cinética, post cuadrado, historia vertical, pase de fotos y título 3D, editables desde Gráficos esenciales. |

## 📖 Cómo se usa cada herramienta

<details open>
<summary><b>✍️ Animar texto</b></summary>

Selecciona una o varias capas de texto (o ninguna) y elige **Lumen › Animar texto › …**. La animación empieza en el instante actual. Si no había textos seleccionados, Lumen Motion crea uno centrado («Tu texto aquí») y, si no había composición abierta, una de 1920×1080.
</details>

<details>
<summary><b>🎬 Título animado y 🪪 rótulo inferior</b></summary>

Coloca el indicador de tiempo donde quieras que aparezcan y elige **Lumen › Añadir título animado** o **Lumen › Añadir rótulo inferior**. Cambia los textos con la herramienta de texto; el color de la barra, en su relleno.
</details>

<details>
<summary><b>🔀 Transición rápida</b></summary>

Selecciona capas y elige **Lumen › Transición rápida › …**. Se añaden fotogramas clave en el punto de entrada y en el de salida de cada capa (medio segundo, o menos si la capa es corta).
</details>

<details>
<summary><b>📈 Curvas de velocidad</b></summary>

Selecciona fotogramas clave (o capas con animación) y elige **Lumen › Curvas de velocidad › …**. *Rebote elástico* hace que el movimiento se pase un poco y vuelva; *Rebote* hace que rebote contra su destino. Las dos usan una expresión que Lumen Motion escribe y sustituye por sí mismo; nunca toca una expresión tuya.
</details>

<details>
<summary><b>📱 Copias y 📤 exportar para redes</b></summary>

Con la composición abierta, **Lumen › Copia vertical (9:16)** (o cuadrada, retrato, horizontal) crea «<nombre> (Vertical 9:16)» con tu composición anidada: los cambios en el original se ven en la copia. **Lumen › Exportar para redes sociales** crea las copias que falten (o reutiliza las que ya hiciste y retocaste), las añade a la cola de procesamiento en MP4 (H.264) y empieza a exportar. Los archivos van junto a tu proyecto.
</details>

## ✨ Todo lo que puedes hacer

| | |
|---|---|
| 🎞️ **Animación** | Fotogramas clave en cualquier propiedad, editor de gráficos, trazados de movimiento, emparentado, expresiones y scripts. |
| 🔷 **Formas y máscaras** | Capas de forma con operadores, máscaras animables, mates de seguimiento y modos de fusión. |
| ✍️ **Texto** | Animadores de texto con selectores, texto en trazado, texto 3D y ajustes preestablecidos de texto. |
| ✨ **Efectos** | Más de 300 efectos (desenfoques, color, distorsión, generadores, partículas, claves), en la GPU. |
| 🧊 **3D** | Cámaras, luces, capas 3D clásicas y avanzadas, y modelos glTF, OBJ, STL y FBX. |
| 🎯 **Seguimiento** | Seguimiento de movimiento y de cámara, estabilización, Roto Brush y Object Matte. |
| 📂 **Importa de todo** | Vídeo (H.264, HEVC, ProRes, VP9, AV1), imágenes, secuencias, PSD, SVG, PDF, Lottie y audio. |
| 📤 **Exporta** | H.264, HEVC, AV1, ProRes, WebM con alfa, GIF, secuencias PNG/TIFF/EXR y Lottie. |
| 💾 **Proyectos seguros** | Autoguardado cada 3 minutos, recuperación tras un cierre inesperado y copia `.bak` del anterior en cada guardado. |
| 🌍 **En tu idioma** | Español por defecto, y también inglés, portugués, francés, alemán, japonés, chino, ucraniano, ruso y checo. |
| 🔄 **Se actualiza solo** | Te avisa cuando sale una versión nueva y se instala con un clic. |
| 🔒 **Tu trabajo es tuyo** | Funciona sin conexión y no envía tus proyectos ni tus datos a ningún sitio. |

## 📁 Archivos de Lumen Motion

| Archivo | Qué es |
|---|---|
| `.lumeneffects` | Proyecto (JSON). Lleva `"format": "lumeneffects"` y `"formatVersion"`: un proyecto de una versión más nueva o dañado da un mensaje claro y nunca se modifica. |
| `.lumeneffectsx` | Copia del proyecto en XML (Archivo › Guardar una copia como XML). |
| `.lumeneffectstemplate` | Plantilla de Gráficos esenciales (con sus medios). |
| `.lumeneffectspreset` | Ajuste preestablecido de animación, efectos o curvas. |

Lumen Motion **abre también los proyectos, plantillas y ajustes del programa del que procede** (`.ecproj`, `.ecprojx`, `.ectemplate`, `.ecpreset`). Al guardarlos se crea un archivo nuevo con la extensión de Lumen Motion junto al original, que no se toca. Cada guardado es atómico (archivo temporal y renombrado) y deja una copia `.bak` de la versión anterior. Los autoguardados van a la carpeta de datos de la app (`%APPDATA%\Lumen Motion`, o `LumenMotionData` junto al `.exe` en modo portable).

## ⬇️ Descargar e instalar

1. Entra en la **[última versión](https://github.com/SifG22/lumen_motion/releases/latest)** y descarga el instalador de tu PC:

   | Tu PC | Instalador | Portable (sin instalar) |
   |---|---|---|
   | La mayoría (Intel o AMD) | [`LumenMotion-Setup-x64.exe`](https://github.com/SifG22/lumen_motion/releases/latest/download/LumenMotion-Setup-x64.exe) | `LumenMotion-<versión>-windows-x64.zip` |
   | Windows en ARM (Snapdragon, Surface Pro X / 11) | [`LumenMotion-Setup-arm64.exe`](https://github.com/SifG22/lumen_motion/releases/latest/download/LumenMotion-Setup-arm64.exe) | `LumenMotion-<versión>-windows-arm64.zip` |

2. Abre el archivo y pulsa **Instalar**. No pide permisos de administrador.
3. Si Windows muestra **«Windows protegió su PC»**, pulsa *Más información* › *Ejecutar de todas formas* (la app aún no está firmada digitalmente).

| | |
|---|---|
| **Requisitos** | Windows 10 (1809) u 11 de 64 bits, 8 GB de RAM (16 GB para 4K) y una gráfica compatible con DirectX 12 o Vulkan. |
| **Versión portable** | Descomprime el `.zip` y abre `LumenMotion.exe`. Con un archivo vacío `portable.txt` al lado, todo se guarda en `LumenMotionData` (ideal para un USB). |
| **Archivos** | El instalador asocia `.lumeneffects`, `.lumeneffectstemplate` y `.lumeneffectspreset` (y los formatos anteriores) con su propio icono. Doble clic abre el archivo; si la app ya está abierta, lo abre en esa ventana. |
| **Desinstalar** | Configuración › Aplicaciones › Lumen Motion (también quita las asociaciones). |

## 🔄 Actualizaciones automáticas

Lumen Motion consulta, como mucho **una vez al día**, `https://github.com/SifG22/lumen_motion/releases/latest/download/latest.json`. Si hay una versión más nueva, te pregunta; si aceptas, descarga el instalador de tu arquitectura, **comprueba su SHA-256** y lo ejecuta. Para desactivarlo, define la variable de entorno `LUMEN_MOTION_NO_UPDATE=1`.

## ❓ Preguntas frecuentes

<details>
<summary><b>¿Es gratis? ¿Lleva marcas de agua?</b></summary>

Es gratis, también para trabajos profesionales, y no añade marcas de agua.
</details>

<details>
<summary><b>¿Abre mis proyectos de After Effects?</b></summary>

Todavía no abre `.aep`. Las expresiones y los scripts de estilo After Effects sí funcionan. Lumen Motion no está afiliado a Adobe.
</details>

<details>
<summary><b>¿Se me puede perder el trabajo si se cierra de golpe?</b></summary>

No debería: hay un autoguardado cada 3 minutos y, si la app se cierra de forma inesperada, al abrirla de nuevo te ofrece recuperar los cambios.
</details>

<details>
<summary><b>He encontrado un fallo o tengo una idea</b></summary>

Abre un [*issue*](https://github.com/SifG22/lumen_motion/issues) o cuéntamelo en [sifg.net](https://sifg.net). Indica tu versión (Ayuda › Acerca de) y los pasos que llevan al fallo.
</details>

## 🛠️ Compilar desde el código

Necesitas [Rust](https://rustup.rs) y las *Build Tools* de Visual Studio 2022 («Desarrollo para el escritorio con C++»).

```bat
:: abrir la app en modo desarrollo
cargo lumen

:: ZIP portable, instalador (necesita Inno Setup 6) y latest.json en dist\
powershell -ExecutionPolicy Bypass -File packaging\lumen\empaquetar.ps1 -Instalador
powershell -ExecutionPolicy Bypass -File packaging\lumen\empaquetar.ps1 -Arch arm64 -Instalador
```

| Dónde | Qué hay |
|---|---|
| [`crates/engine/src/commands/lumen.rs`](crates/engine/src/commands/lumen.rs) | Las herramientas del menú Lumen (tests en `tests_lumen.rs`) |
| [`crates/project/src/files.rs`](crates/project/src/files.rs) | Las extensiones de archivo y la compatibilidad con las anteriores |
| [`crates/engine/src/links.rs`](crates/engine/src/links.rs) | Nombre, versión y enlaces |
| [`apps/lumen-motion/src/updater.rs`](apps/lumen-motion/src/updater.rs) | El actualizador automático |
| [`packaging/lumen/`](packaging/lumen/) | Empaquetado, instalador (Inno Setup) y novedades de cada versión |
| [`assets/app-icon/`](assets/app-icon/) | El icono en todos sus tamaños |
| [`docs/`](docs/) | Documentación técnica (en inglés) |

**Publicar una versión:** sube la versión en `VERSION` y en `[workspace.package] version` de `Cargo.toml`, añade sus novedades a [`packaging/lumen/NOVEDADES.md`](packaging/lumen/NOVEDADES.md) y crea la etiqueta (`git tag v0.0.2 && git push origin v0.0.2`). GitHub Actions ([`lumen-release.yml`](.github/workflows/lumen-release.yml)) compila x64 y ARM64, genera instaladores, ZIP portable y `latest.json`, comprueba que se instalan, asocian los archivos y arrancan, y publica la Release. Los tests se pasan en Windows con cada cambio ([`lumen-tests.yml`](.github/workflows/lumen-tests.yml)).

## 📜 Términos, licencias y créditos

- El uso de Lumen Motion se rige por los **[términos y condiciones de SifG22](assets/legal/TERMINOS.txt)**, que también se muestran al instalar y en Ayuda › Acerca de › Términos.
- Los nombres **Lumen** y **Lumen Motion** y su marca son de SifG22. El icono es obra de SifG22 ([`assets/app-icon/LICENSE.txt`](assets/app-icon/LICENSE.txt)).
- Lumen Motion es software de código abierto con **licencia MIT** ([LICENSE-MIT](LICENSE-MIT), [NOTICE](NOTICE)) o Apache-2.0; los avisos de copyright que exige la licencia están en esos archivos.
- Las fuentes (Inter, Noto Serif, JetBrains Mono, SIL OFL 1.1) y demás componentes de terceros, con sus licencias, están en [ATTRIBUTION.md](ATTRIBUTION.md).
- Lumen Motion no contiene código, iconos, presets ni LUT de Adobe, ni código GPL o LGPL. No está afiliado ni respaldado por Adobe.

<sub>Adobe y After Effects son marcas de Adobe Inc. Instagram, TikTok, YouTube y otras son marcas de sus propietarios; solo se nombran para describir formatos compatibles.</sub>

---

<div align="center">

<img alt="" src="docs/lumen/icon.png" width="40">

**Lumen Motion** · Hecho por **[SifG22](https://sifg.net)**

</div>
