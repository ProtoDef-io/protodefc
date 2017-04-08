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
    var type_input_0 = input;
    {
        var type_input_1 = type_input_0["x"];
        {
            var size = type_3_size_of(type_input_1);
            count = count + size;
        }
        var type_input_2 = type_input_0["y"];
        {
            var size = type_3_size_of(type_input_2);
            count = count + size;
        }
        var type_input_3 = type_input_0["z"];
        {
            var size = type_3_size_of(type_input_3);
            count = count + size;
        }
    }
    return count;
}
function type_4_serialize(input, buffer, offset) {
    var type_input_0 = input;
    {
        var type_input_1 = type_input_0["x"];
        {
            var offset = type_3_serialize(type_input_1, buffer, offset);
        }
        var type_input_2 = type_input_0["y"];
        {
            var offset = type_3_serialize(type_input_2, buffer, offset);
        }
        var type_input_3 = type_input_0["z"];
        {
            var offset = type_3_serialize(type_input_3, buffer, offset);
        }
    }
    return offset;
}
function type_4_deserialize(buffer, offset) {
    {
        {
            var [type_output_1, offset] = type_3_deserialize(buffer, offset);
        }
        {
            var [type_output_2, offset] = type_3_deserialize(buffer, offset);
        }
        {
            var [type_output_3, offset] = type_3_deserialize(buffer, offset);
        }
        var type_output_0 = { x: type_output_1, y: type_output_2, z: type_output_3 };
    }
    return [type_output_0, offset];
}
function type_5_size_of(input) {
    var count = 0;
    var type_input_4 = input;
    {
        var type_input_5 = type_input_4["entity_id"];
        {
            var size = type_2_size_of(type_input_5);
            count = count + size;
        }
        var type_input_6 = type_input_4["position"];
        {
            var size = type_4_size_of(type_input_6);
            count = count + size;
        }
        var type_input_7 = type_input_4["entity_type"];
        switch (type_input_7.tag) {
            case "player": {
                var type_input_7 = 0;
                break;
            }
            case "zombie": {
                var type_input_7 = 1;
                break;
            }
        }
        {
            var size = type_1_size_of(type_input_7);
            count = count + size;
        }
        var type_input_8 = type_input_4["entity_type"];
        {
            switch (type_input_8.tag) {
                case "player": {
                    var type_input_9 = type_input_8.data;
                    {
                    }
                    break;
                }
                case "zombie": {
                    var type_input_10 = type_input_8.data;
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
    var type_input_4 = input;
    {
        var type_input_5 = type_input_4["entity_id"];
        {
            var offset = type_2_serialize(type_input_5, buffer, offset);
        }
        var type_input_6 = type_input_4["position"];
        {
            var offset = type_4_serialize(type_input_6, buffer, offset);
        }
        var type_input_7 = type_input_4["entity_type"];
        switch (type_input_7.tag) {
            case "player": {
                var type_input_7 = 0;
                break;
            }
            case "zombie": {
                var type_input_7 = 1;
                break;
            }
        }
        {
            var offset = type_1_serialize(type_input_7, buffer, offset);
        }
        var type_input_8 = type_input_4["entity_type"];
        {
            switch (type_input_8.tag) {
                case "player": {
                    var type_input_9 = type_input_8.data;
                    {
                    }
                    break;
                }
                case "zombie": {
                    var type_input_10 = type_input_8.data;
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
            var [type_output_5, offset] = type_2_deserialize(buffer, offset);
        }
        {
            var [type_output_6, offset] = type_4_deserialize(buffer, offset);
        }
        {
            var [type_output_7, offset] = type_1_deserialize(buffer, offset);
        }
        {
            switch (type_output_7) {
                case 0: {
                    {
                        var type_output_9 = {  };
                    }
                    var type_output_8 = { tag: "player", data: type_output_9 };
                    break;
                }
                case 1: {
                    {
                        var type_output_10 = {  };
                    }
                    var type_output_8 = { tag: "zombie", data: type_output_10 };
                    break;
                }
            }
        }
        var type_output_4 = { entity_id: type_output_5, position: type_output_6, entity_type: type_output_8 };
    }
    return [type_output_4, offset];
}
function type_6_size_of(input) {
    var count = 0;
    var type_input_11 = input;
    {
        var type_input_12 = type_input_11;
        switch (type_input_12.tag) {
            case "handshake": {
                var type_input_12 = 0;
                break;
            }
            case "ping": {
                var type_input_12 = 1;
                break;
            }
            case "spawn_entity": {
                var type_input_12 = 2;
                break;
            }
        }
        {
            var size = type_1_size_of(type_input_12);
            count = count + size;
        }
        var type_input_13 = type_input_11;
        {
            switch (type_input_13.tag) {
                case "handshake": {
                    var type_input_14 = type_input_13.data;
                    {
                        var type_input_15 = type_input_14["nonce"];
                        {
                            var size = type_2_size_of(type_input_15);
                            count = count + size;
                        }
                    }
                    break;
                }
                case "ping": {
                    var type_input_16 = type_input_13.data;
                    {
                        var type_input_17 = type_input_16["nonce"];
                        {
                            var size = type_2_size_of(type_input_17);
                            count = count + size;
                        }
                        var type_input_18 = type_input_16["timestamp"];
                        {
                            var size = type_2_size_of(type_input_18);
                            count = count + size;
                        }
                    }
                    break;
                }
                case "spawn_entity": {
                    var type_input_19 = type_input_13.data;
                    {
                        var size = type_5_size_of(type_input_19);
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
    var type_input_11 = input;
    {
        var type_input_12 = type_input_11;
        switch (type_input_12.tag) {
            case "handshake": {
                var type_input_12 = 0;
                break;
            }
            case "ping": {
                var type_input_12 = 1;
                break;
            }
            case "spawn_entity": {
                var type_input_12 = 2;
                break;
            }
        }
        {
            var offset = type_1_serialize(type_input_12, buffer, offset);
        }
        var type_input_13 = type_input_11;
        {
            switch (type_input_13.tag) {
                case "handshake": {
                    var type_input_14 = type_input_13.data;
                    {
                        var type_input_15 = type_input_14["nonce"];
                        {
                            var offset = type_2_serialize(type_input_15, buffer, offset);
                        }
                    }
                    break;
                }
                case "ping": {
                    var type_input_16 = type_input_13.data;
                    {
                        var type_input_17 = type_input_16["nonce"];
                        {
                            var offset = type_2_serialize(type_input_17, buffer, offset);
                        }
                        var type_input_18 = type_input_16["timestamp"];
                        {
                            var offset = type_2_serialize(type_input_18, buffer, offset);
                        }
                    }
                    break;
                }
                case "spawn_entity": {
                    var type_input_19 = type_input_13.data;
                    {
                        var offset = type_5_serialize(type_input_19, buffer, offset);
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
            var [type_output_12, offset] = type_1_deserialize(buffer, offset);
        }
        {
            switch (type_output_12) {
                case 0: {
                    {
                        {
                            var [type_output_15, offset] = type_2_deserialize(buffer, offset);
                        }
                        var type_output_14 = { nonce: type_output_15 };
                    }
                    var type_output_13 = { tag: "handshake", data: type_output_14 };
                    break;
                }
                case 1: {
                    {
                        {
                            var [type_output_17, offset] = type_2_deserialize(buffer, offset);
                        }
                        {
                            var [type_output_18, offset] = type_2_deserialize(buffer, offset);
                        }
                        var type_output_16 = { nonce: type_output_17, timestamp: type_output_18 };
                    }
                    var type_output_13 = { tag: "ping", data: type_output_16 };
                    break;
                }
                case 2: {
                    {
                        var [type_output_19, offset] = type_5_deserialize(buffer, offset);
                    }
                    var type_output_13 = { tag: "spawn_entity", data: type_output_19 };
                    break;
                }
            }
        }
        var type_output_11 = type_output_13;
    }
    return [type_output_11, offset];
}
var exports = {
"::u8": {"size_of": type_1_size_of, "serialize": type_1_serialize, "deserialize": type_1_deserialize },
"::u64": {"size_of": type_2_size_of, "serialize": type_2_serialize, "deserialize": type_2_deserialize },
"::f64": {"size_of": type_3_size_of, "serialize": type_3_serialize, "deserialize": type_3_deserialize },
"::position": {"size_of": type_4_size_of, "serialize": type_4_serialize, "deserialize": type_4_deserialize },
"::entity_data": {"size_of": type_5_size_of, "serialize": type_5_serialize, "deserialize": type_5_deserialize },
"::packet": {"size_of": type_6_size_of, "serialize": type_6_serialize, "deserialize": type_6_deserialize }
};
