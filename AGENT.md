# 32BitGo — Agent Guide

Contrato permanente para agentes de IA neste repositório. Toda mudança de código,
documentação ou processo de engenharia reversa deve obedecer a este arquivo.

**32BitGo** é uma alternativa nativa, local-first e open-source ao ecossistema
proprietário da 8BitDo (Ultimate Software V1/V2, utilitários de dongle e
firmwares isolados). O nome é paródia (*8Bit→32Bit*, *Do→Go*) e **não** implica
uso da linguagem Go.

Licença dual: **MIT OR Apache-2.0** (padrão OpenLogi).

Escopo imediato (Fase 1–2): apenas **8BitDo Retro Mechanical Keyboard — NES
(N Edition)**. Demais edições e periféricos existem só como contratos extensíveis
até a Fase 3.

---

## Stack obrigatória

| Camada | Tecnologia |
|--------|------------|
| Linguagem | **Rust** (edition 2021+, workspace Cargo) |
| HID/USB | `hidapi` (+ abstração em `bitgo-hid`); sem bindings exclusivos de um SO |
| CLI | `clap` (`bitgo-cli`) |
| GUI (Fase 2) | **Iced** (`bitgo-ui`) — GPU nativa, sem WebView |
| Temas | `bitgo-theme` com Design Tokens; default **NES** |

Proibido: Python, Assembly (código ou disassembly de binários), Go por influência
do nome, scripts auxiliares em linguagens interpretadas.

---

## Arquitetura em três camadas

### 1. Hardware Transport Layer (`bitgo-hid`)

- Única camada autorizada a abrir handles USB/HID.
- API estável: `enumerate`, `open`, `read`, `write`, `get_feature_report`,
  `send_feature_report`, com timeouts e cancelamento.
- Zero lógica de produto 8BitDo aqui — só transporte + identidade USB
  (`vid`, `pid`, `usage_page`, `usage`, path de interface).
- Toda chamada de I/O retorna `Result`; nunca `unwrap`/`expect` em caminhos de
  hardware em código de produção.
- Isolar diferenças Linux / Windows / macOS atrás de traits; proibir `#cfg`
  de plataforma espalhado em callers acima desta crate.

### 2. Core & Device Plugin Engine (`bitgo-core` + plugins)

- `bitgo-core` define o contrato de plugin (`DeviceDriver`), modelos de perfil,
  tecla, macro, Super Button, bateria e erros de domínio.
- Plugins vivem em crates `bitgo-device-*`. O primeiro e único implementado
  agora: `bitgo-device-retro-keyboard` (variante NES).
- O core **não** importa crates de GUI. Plugins **não** importam Iced.
- Registro de dispositivos por `(vid, pid, interface hints)` + detecção de
  edição (NES / futuras) sem hardcode de UI.

### 3. Native GUI / Theme Engine (`bitgo-ui` + `bitgo-theme`)

- UI consome apenas APIs de `bitgo-core` / IPC futuro — nunca `hidapi` direto.
- Temas são pacotes de Design Tokens (`colors`, `typography`, `spacing`,
  `surfaces`). Ao detectar Retro Keyboard NES, carregar tema `nes` por padrão.
- Novos temas (Famicom, C64, M) entram como módulos de tokens sem alterar o
  layout estrutural dos widgets.

```
┌─────────────────────────────────────────────┐
│  bitgo-ui  +  bitgo-theme (NES default)     │
├─────────────────────────────────────────────┤
│  bitgo-core  +  bitgo-device-retro-keyboard │
├─────────────────────────────────────────────┤
│  bitgo-hid  (hidapi / libusb user-space)    │
└─────────────────────────────────────────────┘
```

---

## Concorrência e memória

- Preferir ownership explícito; `Arc`/`Mutex` só nas fronteiras de compartilhamiento
  de handle HID ou estado de sessão.
- Um writer exclusivo por dispositivo: serializar comandos HID em fila/async mutex
  por `DeviceId`. Leituras de status não podem intercalar writes de perfil sem
  barreira.
- Sem `unsafe` fora de FFI devidamente documentada e revisada; hidapi já encapsula
  o FFI — não reintroduzir ponteiros crus nos plugins.
- Proibir deadlocks: locks na ordem Transport → Session → Cache, nunca o inverso.
- Operações longas (dump de perfil, sync) devem ser canceláveis e não bloquear a
  thread da UI.

---

## Erros de I/O de hardware

- Tipar erros com `thiserror` em camadas internas; `anyhow` só em binários
  (`bitgo-cli`, `bitgo-ui`).
- Classificar: `Disconnected`, `Timeout`, `Protocol`, `ValidationRejected`,
  `UnsupportedEndpoint`, `PermissionDenied`.
- Após `Disconnected` ou erro de protocolo irrecuperável: fechar handle, limpar
  sessão, exigir re-enumeração — não reutilizar handle corrompido.
- Nunca silenciar `Err` de write HID; logar payload hash/tamanho (não dump completo
  de perfis do usuário em logs default).

---

## Segurança HID — anti-brick

1. **Allowlist de endpoints:** apenas Usage/Report IDs mapeados para configuração
   de teclas, macros, perfis, Super Buttons e status/bateria.
2. **Bloqueio total de DFU/OTA/bootloader** nesta fase. Qualquer report cujo
   descritor ou assinatura de bytes indique atualização de firmware deve ser
   rejeitado em `bitgo-core` antes do transporte.
3. **Validação de payload:** schemas de tamanho fixo/max, checksums conhecidos e
   ranges de keycodes antes de `write`/`send_feature_report`.
4. **Dry-run:** CLI/GUI devem oferecer modo que serializa o frame sem enviar.
5. **Fixtures:** capturas USB anonimizadas em testes — nunca gravar no dispositivo
   em CI sem hardware harness explícito e flag de opt-in.
6. **Sem writes exploratórios** em loops de fuzz contra hardware real.
7. Engenharia reversa **somente** por captura de tráfego USB/HID e descritores —
   **proibido** disassembly/decompile de Ultimate Software ou firmwares.

---

## Independência de plataforma

- Todo recurso OS-específico (udev, WinUSB/SetupAPI, IOKit) fica em `bitgo-hid`
  ou `packaging/`, nunca em plugins de produto.
- Features Cargo por target são aceitas só no transporte e no empacotamento.
- Testes de lógica de protocolo devem rodar sem dispositivo (`#[cfg(test)]` +
  fixtures).

---

## Fluxo de trabalho do agente

1. Ler este arquivo e o `PLAN.md` antes de expandir escopo.
2. Não implementar drivers além do Retro Keyboard NES até a Fase 3 autorizada.
3. Não adicionar dependências interpretadas ou toolchains Python.
4. Preferir `cargo check -p <crate>` / testes focados no loop interno.
5. Commits Conventional Commits quando o usuário pedir commit.
6. Linux: manter `packaging/udev/99-32bitgo-8bitdo.rules` sincronizado com VIDs/PIDs
   conhecidos do escopo ativo.

---

## Mapa de crates

| Crate | Papel |
|-------|--------|
| `bitgo-hid` | Transport Layer multiplataforma |
| `bitgo-core` | Tipos, plugin trait, validação, anti-DFU |
| `bitgo-device-retro-keyboard` | Driver do Retro Keyboard (NES primeiro) |
| `bitgo-theme` | Design Tokens; tema `nes` embutido |
| `bitgo-cli` | PoC Fase 1: enumerate / dump / dry-run |
| `bitgo-ui` | GUI Iced (Fase 2; stub na fundação) |
