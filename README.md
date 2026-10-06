# alturometro-meme

Mide tu estatura en platanos, manzanas, Big Macs y otras unidades que nadie pidio.

## Dos versiones

- **CLI** (`alturometro-meme/`) — binario estatico de 485 KB, corre en cualquier Linux x86_64 sin instalar nada.
- **GUI** (`alturometro-gui/`) — ventana con Slint, 14 MB. Necesita fontconfig/freetype (viene en cualquier escritorio Linux).

## Compilar desde fuente

Necesitas Rust 1.99+ (o la ultima estable):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### CLI (estatico, sin dependencias)

```bash
cd alturometro-meme
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
# El binario queda en target/x86_64-unknown-linux-musl/release/alturometro
# Copialo a donde quieras, no necesita .so ni libc.
```

### GUI

```bash
cd alturometro-gui
cargo build --release
# Binario en target/release/alturometro-gui
```

La GUI depende de fontconfig y freetype a la hora de correr. Cualquier
distribucion Linux con escritorio (GNOME, KDE, XFCE, etc.) ya las tiene.

## Binarios incluidos

El ZIP trae dos binarios compilados:

| Binario | Tamano | Tipo | Donde corre |
|---------|--------|------|-------------|
| `alturometro` | ~485 KB | estatico (musl) | Cualquier Linux x86_64 |
| `alturometro-gui` | ~14 MB | dinamico (glibc) | Linux x86_64 con fontconfig |

## Uso

```bash
# CLI con argumento
./alturometro 1.81

# CLI interactivo
./alturometro

# GUI
./alturometro-gui
```

## De donde salen los numeros

Nada es inventado. Todas las medidas vienen de fuentes verificables:

| Unidad | Tamano | Fuente |
|--------|--------|--------|
| Platano | 18 cm | USDA: platanos Cavendish medianos 17-20 cm |
| Manzana | 7.5 cm diam. | Wikipedia (Malus domestica): 7.0-8.3 cm diametro comercial |
| Big Mac | 9 cm alto | WhatsNeue (2018): 6.9 cm alto x 10.4 cm diam., medido con calibre |
| Patito de hule | 10 cm | Estandar comercial de fabricantes (4 pulgadas) |
| Tiburon blanco | 4.5 m | CSULB Shark Lab: adultos promedio 4.3-4.5 m |
| Jirafa | 5 m | Guinness World Records / San Diego Zoo: machos 4.6-5.5 m |
| New Routemaster | 11.1 m | TfL / Wikipedia: autobus doble piso londinense |
| Ballena azul | 24 m | Wikipedia / Monterey Bay Aquarium: adulto promedio 24-25 m |
| Campo de futbol | 105 m | Reglamento FIFA |
| Torre Eiffel | 330 m | tour-eiffel.paris: 330 m con antena |

## Licencia

CLI: WTFPL v2 (hace lo que quieras).
GUI: WTFPL v2. Slint en si es GPL-3.0, pero tu codigo derivado puede ser WTFPL.