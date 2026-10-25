# 32BitGo

Alternativa **nativa**, **local-first** e **open-source** ao [8BitDo Ultimate Software V2](https://app.8bitdo.com/Ultimate-Software-V2/) — com suporte de primeira classe a **Linux**, Windows e macOS.

O nome é uma paródia/homenagem à 8BitDo (*8Bit → 32Bit*, *Do → Go*). **Não** usa a linguagem Go.

> Escopo atual: **8BitDo Retro Mechanical Keyboard — NES (N Edition)**  
> Demais edições e periféricos 8BitDo estão previstos na arquitetura de plugins (ver [`PLAN.md`](PLAN.md)).

## Por quê

O software oficial é proprietário, pesado, fragmentado (Ultimate V1/V2, utilitários de dongle, firmwares isolados) e **sem Linux**. O 32BitGo mira:

- Remapeamento de teclas, macros, perfis e Super Buttons
- Leitura de status/bateria
- Motor de temas dinâmico alinhado à edição física (primeiro: **NES**)
- Hub futuro unificado para o ecossistema 8BitDo

## Stack

| Camada | Escolha |
|--------|---------|
| Linguagem | **Rust** |
| Transporte HID | `hidapi` (`bitgo-hid`) |
| Core / plugins | `bitgo-core` + `bitgo-device-retro-keyboard` |
| Temas | `bitgo-theme` (default **NES**) |
| CLI | `bitgo-cli` |
| GUI (Fase 2) | **Iced** (`bitgo-ui`) |

Detalhes de arquitetura, RE (USB capture only — sem Assembly) e roadmap: [`PLAN.md`](PLAN.md).  
Regras para agentes de IA: [`AGENT.md`](AGENT.md).

## Status

**Marco Zero** — fundação do repositório, contratos de camadas e CLI esqueleto. Protocolo HID ainda em engenharia reversa via captura de tráfego.

## Build

```bash
cargo build -p bitgo-cli
cargo run -p bitgo-cli -- --help
```

Requisitos: Rust 1.85+ (`rust-toolchain.toml`), libhidapi no sistema (Linux: pacote `libhidapi-hidraw` / `libhidapi-libusb` conforme distro).

### Linux — udev (sem root)

```bash
sudo cp packaging/udev/99-32bitgo-8bitdo.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Reconecte o teclado e adicione o usuário ao grupo adequado (`plugdev` / acesso `uaccess`) se a distro exigir.

## Estrutura

```text
crates/
  bitgo-hid/                     # Hardware Transport Layer
  bitgo-core/                    # Plugin engine + validação anti-DFU
  bitgo-device-retro-keyboard/   # Driver Retro Keyboard (NES)
  bitgo-theme/                   # Design Tokens (NES)
  bitgo-cli/                     # PoC Fase 1
  bitgo-ui/                      # GUI Iced (Fase 2 stub)
packaging/udev/                  # Regras udev
```

## Licença

Licenciamento dual, no mesmo estilo do [OpenLogi](https://github.com/AprilNEA/OpenLogi):

- [Apache License 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

Você pode escolher qualquer uma das duas.

## Aviso legal

32BitGo é projeto independente, não afiliado à 8BitDo. “8BitDo” e nomes de produtos são marcas de seus respectivos proprietários. Engenharia reversa limitada a análise de protocolo USB/HID; **sem** atualização de firmware nesta fase.
