# 📡 snmp-iface

[![Build](https://github.com/lis1991/snmp-iface/actions/workflows/build.yml/badge.svg)](https://github.com/lis1991/snmp-iface/actions/workflows/build.yml)
[![Release](https://github.com/lis1991/snmp-iface/actions/workflows/release.yml/badge.svg)](https://github.com/lis1991/snmp-iface/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)

SNMP-просмотрщик сетевых интерфейсов — **pure Rust**, без системных зависимостей, без `libsnmp`.

Опрашивает `ifTable` (RFC 1213) по `SNMPv2c` и выводит красивую таблицу прямо в терминал.

---

## 📦 Быстрый старт

### Скачать готовый бинарник

Перейдите в раздел [**Releases**](https://github.com/lis1991/snmp-iface/releases) и скачайте файл для своей ОС.

```bash
# Linux
chmod +x snmp-iface-linux-x86_64
./snmp-iface-linux-x86_64 192.168.1.1

# Windows
snmp-iface-windows-x86_64.exe 192.168.1.1
```

### Собрать из исходников

```bash
git clone https://github.com/lis1991/snmp-iface.git
cd snmp-iface
cargo build --release
./target/release/snmp-iface <IP> [community] [port]
```

**Требования:** Rust 1.70+

---

## 🚀 Использование

```
snmp-iface <host> [community] [port]
```

| Аргумент | По умолчанию | Описание |
|----------|-------------|----------|
| `host` | — | IP-адрес или hostname устройства |
| `community` | `public` | SNMP community string |
| `port` | `161` | UDP порт |

### Пример вывода

```
📡  SNMP-опрос устройства: 192.168.1.1:161  community='public'
───────────────────────────────────────────────────────────────────────────────────────────────
#   Порт   Скорость     MTU    Admin    Oper     MAC-адрес          Вх. трафик  Исх. трафик  Вх. пакеты  Исх. пакеты  Вх. ошибки  Исх. ошибки
───────────────────────────────────────────────────────────────────────────────────────────────
1   lo     10 Мбит/с   65536  ✅ up    ✅ up    —                  1.4 ГБ      1.4 ГБ       97 532 354  97 532 354   0           0
2   eth1   100 Мбит/с  1500   ✅ up    ✅ up    9c:d3:32:00:4c:63  1.1 ГБ      221.4 МБ     75 964 247  12 048 102   0           0
3   eth2   10 Мбит/с   1500   ✅ up    🔴 down  9c:d3:32:00:4c:64  23.9 КБ     27.7 КБ      102         89           0           0
4   eth0   10 Мбит/с   1500   ✅ up    🔴 down  9c:d3:32:00:4c:65  3.5 МБ      17.1 МБ      9 116       45 210       0           0
```

---

## 📋 Собираемые OID

| OID | Поле | Описание |
|-----|------|----------|
| `.1.3.6.1.2.1.2.2.1.2` | `ifDescr` | Имя интерфейса |
| `.1.3.6.1.2.1.2.2.1.5` | `ifSpeed` | Скорость (бит/с) |
| `.1.3.6.1.2.1.2.2.1.4` | `ifMtu` | MTU |
| `.1.3.6.1.2.1.2.2.1.6` | `ifPhysAddress` | MAC-адрес |
| `.1.3.6.1.2.1.2.2.1.7` | `ifAdminStatus` | Admin-статус (1=up, 2=down) |
| `.1.3.6.1.2.1.2.2.1.8` | `ifOperStatus` | Oper-статус (1=up, 2=down) |
| `.1.3.6.1.2.1.2.2.1.10` | `ifInOctets` | Входящий трафик |
| `.1.3.6.1.2.1.2.2.1.16` | `ifOutOctets` | Исходящий трафик |
| `.1.3.6.1.2.1.2.2.1.11` | `ifInUcastPkts` | Входящие пакеты |
| `.1.3.6.1.2.1.2.2.1.17` | `ifOutUcastPkts` | Исходящие пакеты |
| `.1.3.6.1.2.1.2.2.1.14` | `ifInErrors` | Входящие ошибки |
| `.1.3.6.1.2.1.2.2.1.20` | `ifOutErrors` | Исходящие ошибки |

---

## 🔧 Требования к устройству

- SNMP-агент с поддержкой **SNMPv2c**
- Открытый **UDP/161** (или другой порт, указанный аргументом)
- Доступная community string (по умолчанию `public`)

---

## 🗺️ Roadmap

- [ ] Поддержка `SNMPv3` (authPriv)
- [ ] `ifXTable` (64-bit счётчики, `ifHCInOctets`)
- [ ] Экспорт в JSON / CSV
- [ ] Фильтрация по имени интерфейса
- [ ] Режим мониторинга с периодическим опросом (`--watch`)

---

## 📄 Лицензия

[MIT](LICENSE)
