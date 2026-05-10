import tkinter as tk

app = tk.Tk()
app.title("TORCS Drivers")

BASE_PORT = 3001

BG = "#2b2b2b"   # dark grey
FG = "#ffffff"   # white

# font fallback (Tkinter will pick first available)
FONT = ("Consolas", 11)  # primary
FALLBACKS = ("DejaVu Sans Mono", "Courier New")

# --- app ---
app.configure(bg=BG)

frame = tk.Frame(app, bg=BG)
frame.pack(side="left", padx=10, pady=10)

labels = []

def make_font():
    # try primary, fall back if unavailable
    available = set(app.tk.call("font", "families"))
    if FONT[0] in available:
        return FONT
    for f in FALLBACKS:
        if f in available:
            return (f, FONT[1])
    return ("Courier", FONT[1])  # last resort

font = make_font()

# create driver labels
for i in range(2):
    text = f"driver {i}: waiting..."
    lbl = tk.Label(frame, text=text, anchor="w",
                   bg=BG, fg=FG, font=font)
    lbl.pack(fill="x")
    labels.append(lbl)

# --- example state updates ---
def update_status(i, status):
    labels[i].config(text=f"driver {i}: {status}")

def simulate():
    for i in range(2):
        port = BASE_PORT + i
        (i, f"connecting on port {port}")
    app.after(2000, lambda: update_status(0, "waiting..."))

app.after(1000, simulate)

app.mainloop()