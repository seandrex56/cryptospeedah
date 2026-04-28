# Gamitin ang Rust image para sa compilation
FROM rust:latest

# 1. Install Python at Streamlit
RUN apt-get update && apt-get install -y python3 python3-pip && rm -rf /var/lib/apt/lists/*
RUN pip3 install streamlit --break-system-packages

WORKDIR /app

# 2. Kopyahin ang lahat ng files (Cargo.toml, src, app.py)
COPY . .

# 3. I-build ang Rust project
RUN cargo build --release

# 4. DITO ANG SOLUSYON: I-rename natin yung output para laging "btc_hunter" ang pangalan
RUN BIN_NAME=$(grep '^name =' Cargo.toml | sed 's/.*= "//;s/"//') && \
    cp target/release/$BIN_NAME ./btc_hunter && \
    chmod +x ./btc_hunter

# 5. Silipin natin kung nandoon ba talaga (para sa logs natin)
RUN ls -l ./btc_hunter

EXPOSE 7860

CMD ["streamlit", "run", "app.py", "--server.port", "7860", "--server.address", "0.0.0.0"]
