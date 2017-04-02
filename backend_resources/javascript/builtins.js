{
    i8: {
        size_of: function(input) { return 1; },
        serialize: function(input, buffer, offset) {
            buffer.writeInt8(input, offset);
            return offset+1;
        },
    },
    u8: {
        size_of: function(input) { return 1; },
        serialize: function(input, buffer, offset) {
            buffer.writeInt8(input, offset);
            return offset+1;
        },
    },
    i16: {
        size_of: function(input) { return 2; },
        serialize: function(input, buffer, offset) {
            buffer.writeInt16BE(input, offset);
            return offset+2;
        },
    },
    u16: {
        size_of: function(input) { return 2; },
        serialize: function(input, buffer, offset) {
            buffer.writeUInt16BE(input, offset);
            return offset+2;
        },
    },
    i32: {
        size_of: function(input) { return 4; },
        serialize: function(input, buffer, offset) {
            buffer.writeInt32BE(input, offset);
            return offset+4;
        },
    },
    u32: {
        size_of: function(input) { return 4; },
        serialize: function(input, buffer, offset) {
            buffer.writeUInt32BE(input, offset);
            return offset+4;
        },
    },
    i64: {
        size_of: function(input) { return 8; },
        serialize: function(input, buffer, offset) {
            buffer.writeInt64BE(input, offset);
            return offset+8;
        },
    },
    u64: {
        size_of: function(input) { return 8; },
        serialize: function(input, buffer, offset) {
            buffer.writeUInt64BE(input, offset);
            return offset+8;
        },
    },
    varint: {
        size_of: function(value) {
            let cursor = 0;
            while(value & ~0x7F) {
                value >>>= 7;
                cursor++;
            }
            return cursor + 1;
        },
    },
}
