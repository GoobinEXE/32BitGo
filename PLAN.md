# 32BitGo — Plano Inicial (Marco Zero)

Documento vivo da fundação. Escopo de implementação imediata: **apenas**
8BitDo Retro Mechanical Keyboard — **NES (N Edition)**.

---

## 1. Stack nativa escolhida

### Decisão: **Rust + hidapi + Iced**

| Critério | Rust + Iced | C++ + Qt/Slint |
|----------|-------------|----------------|
| Segurança de memória | Ownership/borrow checker; sem UAF em paths HID | Manual / smart pointers; custo maior em I/O concorrente |
| Ecossistema USB/HID | `hidapi`, `rusb`, padrões maduros (ex.: OpenLogi) | libusb/hidapi C; bindings manuais |
| GUI multiplataforma leve | Iced (wgpu), sem WebView, binário único por target | Qt pesado; Slint competitivo mas ecossistema HID menos integrado ao restante |
| Empacotamento Linux | User-space + udev; crates estáveis | Similar, com ABI C++ mais frágil entre distros |
| Alinhamento a referência open-source | Mesmo eixo do OpenLogi (Rust, dual license) | — |

**Por que não Go:** o sufixo *Go* do nome é só paródia da marca 8BitDo. Go
oferece GC e ecossistema HID inferior para este perfil de app.

**Por que não Tauri (nesta fase):** WebView aumenta superfície e consumo; o alvo
é app de configuração de hardware ultraleve. Iced mantém UI 100% Rust.

**Por que não Assembly / RE de binários:** protocolo se recupera por captura USB
e descritores HID; disassembly viola restrição do projeto e não é necessário
para o Marco Zero.

Workspace Cargo em `crates/*`, licença `MIT OR Apache-2.0`.

---

## 2. Estratégia de engenharia reversa (sem Assembly)

Objetivo: documentar reports HID de configuração (perfis, remaps, macros, Super
Buttons, bateria) do Retro Keyboard NES, comparando o tráfego do
[8BitDo Ultimate Software V2](https://app.8bitdo.com/Ultimate-Software-V2/) com
o dispositivo real.

### 2.1 Inventário USB

1. Conectar o teclado (USB cabo e, depois, dongle 2.4GHz / BT se aplicável).
2. Registrar `idVendor` / `idProduct`, strings iManufacturer/iProduct, bInterface
   Number, `bInterfaceClass` (HID=3), `bcdDevice`.
3. Dump de descritores HID (report descriptor) via OS tools:
   - Linux: `usbhid-dump`, `hidrd-convert`, sysfs em `/sys/bus/hid/`
   - macOS: IO Registry / `ioreg`
   - Windows: USB Device Tree Viewer / HID descriptor dumpers
4. Classificar interfaces: teclado boot, consumer, **vendor config** (alvo do
   protocolo 32BitGo).

Constantes iniciais (a confirmar no hardware — VID típico 8BitDo `0x2DC8`):

```text
VID: 0x2DC8
PID: <preencher após captura no Retro Keyboard NES>
Edition hint: N Edition → ThemeId::Nes
```

### 2.2 Captura de tráfego

| SO | Ferramenta |
|----|------------|
| Windows | Wireshark + USBPcap |
| Linux | Wireshark + `usbmon` (`modprobe usbmon`) |
| macOS | Wireshark USB (onde suportado) + logs de HID do SO |

Procedimento:

1. Iniciar captura filtrada no bus/dispositivo do teclado.
2. Baseline: plug-in, idle, leitura de bateria/status no app oficial.
3. Diff controlado — **uma alteração por captura**:
   - Remap de uma tecla
   - Criação/edição de uma macro
   - Bind de um Super Button
   - Troca/salvamento de perfil
4. Exportar frames HID interrupt/control (OUT/SET_REPORT / GET_REPORT).
5. Comparar byte arrays: prefixos fixos (header/cmd), campos variáveis
   (keycode, slot, profile id), trailers/checksums.
6. Anonimizar e versionar fixtures em `fixtures/hid/` (sem dados pessoais).

### 2.3 Modelo de protocolo (alvo)

```text
Host ──Feature/Output Report──► Device (config allowlist)
Host ◄──Input/Feature Report─── Device (ack / status / battery)
```

Implementação: decoder/encoder em `bitgo-device-retro-keyboard`, validação em
`bitgo-core` (tamanho, ranges, rejeição DFU).

### 2.4 Detecção de variante

Pipeline: enumerate → match VID/PID → ler identity report ou USB product
string → `KeyboardEdition::{Nes, …}` → selecionar plugin + tema.

Edições futuras (Famicom, C64, M) só adicionam mapping de edition→theme/driver
flags; o protocolo de teclas pode ser compartilhado se a captura confirmar.

---

## 3. Arquitetura de temas (inicial: NES Edition)

### 3.1 Design Tokens

`bitgo-theme` expõe:

```rust
pub struct ThemeTokens {
    pub id: ThemeId,          // Nes | Famicom | C64 | M | ...
    pub display_name: &'static str,
    pub colors: ColorTokens,  // bg, surface, accent, text, danger, ...
    pub typography: TypeTokens,
    pub spacing: SpacingTokens,
    pub radii: RadiusTokens,
}
```

Tema **NES** embutido como default para Retro Keyboard N Edition:

- Fundo escuro estilo console (cinzas frios + preto)
- Accent vermelho/laranja “power LED” NES
- Tipografia display pixel-adjacent (fonte libre embutida na Fase 2)
- Super Buttons e layout visual alinhados à silhueta do teclado físico NES

### 3.2 Resolução dinâmica

1. Device plugin reporta `ThemeHint::Nes`.
2. `ThemeEngine::resolve(hint)` → `&'static ThemeTokens`.
3. UI aplica tokens via stylesheet Iced / helpers — widgets não hardcodam hex.

Novos temas = novo módulo `themes/famicom.rs` etc. + entrada no registry.
Nenhum tema além de NES é implementado no Marco Zero / Fase 1–2 inicial.

---

## 4. Hub único 8BitDo — contrato de plugins (futuro)

Escopo **futuro** (Fase 3): unificar Ultimate V1/V2, utilitários de adaptador,
controles, mouses, arcade sticks, numpads e dongles.

### 4.1 Trait estável (`bitgo-core`)

```rust
pub trait DeviceDriver: Send + Sync {
    fn identity(&self) -> DeviceIdentity;
    fn capabilities(&self) -> Capabilities;
    fn read_status(&mut self, transport: &mut dyn HidTransport)
        -> Result<DeviceStatus, CoreError>;
    fn pull_profile(&mut self, transport: &mut dyn HidTransport, slot: ProfileSlot)
        -> Result<Profile, CoreError>;
    fn push_profile(&mut self, transport: &mut dyn HidTransport, profile: &Profile)
        -> Result<(), CoreError>;
    // Sem métodos firmware/DFU nesta API até política explícita de release.
}
```

### 4.2 Registry

```text
DeviceRegistry
  ├─ retro_keyboard (NES)     ← implementado
  ├─ retro_keyboard (outras)  ← Fase 3 temas + PIDs
  ├─ gamepad_*                ← Fase 3
  ├─ mouse_*                  ← Fase 3
  ├─ arcade_* / numpad_*      ← Fase 3
  └─ dongle_*                 ← Fase 3
```

Cada plugin é crate isolada; o binário registra só o que compila com features.
GUI lista dispositivos por `DeviceIdentity` genérica.

---

## 5. Suporte nativo ao Linux (udev)

Arquivo: `packaging/udev/99-32bitgo-8bitdo.rules`

- `MODE="0660"`, `GROUP="plugdev"` (ou `TAG+="uaccess"` / `TAG+="seat"` conforme
  distro) para VID `2dc8` e PIDs do Retro Keyboard documentados.
- Objetivo: I/O HID em user-space **sem root**.
- README instrui: copiar rules → `udevadm control --reload-rules` → replug.

Windows: WinUSB não deve ser necessário se a interface HID padrão for usada.
macOS: acesso via hidapi; documentar permissões se o OS exigir.

---

## 6. Roadmap

### Fase 1 — Fundação + RE + CLI (atual)

- Workspace, AGENT/PLAN/README, licenças, udev skeleton
- Capturas HID e fixtures
- `bitgo-cli`: list, inspect descriptors, dry-run encode, status quando protocolo
  estiver mapeado
- Bloqueio DFU no core

### Fase 2 — GUI nativa multiplataforma

- `bitgo-ui` com Iced + tema NES
- Remap, macros, Super Buttons, perfis
- Linux / Windows / macOS packing mínimo

### Fase 3 — Expansão hub

- Temas Famicom, C64, M
- Novos plugins de periféricos 8BitDo
- Substituição progressiva dos utilitários oficiais no fluxo do usuário

---

## 7. Fora de escopo (agora)

- Escrita em bootloader / OTA / DFU
- Drivers de gamepads/mouses/etc.
- Temas não-NES
- Conta cloud, telemetria, dependência do app oficial em runtime
