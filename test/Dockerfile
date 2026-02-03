FROM ghcr.io/servercontainers/samba:smbd-only-latest

ENV NETBIOS_DISABLE=1
ENV AVAHI_DISABLE=1
ENV WSDD2_DISABLE=1

# Enable only 2.0.2 - 3.1.1 dialects.
ENV SAMBA_GLOBAL_CONFIG_server_SPACE_min_SPACE_protocol=SMB2_02
ENV SAMBA_GLOBAL_CONFIG_server_SPACE_max_SPACE_protocol=SMB3_11

# Increase connection limits and set aggressive cleanup
ENV SAMBA_GLOBAL_CONFIG_max_SPACE_connections=0
ENV SAMBA_GLOBAL_CONFIG_deadtime=5
ENV SAMBA_GLOBAL_CONFIG_keepalive=30
ENV SAMBA_GLOBAL_CONFIG_socket_SPACE_options="TCP_NODELAY SO_RCVBUF=131072 SO_SNDBUF=131072"

RUN mkdir -p /shares/MyShare /shares/PublicShare && \
    chmod -R 777 /shares

RUN mkdir /shares/MyShare/bla && echo "test" > /shares/MyShare/bla/blabla.txt