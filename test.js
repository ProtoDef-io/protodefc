var type_1_size_of = types["::u8"]["size_of"];
var type_1_serialize = types["::u8"]["serialize"];
var type_1_deserialize = types["::u8"]["deserialize"];
var type_2_size_of = types["::u64"]["size_of"];
var type_2_serialize = types["::u64"]["serialize"];
var type_2_deserialize = types["::u64"]["deserialize"];
var type_3_size_of = types["::f64"]["size_of"];
var type_3_serialize = types["::f64"]["serialize"];
var type_3_deserialize = types["::f64"]["deserialize"];
function type_4_size_of(input) {
    var count = 0;
    var type_var_4 = input;
    {
        var type_var_1 = type_var_4["x"];
        var type_var_2 = type_var_4["y"];
        var type_var_3 = type_var_4["z"];
        {
            var size = type_3_size_of(type_var_1);
            count = count + size;
        }
        {
            var size = type_3_size_of(type_var_2);
            count = count + size;
        }
        {
            var size = type_3_size_of(type_var_3);
            count = count + size;
        }
    }
    return count;
}
function type_4_serialize(input, buffer, offset) {
    var type_var_4 = input;
    {
        var type_var_1 = type_var_4["x"];
        var type_var_2 = type_var_4["y"];
        var type_var_3 = type_var_4["z"];
        {
            var offset = type_3_serialize(type_var_1, buffer, offset);
        }
        {
            var offset = type_3_serialize(type_var_2, buffer, offset);
        }
        {
            var offset = type_3_serialize(type_var_3, buffer, offset);
        }
    }
    return offset;
}
function type_4_deserialize(buffer, offset) {
    {
        {
            var [type_var_1, offset] = type_3_deserialize(buffer, offset);
        }
        {
            var [type_var_2, offset] = type_3_deserialize(buffer, offset);
        }
        {
            var [type_var_3, offset] = type_3_deserialize(buffer, offset);
        }
        var type_var_4 = { x: type_var_1, y: type_var_2, z: type_var_3 };
    }
    return [type_var_4, offset];
}
function type_5_size_of(input) {
    var count = 0;
    var type_var_11 = input;
    {
        var type_var_5 = type_var_11["entity_id"];
        var type_var_6 = type_var_11["position"];
        var type_var_10 = type_var_11["entity_type"];
        {
            switch (type_var_10.tag) {
                case "player": {
                    var type_11_int_val_0 = 0;
                    break;
                }
                case "zombie": {
                    var type_11_int_val_0 = 1;
                    break;
                }
            }
            var type_var_7 = type_11_int_val_0;
        }
        {
            var size = type_2_size_of(type_var_5);
            count = count + size;
        }
        {
            var size = type_4_size_of(type_var_6);
            count = count + size;
        }
        {
            var size = type_1_size_of(type_var_7);
            count = count + size;
        }
        {
            switch (type_var_10.tag) {
                case "player": {
                    var type_var_8 = type_var_10.data;
                    {
                    }
                    break;
                }
                case "zombie": {
                    var type_var_9 = type_var_10.data;
                    {
                    }
                    break;
                }
            }
        }
    }
    return count;
}
function type_5_serialize(input, buffer, offset) {
    var type_var_11 = input;
    {
        var type_var_5 = type_var_11["entity_id"];
        var type_var_6 = type_var_11["position"];
        var type_var_10 = type_var_11["entity_type"];
        {
            switch (type_var_10.tag) {
                case "player": {
                    var type_11_int_val_0 = 0;
                    break;
                }
                case "zombie": {
                    var type_11_int_val_0 = 1;
                    break;
                }
            }
            var type_var_7 = type_11_int_val_0;
        }
        {
            var offset = type_2_serialize(type_var_5, buffer, offset);
        }
        {
            var offset = type_4_serialize(type_var_6, buffer, offset);
        }
        {
            var offset = type_1_serialize(type_var_7, buffer, offset);
        }
        {
            switch (type_var_10.tag) {
                case "player": {
                    var type_var_8 = type_var_10.data;
                    {
                    }
                    break;
                }
                case "zombie": {
                    var type_var_9 = type_var_10.data;
                    {
                    }
                    break;
                }
            }
        }
    }
    return offset;
}
function type_5_deserialize(buffer, offset) {
    {
        {
            var [type_var_5, offset] = type_2_deserialize(buffer, offset);
        }
        {
            var [type_var_6, offset] = type_4_deserialize(buffer, offset);
        }
        {
            var [type_var_7, offset] = type_1_deserialize(buffer, offset);
        }
        {
            {
                var type_11_tag = type_var_7;
            }
            switch (type_11_tag) {
                case 0: {
                    {
                        var type_var_8 = {  };
                    }
                    var type_var_10 = { tag: "player", data: type_var_8 };
                    break;
                }
                case 1: {
                    {
                        var type_var_9 = {  };
                    }
                    var type_var_10 = { tag: "zombie", data: type_var_9 };
                    break;
                }
            }
        }
        var type_var_11 = { entity_id: type_var_5, position: type_var_6, entity_type: type_var_10 };
    }
    return [type_var_11, offset];
}
function type_6_size_of(input) {
    var count = 0;
    var type_var_20 = input;
    {
        var type_var_19 = type_var_20;
        {
            switch (type_var_19.tag) {
                case "handshake": {
                    var type_20_int_val_0 = 0;
                    break;
                }
                case "ping": {
                    var type_20_int_val_0 = 1;
                    break;
                }
                case "spawn_entity": {
                    var type_20_int_val_0 = 2;
                    break;
                }
            }
            var type_var_12 = type_20_int_val_0;
        }
        {
            var size = type_1_size_of(type_var_12);
            count = count + size;
        }
        {
            switch (type_var_19.tag) {
                case "handshake": {
                    var type_var_14 = type_var_19.data;
                    {
                        var type_var_13 = type_var_14["nonce"];
                        {
                            var size = type_2_size_of(type_var_13);
                            count = count + size;
                        }
                    }
                    break;
                }
                case "ping": {
                    var type_var_17 = type_var_19.data;
                    {
                        var type_var_15 = type_var_17["nonce"];
                        var type_var_16 = type_var_17["timestamp"];
                        {
                            var size = type_2_size_of(type_var_15);
                            count = count + size;
                        }
                        {
                            var size = type_2_size_of(type_var_16);
                            count = count + size;
                        }
                    }
                    break;
                }
                case "spawn_entity": {
                    var type_var_18 = type_var_19.data;
                    {
                        var size = type_5_size_of(type_var_18);
                        count = count + size;
                    }
                    break;
                }
            }
        }
    }
    return count;
}
function type_6_serialize(input, buffer, offset) {
    var type_var_20 = input;
    {
        var type_var_19 = type_var_20;
        {
            switch (type_var_19.tag) {
                case "handshake": {
                    var type_20_int_val_0 = 0;
                    break;
                }
                case "ping": {
                    var type_20_int_val_0 = 1;
                    break;
                }
                case "spawn_entity": {
                    var type_20_int_val_0 = 2;
                    break;
                }
            }
            var type_var_12 = type_20_int_val_0;
        }
        {
            var offset = type_1_serialize(type_var_12, buffer, offset);
        }
        {
            switch (type_var_19.tag) {
                case "handshake": {
                    var type_var_14 = type_var_19.data;
                    {
                        var type_var_13 = type_var_14["nonce"];
                        {
                            var offset = type_2_serialize(type_var_13, buffer, offset);
                        }
                    }
                    break;
                }
                case "ping": {
                    var type_var_17 = type_var_19.data;
                    {
                        var type_var_15 = type_var_17["nonce"];
                        var type_var_16 = type_var_17["timestamp"];
                        {
                            var offset = type_2_serialize(type_var_15, buffer, offset);
                        }
                        {
                            var offset = type_2_serialize(type_var_16, buffer, offset);
                        }
                    }
                    break;
                }
                case "spawn_entity": {
                    var type_var_18 = type_var_19.data;
                    {
                        var offset = type_5_serialize(type_var_18, buffer, offset);
                    }
                    break;
                }
            }
        }
    }
    return offset;
}
function type_6_deserialize(buffer, offset) {
    {
        {
            var [type_var_12, offset] = type_1_deserialize(buffer, offset);
        }
        {
            {
                var type_20_tag = type_var_12;
            }
            switch (type_20_tag) {
                case 0: {
                    {
                        {
                            var [type_var_13, offset] = type_2_deserialize(buffer, offset);
                        }
                        var type_var_14 = { nonce: type_var_13 };
                    }
                    var type_var_19 = { tag: "handshake", data: type_var_14 };
                    break;
                }
                case 1: {
                    {
                        {
                            var [type_var_15, offset] = type_2_deserialize(buffer, offset);
                        }
                        {
                            var [type_var_16, offset] = type_2_deserialize(buffer, offset);
                        }
                        var type_var_17 = { nonce: type_var_15, timestamp: type_var_16 };
                    }
                    var type_var_19 = { tag: "ping", data: type_var_17 };
                    break;
                }
                case 2: {
                    {
                        var [type_var_18, offset] = type_5_deserialize(buffer, offset);
                    }
                    var type_var_19 = { tag: "spawn_entity", data: type_var_18 };
                    break;
                }
            }
        }
        var type_var_20 = type_var_19;
    }
    return [type_var_20, offset];
}
var exports = {
"::u8": {"size_of": type_1_size_of, "serialize": type_1_serialize, "deserialize": type_1_deserialize },
"::u64": {"size_of": type_2_size_of, "serialize": type_2_serialize, "deserialize": type_2_deserialize },
"::f64": {"size_of": type_3_size_of, "serialize": type_3_serialize, "deserialize": type_3_deserialize },
"::position": {"size_of": type_4_size_of, "serialize": type_4_serialize, "deserialize": type_4_deserialize },
"::entity_data": {"size_of": type_5_size_of, "serialize": type_5_serialize, "deserialize": type_5_deserialize },
"::packet": {"size_of": type_6_size_of, "serialize": type_6_serialize, "deserialize": type_6_deserialize }
};
