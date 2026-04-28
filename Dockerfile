FROM rust:latest

# 1. Install Python at Streamlit
RUN apt-get update && apt-get install -y python3 python3-pip && rm -rf /var/lib/apt/lists/*
RUN pip3 install streamlit --break-system-packages

WORKDIR /app

# 2. Kopyahin ang lahat ng files
COPY . .

# 3. I-build ang Rust project
RUN cargo build --release

# 4. I-rename ang binary para sigurado
RUN BIN_NAME=$(grep '^name =' Cargo.toml | sed 's/.*= "//;s/"//') && \
    cp target/release/$BIN_NAME ./btc_hunter && \
    chmod +x ./btc_hunter

# 5. Requirement para sa Streamlit (Importante sa Render)
RUN echo "streamlit" > requirements.txt

# 6. Gamitin ang dynamic port ng Render
CMD ["sh", "-c", "streamlit run app.py --server.port $PORT --server.address 0.0.0.0"]
