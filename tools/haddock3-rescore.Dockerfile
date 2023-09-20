FROM python:3.9
LABEL author="Rodrigo V. Honorato <r.vargashonorato@uu.nl>"
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  build-essential \
  git \
  ca-certificates \
  &&  \
  apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /opt
RUN git clone --recursive https://github.com/haddocking/haddock3.git
WORKDIR /opt/haddock3
RUN git checkout interactive_rescoring
WORKDIR /opt/haddock3/src/fcc/src
RUN make
WORKDIR /opt/haddock3
RUN pip install --no-cache-dir -r requirements.txt && \
  python setup.py develop
WORKDIR /opt/haddock3/bin
COPY tools/cns .
WORKDIR /
ENV PYTHONPATH=/opt/haddock3/src