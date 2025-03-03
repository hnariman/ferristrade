FROM fedora:41
RUN sudo dnf update -y
RUN sudo dnf install \
  clang \
  clang-devel \
  clang-tools-extra \
  libxkbcommon-devel \
  pkg-config \
  openssl-devel \
  libxcb-devel \
  gtk3-devel \
  atk \
  fontconfig-devel 
