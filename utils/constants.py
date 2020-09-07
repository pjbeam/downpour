#!/usr/bin/python3

data = {
    "PEER_ID_VERSION_NUMBER": "0000",
    "PEER_ID_PREFIX": "YM"
}

data["BASE_PEER_ID_STRING"] = "-" + data["PEER_ID_PREFIX"] + data["PEER_ID_VERSION_NUMBER"] + "-"

def constants():
    return data
