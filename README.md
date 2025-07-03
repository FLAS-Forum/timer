# 🕒 Arch Linux Timer (Rust CLI)

Ein einfacher Timer für das Terminal für Arch Linux – geschrieben in Rust. Gib z. B. timer 2m30s ein und sieh die Zeit rückwärts zählen.

## 🔧 Funktionen

- CLI-Timer mit einfacher Eingabe (z. B. 10s, 5m, 1h30m20s)
- Live-Countdown im Terminal
- Beep am Ende der Zeit

---

## 🖥️ Beispiel

$ timer 2m30s
⏱️  Noch 02:30
⏱️  Noch 02:29
...
✅ Zeit abgelaufen!

---

## Installation und Deinstallation

### Repository klonen, Paket bauen und installieren

git clone https://github.com/FLAS-Forum/timer.git  
cd timer  
makepkg -si

### Deinstallation

sudo pacman -Rns timer timer-debug

---

## Verwendung

timer 10s  
timer 5m  
timer 1h30m20s

Das Programm zählt die angegebene Zeit herunter und gibt am Ende einen Signalton aus.

---

## Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert.
