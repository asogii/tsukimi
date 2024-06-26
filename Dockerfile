FROM archlinux:latest as builder

ENV CARGO_TERM_COLOR=always \
    CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    RUST_BACKTRACE=full

RUN pacman -Syu --noconfirm &&\
    pacman -S --noconfirm git base-devel sudo

RUN useradd -m -G wheel -s /bin/bash alice \
    && echo 'alice:password' | chpasswd

RUN echo '%wheel ALL=(ALL) NOPASSWD: ALL' > /etc/sudoers.d/99_wheel

USER alice

WORKDIR /home/alice

RUN git clone https://aur.archlinux.org/paru.git \
    && cd paru \
    && makepkg -si --noconfirm

COPY . .

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y &&\
    export PATH=$HOME/.cargo/bin:$PATH &&\
    sudo pacman -S --noconfirm libadwaita mpv gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad gst-plugins-ugly &&\
    paru -S clapper &&\
    cargo build --release --locked &&\
    cargo install cargo-deb --no-default-features &&\
    cargo deb

RUN wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-$(uname -m).AppImage -O /usr/local/bin/appimagetool &&\
    chmod +x /usr/local/bin/appimagetool &&\
    sed -i 's|AI\x02|\x00\x00\x00|' /usr/local/bin/appimagetool &&\
    cargo install cargo-appimage &&\
    APPIMAGE_EXTRACT_AND_RUN=1 cargo appimage

FROM ubuntu:latest

WORKDIR /usr/src/tsukimi

VOLUME /usr/src/tsukimi

COPY --from=builder /home/alice/target/release/tsukimi /usr/src/tsukimi/

COPY --from=builder /home/alice/target/debian/*.deb /usr/src/tsukimi/

COPY --from=builder /home/alice/moe.tsuna.tsukimi.gschema.xml /usr/src/tsukimi/

ENTRYPOINT ["sleep","3600"]
