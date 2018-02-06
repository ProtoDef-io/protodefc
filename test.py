def type_8_size_of(input):
    count = 0
    type_var_3 = input
    type_var_1 = type_var_3["nonce"]
    type_var_2 = type_var_3["timestamp"]
    size = type_2_size_of(type_var_1)
    count = count + size
    size = type_2_size_of(type_var_2)
    count = count + size
    return count
def type_8_serialize(input, buffer, offset):
    type_var_3 = input
    type_var_1 = type_var_3["nonce"]
    type_var_2 = type_var_3["timestamp"]
    offset = type_2_serialize(type_var_1, buffer, offset)
    offset = type_2_serialize(type_var_2, buffer, offset)
    return offset
def type_8_deserialize(buffer, offset):
    (type_var_1, offset) = type_2_deserialize(buffer, offset)
    (type_var_2, offset) = type_2_deserialize(buffer, offset)
    type_var_3 = { "nonce": type_var_1, "timestamp": type_var_2 }
    return (type_var_3, offset)
def type_9_size_of(input):
    count = 0
    type_var_10 = input
    type_var_9 = type_var_10
    if type_var_9.tag == "handshake":
        type_10_int_val_0 = 0
    elif type_var_9.tag == "ping":
        type_10_int_val_0 = 1
    elif type_var_9.tag == "spawn_entity":
        type_10_int_val_0 = 2
    else:
        raise Exception("error")
    type_var_4 = type_10_int_val_0
    size = type_1_size_of(type_var_4)
    count = count + size
    if type_var_9.tag == "handshake":
        type_var_6 = type_var_9.data
        type_var_5 = type_var_6["nonce"]
        size = type_2_size_of(type_var_5)
        count = count + size
    elif type_var_9.tag == "ping":
        type_var_7 = type_var_9.data
        size = type_8_size_of(type_var_7)
        count = count + size
    elif type_var_9.tag == "spawn_entity":
        type_var_8 = type_var_9.data
        size = type_7_size_of(type_var_8)
        count = count + size
    else:
        raise Exception("error")
    return count
def type_9_serialize(input, buffer, offset):
    type_var_10 = input
    type_var_9 = type_var_10
    if type_var_9.tag == "handshake":
        type_10_int_val_0 = 0
    elif type_var_9.tag == "ping":
        type_10_int_val_0 = 1
    elif type_var_9.tag == "spawn_entity":
        type_10_int_val_0 = 2
    else:
        raise Exception("error")
    type_var_4 = type_10_int_val_0
    offset = type_1_serialize(type_var_4, buffer, offset)
    if type_var_9.tag == "handshake":
        type_var_6 = type_var_9.data
        type_var_5 = type_var_6["nonce"]
        offset = type_2_serialize(type_var_5, buffer, offset)
    elif type_var_9.tag == "ping":
        type_var_7 = type_var_9.data
        offset = type_8_serialize(type_var_7, buffer, offset)
    elif type_var_9.tag == "spawn_entity":
        type_var_8 = type_var_9.data
        offset = type_7_serialize(type_var_8, buffer, offset)
    else:
        raise Exception("error")
    return offset
def type_9_deserialize(buffer, offset):
    (type_var_4, offset) = type_1_deserialize(buffer, offset)
    type_10_tag = type_var_4
    if type_10_tag == 0:
        (type_var_5, offset) = type_2_deserialize(buffer, offset)
        type_var_6 = { "nonce": type_var_5 }
        type_var_9 = { tag: "handshake", data: type_var_6 }
    elif type_10_tag == 1:
        (type_var_7, offset) = type_8_deserialize(buffer, offset)
        type_var_9 = { tag: "ping", data: type_var_7 }
    elif type_10_tag == 2:
        (type_var_8, offset) = type_7_deserialize(buffer, offset)
        type_var_9 = { tag: "spawn_entity", data: type_var_8 }
    else:
        raise Exception("error")
    type_var_10 = type_var_9
    return (type_var_10, offset)
type_1_size_of = types["::u8"]["size_of"]
type_1_serialize = types["::u8"]["serialize"]
type_1_deserialize = types["::u8"]["deserialize"]
type_2_size_of = types["::u64"]["size_of"]
type_2_serialize = types["::u64"]["serialize"]
type_2_deserialize = types["::u64"]["deserialize"]
type_3_size_of = types["::f64"]["size_of"]
type_3_serialize = types["::f64"]["serialize"]
type_3_deserialize = types["::f64"]["deserialize"]
type_4_size_of = types["::sized_string"]["size_of"]
type_4_serialize = types["::sized_string"]["serialize"]
type_4_deserialize = types["::sized_string"]["deserialize"]
def type_5_size_of(input):
    count = 0
    type_var_13 = input
    type_var_12 = type_var_13
    type_13_int_val_0 = len(type_var_12.encode('utf8'))
    type_var_11 = type_13_int_val_0
    size = type_1_size_of(type_var_11)
    count = count + size
    size = type_4_size_of(type_var_12)
    count = count + size
    return count
def type_5_serialize(input, buffer, offset):
    type_var_13 = input
    type_var_12 = type_var_13
    type_13_int_val_0 = len(type_var_12.encode('utf8'))
    type_var_11 = type_13_int_val_0
    offset = type_1_serialize(type_var_11, buffer, offset)
    offset = type_4_serialize(type_var_12, buffer, offset)
    return offset
def type_5_deserialize(buffer, offset):
    (type_var_11, offset) = type_1_deserialize(buffer, offset)
    arg_0 = type_var_11
    (type_var_12, offset) = type_4_deserialize(buffer, offset, arg_0)
    type_var_13 = type_var_12
    return (type_var_13, offset)
def type_6_size_of(input):
    count = 0
    type_var_17 = input
    type_var_14 = type_var_17["x"]
    type_var_15 = type_var_17["y"]
    type_var_16 = type_var_17["z"]
    size = type_3_size_of(type_var_14)
    count = count + size
    size = type_3_size_of(type_var_15)
    count = count + size
    size = type_3_size_of(type_var_16)
    count = count + size
    return count
def type_6_serialize(input, buffer, offset):
    type_var_17 = input
    type_var_14 = type_var_17["x"]
    type_var_15 = type_var_17["y"]
    type_var_16 = type_var_17["z"]
    offset = type_3_serialize(type_var_14, buffer, offset)
    offset = type_3_serialize(type_var_15, buffer, offset)
    offset = type_3_serialize(type_var_16, buffer, offset)
    return offset
def type_6_deserialize(buffer, offset):
    (type_var_14, offset) = type_3_deserialize(buffer, offset)
    (type_var_15, offset) = type_3_deserialize(buffer, offset)
    (type_var_16, offset) = type_3_deserialize(buffer, offset)
    type_var_17 = { "x": type_var_14, "y": type_var_15, "z": type_var_16 }
    return (type_var_17, offset)
def type_7_size_of(input):
    count = 0
    type_var_24 = input
    type_var_18 = type_var_24["entity_id"]
    type_var_19 = type_var_24["position"]
    type_var_23 = type_var_24["entity_type"]
    if type_var_23.tag == "player":
        type_24_int_val_0 = 0
    elif type_var_23.tag == "zombie":
        type_24_int_val_0 = 1
    else:
        raise Exception("error")
    type_var_20 = type_24_int_val_0
    size = type_2_size_of(type_var_18)
    count = count + size
    size = type_6_size_of(type_var_19)
    count = count + size
    size = type_1_size_of(type_var_20)
    count = count + size
    if type_var_23.tag == "player":
        type_var_21 = type_var_23.data
    elif type_var_23.tag == "zombie":
        type_var_22 = type_var_23.data
    else:
        raise Exception("error")
    return count
def type_7_serialize(input, buffer, offset):
    type_var_24 = input
    type_var_18 = type_var_24["entity_id"]
    type_var_19 = type_var_24["position"]
    type_var_23 = type_var_24["entity_type"]
    if type_var_23.tag == "player":
        type_24_int_val_0 = 0
    elif type_var_23.tag == "zombie":
        type_24_int_val_0 = 1
    else:
        raise Exception("error")
    type_var_20 = type_24_int_val_0
    offset = type_2_serialize(type_var_18, buffer, offset)
    offset = type_6_serialize(type_var_19, buffer, offset)
    offset = type_1_serialize(type_var_20, buffer, offset)
    if type_var_23.tag == "player":
        type_var_21 = type_var_23.data
    elif type_var_23.tag == "zombie":
        type_var_22 = type_var_23.data
    else:
        raise Exception("error")
    return offset
def type_7_deserialize(buffer, offset):
    (type_var_18, offset) = type_2_deserialize(buffer, offset)
    (type_var_19, offset) = type_6_deserialize(buffer, offset)
    (type_var_20, offset) = type_1_deserialize(buffer, offset)
    type_24_tag = type_var_20
    if type_24_tag == 0:
        type_var_21 = {  }
        type_var_23 = { tag: "player", data: type_var_21 }
    elif type_24_tag == 1:
        type_var_22 = {  }
        type_var_23 = { tag: "zombie", data: type_var_22 }
    else:
        raise Exception("error")
    type_var_24 = { "entity_id": type_var_18, "position": type_var_19, "entity_type": type_var_23 }
    return (type_var_24, offset)
exports = {
"::test::packet": {"size_of": type_9_size_of, "serialize": type_9_serialize, "deserialize": type_9_deserialize }
}
