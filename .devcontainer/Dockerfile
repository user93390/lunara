FROM rust:alpine

ARG USERNAME=dev
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN apk add --no-cache sudo bash shadow \
    && groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

COPY install_libs.sh .
RUN chmod +x install_libs.sh

USER $USERNAME

RUN ./install_libs.sh

RUN mkdir -p ~/libs
RUN git clone --single-branch --branch master https://github.com/flutter/flutter.git ~/libs/flutter


ENV PATH="/home/${USERNAME}/libs/flutter/bin:${PATH}"