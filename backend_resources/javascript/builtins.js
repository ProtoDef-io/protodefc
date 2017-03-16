{
    i8: {
        size_of: function(input) { return 1; },
    },
    u8: {
        size_of: function(input) { return 1; },
    },
    i16: {
        size_of: function(input) { return 2; },
    },
    u16: {
        size_of: function(input) { return 2; },
    },
    i32: {
        size_of: function(input) { return 4; },
    },
    u32: {
        size_of: function(input) { return 4; },
    },
    i64: {
        size_of: function(input) { return 8; },
    },
    u64: {
        size_of: function(input) { return 8; },
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
