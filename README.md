# 🕒 Arch Linux Timer (Rust CLI)

Ein einfacher Timer für das Terminal für Arch Linux – geschrieben in Rust.
Beispiel: `flas-timer 2m30s` zählt 2 Minuten und 30 Sekunden herunter.

---

## 🔧 Funktionen

* Einfache Eingabe wie `10s`, `5m`, `1h30m20s`
* Live-Countdown im Terminal
* Signalton (`beep`) am Ende

---

## 🖥️ Beispiel

```bash
$ flas-timer 2m30s
⏱️  Noch 02:30
⏱️  Noch 02:29
...
✅ Zeit abgelaufen!
```

---

## 📦 Installation

**1. Repository klonen**

```bash
git clone https://github.com/FLAS-Forum/timer.git
cd timer
```

**2. Paket bauen und installieren**

```bash
makepkg -si
```

> **Hinweis bei möglichem Fehler:**
> Falls beim Bauen folgende Meldung erscheint:
>
> ```
> error: rustup could not choose a version of cargo to run
> ```
>
> dann einfach ausführen:
>
> ```bash
> rustup default stable
> ```
>
> und den Build erneut starten. Also den befehl von Schritt 2 ausführen.
>
> Die genaue Rust-Version spielt keine Rolle — ich empfehle die aktuelle stabile Version (`rustup update`).

---

## ❌ Deinstallation

```bash
sudo pacman -Rns flas-timer flas-timer-debug
```

---

## ▶️ Verwendung

```bash
flas-timer 10s
flas-timer 5m
flas-timer 1h30m20s
```

Das Programm zählt die angegebene Zeit herunter und gibt am Ende einen Signalton aus.

---

## 💡 Tipp: Kürzerer Befehl `timer`

Wenn du statt `flas-timer` einfach nur `timer` tippen möchtest:

```bash
echo 'alias timer="flas-timer"' >> ~/.bashrc
source ~/.bashrc
```

---

## 📜 Lizenz

Dieses Projekt ist unter der **MIT-Lizenz** lizenziert.
