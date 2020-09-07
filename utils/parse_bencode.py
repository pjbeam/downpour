#!/usr/bin/python3

import sys, os, bencodepy

def b_decode(bencoded_data):
    return bencodepy.decode(bencoded_data)

def b_encode(b_decoded_data):
    return bencodepy.encode(b_decoded_data)

def parse_file(file_path):
    with open(file_path, "rb") as bencode_file:
        bencode_data = bencode_file.read()
        return b_decode(bencode_data)
