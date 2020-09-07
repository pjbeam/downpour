#!/usr/bin/python3

import parse_bencode, urllib.parse, hashlib, random, port_logic
from constants import constants

test_path = "../test_data/She_Gods_of_Shark_Reef.avi.torrent"

def get_decoded_metadata(file_path):
    return parse_bencode.parse_file(file_path)

def url_encode(str):
    return urllib.parse.quote(str)

def generate_peer_id():
    base_peer_string = constants()["BASE_PEER_ID_STRING"]
    random_numbers = "%0.12d" % random.randint(0,999999999999)
    peer_id_string = base_peer_string + random_numbers
    return url_encode(peer_id_string)

def get_info_hash(info_value):
    bencoded_info_dict = parse_bencode.b_encode(info_value)
    info_hash = hashlib.sha1(bencoded_info_dict).digest()
    return url_encode(info_hash)

def get_left(left_value):
    return str(left_value)

def build_query_dict(file_path):
    decoded_metadata = get_decoded_metadata(file_path)
    print(decoded_metadata[b'announce'])
    info_hash = get_info_hash(decoded_metadata[b'info'])
    peer_id = generate_peer_id()
    port = port_logic.get_port()
    left = get_left(decoded_metadata[b'info'][b'length'])
    return {
        "info_hash": info_hash,
        "peer_id": peer_id,
        "port": port,
        "left": left,
        "uploaded": "0",
        "downloaded": "0"
    }

def build_query(file_path):
    params_dict = build_query_dict(file_path)
    base_query_string = "?"
    for key in params_dict:
        base_query_string += key + "=" + params_dict[key] + "&"

    return base_query_string

print(build_query(test_path))
