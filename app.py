import streamlit as st
import subprocess
import os
import time

# I-setup ang page para magmukhang professional dashboard
st.set_page_config(
    page_title="Drex Foxel Hunter",
    page_icon="🦊",
    layout="wide",
    initial_sidebar_state="collapsed"
)

# --- CSS PARA SA MATRIX LOOK (HINDI MAG-S-SCROLL ANG BUONG PAGE) ---
st.markdown("""
    <style>
    .main { background-color: #000000; }
    div.stButton > button {
        width: 100%;
        border-radius: 5px;
        height: 3em;
        background-color: #111;
        color: #00FF00;
        border: 1px solid #00FF00;
    }
    div.stButton > button:hover {
        background-color: #00FF00;
        color: black;
    }
    .terminal-box {
        background-color: #000000;
        color: #00FF00;
        padding: 15px;
        border: 2px solid #00FF00;
        font-family: 'Courier New', monospace;
        height: 450px;
        overflow-y: hidden;
        display: flex;
        flex-direction: column-reverse;
    }
    </style>
    """, unsafe_allow_html=True)

st.title("🦊 DREX FOXEL ULTRA-HUNTER")
st.markdown("<p style='color: #00FF00;'>Status: Engine Ready for Brute Force</p>", unsafe_allow_html=True)

HUNTER_BIN = "/app/btc_hunter"
LOG_FILE = "live_logs.txt"

# --- ENGINE FUNCTIONS ---
def is_running():
    try:
        subprocess.check_output(["pgrep", "-f", HUNTER_BIN])
        return True
    except:
        return False

# --- CONTROLS ---
col1, col2 = st.columns(2)

with col1:
    if st.button("🚀 START HATAW MODE"):
        if not is_running():
            # I-clear ang log file para iwas lag
            with open(LOG_FILE, "w") as f:
                f.write("--- INITIALIZING ENGINE ---\n")
            
            # GHOST MODE: Kahit i-close ang app, tuloy ang Rust sa server
            subprocess.Popen(
                [HUNTER_BIN], 
                stdout=open(LOG_FILE, "a"), 
                stderr=open(LOG_FILE, "a"),
                start_new_session=True
            )
            st.rerun()

with col2:
    if st.button("🛑 STOP ENGINE"):
        os.system(f"pkill -f {HUNTER_BIN}")
        st.rerun()

st.divider()

# --- THE MATRIX TERMINAL ---
st.subheader("📟 LIVE BRUTE FORCE STREAM")
log_container = st.empty()

if is_running():
    # Walang delay na loop para sa "Matrix" feel
    while True:
        if os.path.exists(LOG_FILE):
            try:
                # 'tail' para mabilis ang paghatak ng huling 20 logs
                logs = subprocess.check_output(["tail", "-n", "20", LOG_FILE]).decode("utf-8")
                
                # I-render sa loob ng fixed-height box
                log_container.markdown(f"""
                <div class="terminal-box">
                    <pre style="white-space: pre-wrap; word-wrap: break-word; margin: 0; color: #00FF00;">{logs}</pre>
                </div>
                """, unsafe_allow_html=True)
            except:
                pass
        
        # Super bilis na update (0.1s para hindi ma-ban ng Hugging Face pero mabilis tignan)
        time.sleep(0.1)
else:
    log_container.info("Engine is IDLE. Click 'START HATAW' to begin the hunt.")
    
