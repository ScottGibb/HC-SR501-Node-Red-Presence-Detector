FROM raspbian/stretch

ARG PORT_NUM=60001
ARG PIR_PIN=26

RUN apt-get update -y
RUN apt-get install -y build-essential
RUN apt-get install python3-pip -y

COPY PresenceDetector.py.py /
COPY ../requirements.txt /

RUN pip3 install -r requirements.txt

EXPOSE $PORT_NUM
ENTRYPOINT ["python3"]
CMD ["-u","PresenceDetector.py,$PORT_NUM, $PIR_PIN"]
