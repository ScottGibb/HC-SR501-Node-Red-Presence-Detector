"""
Simple TCP Presence Detector Script to flag to Node Red that someone is present
"""
import logging
import socket

# Socket
import sys
import time


IP_ADDRESS = "0.0.0.0"
PORT_NUM = sys.argv[1]
PIR_PIN = sys.argv[2]

log = logging.getLogger('Transmitter Logger')

def setup_logging():
    """
    Sets up the Logger object log for use throughout the script
    """
    log.setLevel(logging.DEBUG)
    format = logging.Formatter("%(asctime)s - %(name)s - %(levelname)s - %(message)s")
    ch = logging.StreamHandler(sys.stdout)
    ch.setFormatter(format)
    log.addHandler(ch)


def main():
    """
    The main application of the program
    """
    global sock
    try:
        setup_logging()
        GPIO.setmode(GPIO.BOARD)
        GPIO.setup(PIR_PIN, GPIO.IN, pull_up_down= GPIO.PUD_DOWN)

        while True:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.bind((IP_ADDRESS, PORT_NUM))
            sock.listen()
            log.info("Listening on Port {}".format(PORT_NUM))
            conn, addr = sock.accept()
            log.info("Accepted Connection")
            sockFile = conn.makefile()

            """
            Blocking Polling Loop, repeatably checks the Sensor pins using a blocking function and sends presence detected when relevant. If write function fails, the code will attempt to reconnect
            """
            while True:
                GPIO.wait_for_edge(PIR_PIN, GPIO.RISING)
                log.info("Detected a Presence, notifying Node Red")
                try:
                    sockFile.write("Detected")
                except:
                    log.error("Server connection failed")
                    log.error("Closing sock and attempting server reconnection")
                    sock.close()
                    break


    finally:
        sock.close()
        GPIO.cleanup()