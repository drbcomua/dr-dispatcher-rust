FROM ubuntu:latest
LABEL authors="dr"

ENTRYPOINT ["top", "-b"]