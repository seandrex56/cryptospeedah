import streamlit as st
import subprocess
import os
import time

# Dashboard Config
st.set_page_config(page_title="Drex Turbo", layout="wide")

# CSS para sa Matrix Look (Fixed Window)
st.markdown("""
    <style>
    .main { background-color: #000; }
    pre { color: #00FF00 !important; font-size: 13px !important; line-height: 1.1 !important; }
    .terminal { 
        background-color: #000; border: 2px solid #00FF00; padding: 15px; 
        height: 450px; overflow-y: hidden; display: flex; flex-direction: column-reverse;
    }
    </style>
    """, unsafe_allow_html=True)

st.title("🦊 DREX FOXEL ULTRA-HUNTER")

# Path sa binary na ginawa ng Dockerfile
HUNTER_BIN = "./btc_hunter"
LOG_FILE = "live_logs.txt"

def is_running():
    try:
        # Check kung buhay ang engine
        subprocess.check_output(["pgrep", "-f", HUNTER_BIN])
        return True
    except:
        return False

# Control Buttons
col1, col2 = st.columns(2)
with col1:
    if st.button("🚀 START HATAW"):
        if not is_running():
            open(LOG_FILE, "w").close()
            # Bubuhayin ang Rust sa background
            subprocess.Popen([HUNTER_BIN], stdout=open(LOG_FILE, "a"), stderr=open(LOG_FILE, "a"), start_new_session=True)
            st.rerun()

with col2:
    if st.button("🛑 STOP"):
        os.system(f"pkill -f {HUNTER_BIN}")
        st.rerun()

st.divider()

# The Matrix Box
st.subheader("📟 LIVE STREAM")
log_area = st.empty()

if is_running():
    while True:
        if os.path.exists(LOG_FILE):
            try:
                # Kunin ang huling 20 lines
                logs = subprocess.check_output(["tail", "-n", "20", LOG_FILE]).decode("utf-8")
                log_area.markdown(f'<div class="terminal"><pre>{logs}</pre></div>', unsafe_allow_html=True)
            except:
                pass
        time.sleep(0.1)
else:
    log_area.info("Engine is IDLE. Click START HATAW to begin the hunt.")
